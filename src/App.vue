<script>
import { defineComponent } from "vue";
import { invoke, convertFileSrc } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import MusicIcon from "./components/icons/music-icon.vue";
import PlayIcon from "./components/icons/play-icon.vue";
import PauseIcon from "./components/icons/pause-icon.vue";
import LeftIcon from "./components/icons/chevron-left-icon.vue";
import RightIcon from "./components/icons/chevron-right-icon.vue";
import Landing from "./components/landing.vue";

export default defineComponent({
  name: "TimerTodoApp",
  components: {
    PlayIcon,
    PauseIcon,
    LeftIcon,
    RightIcon,
    Landing,
    MusicIcon,
  },
  data() {
    return {
      current: null,
      playing: false,
      currentDuration: 0,
      totalDuration: 0,
      songs: [],
      selected: null,
    };
  },
  async mounted() {
    const songs = await invoke("get_songs");
    this.songs = songs;
    if (this.songs.length) {
      this.selected = this.songs[0];
      this.current = new Audio(convertFileSrc(this.songs[0].path));
    }
  },
  unmounted() {},
  watch: {
    current() {
      if (!this.current) return;

      this.current.addEventListener("timeupdate", () => {
        this.currentDuration = this.current.currentTime;
      });

      this.current.addEventListener("loadeddata", () => {
        this.totalDuration = this.current.duration;
      });

      this.current.addEventListener("ended", () => {
        this.playing = false;
      });
    },
  },
  computed: {
    selectedCoverImagePath() {
      return convertFileSrc(this.selected.cover_image_path);
    },
    formattedTotalDuration() {
      if (!this.totalDuration) return "00:00";

      const mins = Math.floor(this.totalDuration / 60);
      const secs = Math.floor(this.totalDuration % 60);

      const formattedMins = String(mins).padStart(2, "0");
      const formattedSecs = String(secs).padStart(2, "0");
      return `${formattedMins}:${formattedSecs}`;
    },
    formattedCurrentDuration() {
      if (!this.currentDuration) return "00:00";

      const mins = Math.floor(this.currentDuration / 60);
      const secs = Math.floor(this.currentDuration % 60);

      const formattedMins = String(mins).padStart(2, "0");
      const formattedSecs = String(secs).padStart(2, "0");
      return `${formattedMins}:${formattedSecs}`;
    },
    currentDurPer() {
      return Math.floor((this.currentDuration / this.totalDuration) * 100);
    },
  },
  methods: {
    onSelectFolder(folder) {
      this.selectedFolder = folder;
      this.scan(this.selectedFolder);
    },
    async scan(folder) {
      const songs = await invoke("scan_folder", { folder });
      this.songs = songs;
      if (this.songs.length) {
        this.selected = this.songs[0];
        this.current = new Audio(convertFileSrc(this.songs[0].path));
      }
    },
    playSong(song) {
      if (this.current && !this.current.paused) {
        this.current.pause();
        this.playing = false;
      }
      this.current = new Audio(convertFileSrc(song.path));
      this.selected = song;
    },
    playPause() {
      if (!this.current) return;

      if (this.current.paused) {
        this.current.play();
        this.playing = true;
      } else {
        this.current.pause();
        this.playing = false;
      }
    },
  },
});
</script>

<template>
  <landing @onSelectFolder="onSelectFolder" v-if="!songs.length" />
  <main
    v-else
    class="bg-[#F88379] text-white min-h-screen min-w-screen text-sm tracking-wide flex items-center w-full min-h-screen"
  >
    <div
      class="h-[85vh] flex flex-col items-center justify-center h-screen w-[70%] p-5 gap-5 opacity-90 mx-auto"
    >
      <div
        class="flex flex-col items-center justify-center gap-5 w-[60%] max-w-[600px]"
      >
        <div
          class="h-60 flex flex-col items-center justify-center border w-full rounded-lg"
        >
          <img
            v-if="selected && selected.cover_image_path"
            :src="selectedCoverImagePath"
            class="w-full h-full object-cover"
          />
          <MusicIcon v-else />
        </div>

        <p v-if="selected && selected.title && selected.artist">
          {{ selected.artist }} - {{ selected.title }}
        </p>
        <p v-else-if="selected">
          {{ selected.path.split("/").pop().split(".").slice(0, -1).join("") }}
        </p>
        <p v-else>select a music to get started</p>

        <div class="w-full flex flex-col gap-1">
          <div class="flex items-center justify-between w-full">
            <p>{{ formattedCurrentDuration }}</p>
            <p>{{ formattedTotalDuration }}</p>
          </div>

          <div class="w-full bg-white h-1">
            <div
              class="h-full bg-green-300"
              :style="{ width: currentDurPer + '%' }"
            ></div>
          </div>
        </div>

        <div class="flex items-center gap-5">
          <button>
            <LeftIcon />
          </button>

          <button @click="playPause" :disabled="!selected">
            <PauseIcon v-if="playing" />
            <PlayIcon v-else />
          </button>

          <button click="nextDay">
            <RightIcon />
          </button>
        </div>
      </div>
    </div>

    <div
      class="bg-[#af4949] opacity-90 flex flex-col items-center w-[30%] h-screen ml-auto"
    >
      <input
        class="border-b-1 border-[#F88379] w-full p-3 outline-none"
        type="text"
        placeholder="Search..."
      />

      <div class="flex flex-col w-full max-h-[89vh] overflow-y-auto">
        <button
          v-for="song in songs"
          class="border-b border-[#F88379] w-full py-3 pl-3 flex cursor-pointer"
          @click="playSong(song)"
        >
          ♫
          {{
            song.title ||
            song.path.split("/").pop().split(".").slice(0, -1).join("")
          }}
        </button>
      </div>
    </div>
  </main>
</template>
