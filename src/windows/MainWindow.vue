<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";




async function openQueryWindow() {
  await invoke("open_query_window");
}

async function openSettingsWindow() {
  await invoke("open_settings_window");
}
</script>

<template>
  
</template>
