<script setup lang="ts">
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";

const emit = defineEmits(['back-to-start']);

const letters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'.split('')
const currentIndex = ref(0)

const currentLetter = computed(() => letters[currentIndex.value])

function next() {
  currentIndex.value = (currentIndex.value + 1) % letters.length
}

function previous() {
  currentIndex.value = (currentIndex.value - 1 + letters.length) % letters.length
}

function onLetterClick() {
  invoke('play_sound', { letter: currentLetter.value })
}

</script>
<template>
    <div class="words-container">
        <div class="left-pannel">
            <img src="../assets/chevron-left.svg" alt="previous character" class="arrow-button" @click="previous"/>
            <img src="../assets/arrow-big-left.svg" alt="Go back to Main Page" class="back-button" @click="emit('back-to-start')"/>
        </div>
        <div class="middle-pannel">
            <button class="main-letter" @click="onLetterClick">{{ currentLetter }}</button>
        </div>
        <div class="right-pannel">
            <img src="../assets/chevron-right.svg" title="next character" class="arrow-button" @click="next"/>
        </div>
    </div>
</template>

<style>
.words-container {
    width: 100%;
    height: 100vh;
    display: flex;
    flex-direction: row;
}

.left-pannel {
    width: 22.5%;
    height: 100%;
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    overflow: hidden;
}
.middle-pannel {
    width: 55%;
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
}
.right-pannel {
    width: 22.5%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
}

.arrow-button {
    height: auto;
    width: 100%;
    cursor: pointer;
    box-sizing: border-box;
    border: 0.5px dashed gray;
}
.back-button {
    position: absolute;
    width: 3.5rem;
    height: auto;
    position: absolute;
    bottom: 0;
    left: 0;
    cursor: pointer;
}

.main-letter {
    margin-top: 10rem;
    text-align: center;
    font-size: clamp(4rem, 130vmin, 50rem);
}
</style>