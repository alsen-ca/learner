<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import Learn from "./components/Learn.vue";

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

const currentView = ref('start'); // Defines principal viewable window at 'app-main' 

function startMain() {
  console.debug("Starting Main application")
  currentView.value = 'learn'
}
</script>

<template>
  <main class="container">
    <div v-if="currentView === 'start'">
      <h1>Yuno Learner</h1>

      <div class="starter" @click="startMain">
        <button class="main-button">Start</button>
      </div>
    </div>

    <Learn
      v-if="currentView === 'learn'"
      @back-to-start="currentView = 'start'"
    />

  </main>
</template>
