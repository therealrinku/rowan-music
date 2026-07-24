use rusqlite::{Connection, Result};

use walkdir::WalkDir;
use std::path::Path;

use lofty::{
    AudioFile,
    TaggedFileExt,
    read_from_path,
};

struct Music {
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

fn connect() {
    // FIXME: use in storage db
    let conn = Connection::open_in_memory()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS music (
            title TEXT,
            genre TEXT,
            album TEXT
            year TEXT,
            duration INTEGER,
            path TEXT,
            cover_image_path TEXT,
            search_text TEXT,
            artist TEXT
        )",
    )?;
}

fn scan(folder: &str) {
    for entry in WalkDir::new(folder)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension() {
                // FIXME: mp3 only for now
                if ext == "mp3" {
                    extract_metadata(path);
                }
            }
        }
    }
}

fn extract_metadata(path: &str) {
    let tagged_file = read_from_path(path).unwrap();

    let dur = tagged_file.properties().duration()

    if let Some(tag) = tagged_file.primary_tag() {
        println!("Title: {:?}", tag.title());
        println!("Artist: {:?}", tag.artist());
        println!("Album: {:?}", tag.album());
        println!("Genre: {:?}", tag.genre());

        for picture in tag.pictures() {
          let image_data = picture.data();

          // save cover image to disk
          let mut hasher = Sha256::new();
          hasher.update(mp3_path.as_bytes());
          let hash = format!("{:x}", hasher.finalize());
          let filename = format!("covers/{}.jpg", hash);

          std::fs::write(filename, image_data).unwrap();
        }

        // save to db
        // saveToDb(tag.title(), tag.artist(), tag.album(), tab.genre(), tag.year(), filename)
    }
}

fn saveToDb(music: Music) {
    conn.execute(
        "INSERT INTO music(title, genre, album, year, duration, path, cover_image_path, search_text, artist) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        (&music.title, &music.genre, &music.album, &music.year, &music.duration, &music.path, &music.cover_image_path, &music.search_text, &music.artist),
    )?;
}

fn loadFromDb(search: &str, page = 1)->Music[] {
    let stmt = conn.prepare("SELECT * FROM music WHERE search_text LIKE %?1% LIMIT 100 OFFSET ?2", search, page)?;
    return stmt;
}
