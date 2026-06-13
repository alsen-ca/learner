<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
//import { chevronLeft } from "../assets/chevron-left.svg"
//import { chevronRight } from "../assets/chevron-left.svg"

const emit = defineEmits(['back-to-start']);

const letter = ref("A")

</script>
<template>
    <p>Hello!</p>
    <button @click="emit('back-to-start')">Click here to emit</button>
</template>