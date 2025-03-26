<script setup lang="ts">
defineProps({
    loading: Boolean
})
</script>

<template>
    <div class="flex flex-column justify-center items-center pb-4 px-4">
        <div class="flex flex-grow border-t border-blue-300"></div>
        <div v-if="loading" class="text-sm text-blue-300 px-4">Loading...</div>
        <div v-if="!loading" class="text-sm text-blue-300 px-4">Ready!</div>
        <div class="flex flex-grow border-t border-blue-300"></div>
    </div>
</template>

<style scoped>
@import "tailwindcss";
</style>