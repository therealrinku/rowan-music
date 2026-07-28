use rusqlite::{Connection, Result};
use walkdir::WalkDir;
use std::path::Path;

use lofty::file::AudioFile;
use lofty::file::TaggedFileExt;
use lofty::read_from_path;
use lofty::tag::ItemKey;

use directories::ProjectDirs;
use sha2::{Sha256, Digest};

use serde::{Serialize, Deserialize};

struct Database {
    conn: Connection,
}

#[derive(Serialize, Deserialize)]
pub struct Music {
    id: i32,
    title: String,
    artist: String,
    genre: String,
    album: String,
    year: String,
    cover_image_path: String,
    search_text: String,
    duration: i32,
    path: String
}

impl Database {
    pub fn new() -> rusqlite::Result<Self> {
        let proj_dirs = ProjectDirs::from("com", "rowan", "music").unwrap();
        let db_path = proj_dirs.data_dir().join("music.db");

        // FIXED: Map std::io::Error into a rusqlite::Error using SqliteFailure or an alternative variant, 
        // or convert it to a custom/other rusqlite error type.
        std::fs::create_dir_all(proj_dirs.data_dir()).map_err(|e| {
            rusqlite::Error::ToSqlConversionFailure(Box::new(e))
        })?;

        Ok(Self {
            conn: Connection::open(db_path)?,
        })
    }
    
    fn setup_table(&self) -> Result<()> {
        self.conn.execute(
          "CREATE TABLE IF NOT EXISTS music (
              id INTEGER PRIMARY KEY,
              title TEXT,
              genre TEXT,
              album TEXT,
              year TEXT,
              duration INTEGER,
              path TEXT,
              cover_image_path TEXT,
              search_text TEXT,
              artist TEXT
          )",
          [],
       )?;

       Ok(())
    }

    fn add_song(&self, music: &Music) -> Result<()> {
      self.conn.execute(
          "INSERT INTO music(title, genre, album, year, duration, path, cover_image_path, search_text, artist) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
          (&music.title, &music.genre, &music.album, &music.year, &music.duration, &music.path, &music.cover_image_path, &music.search_text, &music.artist),
       )?;

       Ok(())
    }

    fn clean_previous(&self) -> Result<()> {
      self.conn.execute("DELETE FROM music", []);
      Ok(())
    }

    fn get_songs(&self) -> Result<rusqlite::Statement> {
      let stmt = self.conn.prepare("SELECT * FROM music WHERE search_text LIKE ?1 LIMIT 100 OFFSET ?2")?;
      // Note: If executing immediately, parameters are bound during query execution. 
      // Returning the prepared statement directly so caller can bind and query.
      Ok(stmt)
    }
}

pub fn get_songs() -> rusqlite::Result<Vec<Music>> {
    let db = Database::new()?;
    db.setup_table();

    let mut stmt = db.conn.prepare("SELECT id, title, artist, genre, album, year, cover_image_path, search_text, duration, path FROM music")?;
    
    let songs = stmt.query_map([], |row| {
        Ok(Music {
            id: row.get(0)?,
            title: row.get(1)?,
            artist: row.get(2)?,
            genre: row.get(3)?,
            album: row.get(4)?,
            year: row.get(5)?,
            cover_image_path: row.get(6)?,
            search_text: row.get(7)?,
            duration: row.get(8)?,
            path: row.get(9)?,
        })
    })?;

    let mut result = Vec::new();
    for song in songs {
        result.push(song?);
    }

    Ok(result)
}

pub fn scan(folder: &str) -> rusqlite::Result<Vec<Music>> {
    let db = Database::new()?;
    db.setup_table();

    // clean exisiting data
    db.clean_previous();   

    for entry in WalkDir::new(folder)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension() {
                // FIXME: mp3 only for now
                if ext == "mp3" {
                    let music_data = extract_metadata(path);
                    db.add_song(&music_data)?;
                }
            }
        }
    }

    let songs = get_songs();
    Ok(songs?)
}

fn extract_metadata(path: &Path) -> Music {
    let tagged_file = read_from_path(path).unwrap();

    let dur = tagged_file.properties().duration();
    let mut hasher = Sha256::new();
    hasher.update(path.to_string_lossy().as_bytes());
    let hash = format!("{:x}", hasher.finalize());
    let cover_filename = format!("covers/{}.jpg", hash);

    // Initialize defaults
    let mut title = String::new();
    let mut artist = String::new();
    let mut genre = String::new();
    let mut album = String::new();
    let mut year = String::new();
    let mut saved_cover_path = String::new();

    if let Some(tag) = tagged_file.primary_tag() {
        for picture in tag.pictures() {
            let image_data = picture.data();
            // save cover image to disk
            if std::fs::write(&cover_filename, image_data).is_ok() {
                saved_cover_path = cover_filename.clone();
            }
            break; // Stop after saving the first picture
        }

        title = tag.get_string(ItemKey::TrackTitle).unwrap_or_default().to_string();
        artist = tag.get_string(ItemKey::TrackArtist).unwrap_or_default().to_string();
        genre = tag.get_string(ItemKey::Genre).unwrap_or_default().to_string();
        album = tag.get_string(ItemKey::AlbumTitle).unwrap_or_default().to_string();
        year = tag.get_string(ItemKey::Year).unwrap_or_default().to_string();
    }

    let search_content = format!("{} {} {}", title, artist, album).to_lowercase();

    Music {
        id: 0,
        title,
        artist,
        genre,
        album,
        year,
        cover_image_path: saved_cover_path,
        search_text: search_content,
        duration: dur.as_secs() as i32,
        path: path.to_string_lossy().into_owned(),
    }
}
