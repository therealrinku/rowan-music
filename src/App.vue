<script lang="ts">
import { defineComponent } from "vue";
import { invoke, convertFileSrc } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import PlayIcon from "./components/icons/play-icon.vue";
import PauseIcon from "./components/icons/pause-icon.vue";
import LeftIcon from "./components/icons/chevron-left-icon.vue";
import RightIcon from "./components/icons/chevron-right-icon.vue";

export default defineComponent({
  name: "TimerTodoApp",
  components: {
    PlayIcon,
    PauseIcon,
    LeftIcon,
    RightIcon,
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
    //invoke("scan_folder", {folder: "/Users/therealrinku/code/rowan-music/src" });

    const songs = await invoke("get_songs");
    this.songs = songs;
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
    playSong(song) {
      this.current = new Audio(convertFileSrc(song.path));
      console.log(this.current);
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
  <main
    class="bg-[#F88379] text-white min-h-screen min-w-screen text-sm tracking-wide flex items-center w-full min-h-screen"
  >
    <div
      class="h-[85vh] flex flex-col items-center justify-center h-screen w-[70%] p-5 gap-5 opacity-90 mx-auto"
    >
      <div
        class="flex flex-col items-center justify-center gap-5 w-[60%] max-w-[600px]"
      >
        <img
          class="w-full h-60 object-cover"
          src="https://www.rollingstone.com/wp-content/uploads/2018/06/weeknd-starboy-album-review-92e9ca7c-8701-41e0-8720-2102a52cd1dd.jpg"
        />

        <p>Weeknd - Starboy</p>

        <div class="w-full flex flex-col gap-1">
          <div class="flex items-center justify-between w-full">
            <p>{{ formattedCurrentDuration }}</p>
            <p>{{ formattedTotalDuration }}</p>
          </div>

          <div class="w-full bg-zinc-300 h-1">
            <div
              class="h-full bg-white"
              :style="{ width: currentDurPer + '%' }"
            ></div>
          </div>
        </div>

        <div class="flex items-center gap-5">
          <button>
            <LeftIcon />
          </button>

          <button @click="playPause">
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
        class="border-b-1 border-gray-900 w-full p-3"
        type="text"
        placeholder="Search..."
      />

      <div class="flex flex-col w-full max-h-[89vh] overflow-y-auto">
        <button
          v-for="song in songs"
          class="border-b w-full py-3 pl-3 flex"
          @click="playSong(song)"
        >
          ♫ {{ song.path.split("/").pop() }}
        </button>
      </div>
    </div>
  </main>
</template>
