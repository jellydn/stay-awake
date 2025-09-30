<template>
  <div id="app">
    <!-- This app runs in system tray only - no main window -->
    <div class="hidden-message">
      This is a system tray application. 
      Look for the icon in your menu bar.
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'

onMounted(() => {
  // Initialize tray functionality when app starts
  console.log('macOS Sleep Prevention app initialized')
})
</script>

<style>
#app {
  display: none; /* No main window - tray only */
}

.hidden-message {
  padding: 20px;
  text-align: center;
  color: #666;
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
}
</style>