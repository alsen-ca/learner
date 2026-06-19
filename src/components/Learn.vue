<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
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

const container = ref<HTMLDivElement | null>(null)

onMounted(() => {
  	container.value?.focus()
})

function handleKeydown(e: { key: any; preventDefault: () => void; }) {
	switch(e.key) {
		case 'ArrowLeft':
			e.preventDefault()
			previous()
			break
		case 'ArrowRight':
			e.preventDefault()
			next()
			break
		case 'Delete':
		case 'Del':
		case 'Backspace':
			e.preventDefault()
			emit('back-to-start')
			break
		case ' ':
			e.preventDefault()
			onLetterClick()
			break
	}
}
</script>
<template>
    <div class="words-container" @keydown="handleKeydown" tabindex="-1" ref="container">
        <div class="left-pannel">
            <button class="arrow-button" @click="previous" aria-label="Previous letter" tabindex="1">
                <img src="../assets/chevron-left.svg" alt="" />
            </button>
            <button class="back-button" @click="emit('back-to-start')" aria-label="Go back to main page" tabindex="4">
                <img src="../assets/arrow-big-left.svg" alt="" />
            </button>
        </div>
        <div class="middle-pannel">
            <button 
                class="main-letter" 
                @click="onLetterClick"
                aria-live="polite"
				tabindex="2">
                {{ currentLetter }}
            </button>
        </div>
        <div class="right-pannel">
            <button class="arrow-button" @click="next" aria-label="Next letter" tabindex="3">
                <img src="../assets/chevron-right.svg" alt="" />
            </button>
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
	background: none;
	padding: 0;
	display: flex;
	align-items: center;
	justify-content: center;
}
button:focus-visible {
	outline: 3px solid gray;
	outline-offset: 4px;
	border-radius: 4px;
}

.arrow-button img,
.back-button img {
	width: 100%;
	height: auto;
	display: block;
}

.back-button {
	position: absolute;
	bottom: 0;
	left: 0;
	width: 3.5rem;
	height: auto;
	cursor: pointer;
	background: none;
	border: none;
	padding: 0;
}

.main-letter {
    margin-top: 10rem;
    text-align: center;
    font-size: clamp(4rem, 130vmin, 50rem);
}
</style>