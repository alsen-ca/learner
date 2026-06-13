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

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

h1 {
  text-align: center;
  font-size: 3rem;
}

button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

button {
  outline: none;
}

.main-button {
  height: 5rem;
  width: 15rem;
  border-color: white;
  border-width: 0.4rem;
  align-items: center;
}

.starter {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>