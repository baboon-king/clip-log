<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { readText, writeText } from "@tauri-apps/api/clipboard";
import { register } from "@tauri-apps/api/globalShortcut";
import { LogicalPosition, appWindow } from "@tauri-apps/api/window";

const history = ref<string[]>([]);

appWindow.hide();

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
}, 800);

const onClick = (text: string) => {
  writeText(text);
  appWindow.hide();
};
register("Option+V", async () => {
  const [x, y] = await invoke<[number, number]>("get_mouse_pos");
  appWindow.setPosition(new LogicalPosition(x, y));
  appWindow.setFocus();
});

const Numbers = ["1", "2", "3", "4", "5", "6"];
window.addEventListener("keyup", (e) => {
  const key = e.key;
  if (Numbers.includes(key)) {
    onClick(history.value[Number(key) - 1]);
  }
});
</script>

<template>
  <div>
    <template v-for="(item, index) in history" :key="item">
      <div style="margin-bottom: 10px" @dblclick="onClick(item)">
        {{ index + 1 }}. {{ item }}
      </div>
    </template>
  </div>
</template>
