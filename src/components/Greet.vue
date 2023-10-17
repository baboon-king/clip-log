<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { readText, writeText } from "@tauri-apps/api/clipboard";
import { register } from "@tauri-apps/api/globalShortcut";
import { LogicalPosition, appWindow } from "@tauri-apps/api/window";

const greetMsg = ref("");
const name = ref("");

const history = ref<string[]>([]);

setInterval(async () => {
  const text = await readText();
  if (!text) {
    return;
  }
  if (history.value.at(0) === text) {
    return;
  }

  const index = history.value.findIndex((item) => item === text);
  if (index !== -1) {
    history.value.splice(index, 1);
  }

  history.value.unshift(text);
}, 100);

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
}

const onClick = (text: string) => {
  writeText(text);
};
register("Option+V", async () => {
  console.log("Shortcut triggered");
  writeText("Shortcut triggered");
  const [x, y] = await invoke<[number, number]>("get_mouse_pos");
  appWindow.setPosition(new LogicalPosition(x, y));
  appWindow.setFocus();
});
</script>

<template>
  <!-- <form class="row" @submit.prevent="greet">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="submit">Greet</button>
  </form> -->

  <p>{{ greetMsg }}</p>

  <div>
    <template v-for="(item, index) in history">
      <div style="margin-bottom: 10px" @dblclick="onClick(item)">
        {{ index + 1 }}. {{ item }}
      </div>
    </template>
  </div>
</template>
