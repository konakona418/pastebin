<script setup lang="ts"></script>

<template>
    <div class="basis-32 aspect-square justify-center item-center 
        bg-gray-100 rounded-lg border-2 border-blue-200
        hover:bg-gray-200 hover:border-blue-400 shadow-lg
        transition-all duration-300 ease-in-out">
        <slot></slot>
    </div>
</template>

<style scoped>
@import "tailwindcss";
</style>