<script setup lang="ts">
import { ref } from 'vue';
import { config, saveConfig } from '../../config';

let loadImageCnt = ref(config.loadImageCnt);

const onSavePrefClicked = async () => {
    config.loadImageCnt = loadImageCnt.value === 0 ? config.loadImageCnt : loadImageCnt.value;
    await saveConfig();
}

</script>

<template>
    <div class="p-8 relative w-full h-full">
        <div class="flex flex-row relative flex-auto w-auto h-auto gap-y-4 gap-x-4
            transition-all duration-500 ease-in-out pb-4 flex-nowrap">
            <div class="text-2xl text-blue-900 font-mono font-bold">Preferences</div>
        </div>
        <div class="flex flex-between w-full place-content-between items-center">
            <div class="flex">
                <span class="text-base text-blue-950 font-mono"> Images to load in a row </span>
            </div>
            <div class="flex pr-12 ">
                <div class="flex justify-center items-center basis-128 
                    hover:shadow-md
                    text-gray-700 bg-white
                    border-blue-100 border-2 rounded-lg
                    hover:outline-none focus:ring
                    transition-colors duration-300
                    hover:border-blue-400">
                    <input type="number" class=" h-8 text-blue-950 font-mono font-bold p-0" v-model="loadImageCnt" />
                </div>
            </div>
        </div>
        <div class="absolute flex bottom-0 left-0 mx-10 my-16 w-32 h-8 rounded-lg
            bg-white hover:bg-blue-400
            border-blue-400 border-2
            hover:outline-none focus:ring
            hover:border-transparent
            text-blue-400 font-mono font-bold
            hover:text-white
            text-center
            items-center justify-center
            cursor-default
            transition-colors duration-300"
            @click="async () => await onSavePrefClicked()"
            >Save changes</div>
    </div>
</template>

<style scoped>
@import "tailwindcss";
</style>