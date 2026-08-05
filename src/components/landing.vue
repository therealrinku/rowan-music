<script>
import { defineComponent } from "vue";
import { open } from "@tauri-apps/plugin-dialog";

export default defineComponent({
  name: "TimerTodoApp",
  data() {
    return {};
  },
  methods: {
    async selectFolder() {
      const selectedPath = await open({
        directory: true,
        multiple: false,
      });

      if (selectedPath === null) {
        //
      } else {
        this.$emit("onSelectFolder", selectedPath);
      }
    },
  },
});
</script>

<template>
  <main
    class="bg-[#F88379] text-white min-h-screen min-w-screen text-sm tracking-wide flex flex-col gap-5 items-center justify-center w-full min-h-screen"
  >
    <p>Welcome to rowan music</p>
    <button @click="selectFolder" class="bg-green-500 px-5 py-2 rounded-md">
      Select Folder
    </button>
    <p class="text-xs text-red-100">currently only mp3 is supported, select a folder that has mp3 files.</p>
  </main>
</template>
