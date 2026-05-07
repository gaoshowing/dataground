<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  greetMsg.value = await invoke("greet", { name: name.value });
}

async function openQueryWindow() {
  await invoke("open_query_window");
}

async function openSettingsWindow() {
  await invoke("open_settings_window");
}
</script>

<template>
  <main class="window-shell">

    <section class="panel">
      <h2>Connection Workspace</h2>
      <p>主入口用于承载连接管理、对象树和默认 SQL 工作区。</p>

      <div class="toolbar">
        <button type="button" @click="openQueryWindow">Open Query</button>
        <button type="button" class="secondary" @click="openSettingsWindow">
          Settings
        </button>
      </div>

      <form class="inline-form" @submit.prevent="greet">
        <input id="greet-input" v-model="name" placeholder="Enter a name..." />
        <button type="submit">Greet</button>
      </form>
      <p class="message">{{ greetMsg }}</p>
    </section>
  </main>
</template>
