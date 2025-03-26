<script setup lang="ts">
import GridItem from "../utils/GridItem.vue";

import { ParsedFolder } from "./parsed-folder";
import { FolderOpenIcon } from "@heroicons/vue/24/outline";

defineProps({
    parsedFolder: { type: ParsedFolder }
});

defineEmits({
    openFolder(_: string) {},
})

</script>

<template>
    <GridItem @click="$emit('openFolder', parsedFolder!.name)">
        <div class="relative flex flex-auto w-full h-full aspect-square">
            <FolderOpenIcon class="absolute flex w-4 h-4 m-2 right-0 aspect-square stroke-blue-500" />
            <div class="flex flex-col gap-1 justify-center items-center w-full h-full aspect-square p-2">
                <img :src="parsedFolder?.thumbNail" class="flex min-w-24 place-self-center w-full h-auto p-2 pb-2"
                    @click="" />
                <div class="absolute text-center text-sm max-w-24 bottom-0 font-mono font-bold text-blue-900 text-nowrap truncate">
                    {{ parsedFolder?.name }}
                </div>
            </div>
        </div>
    </GridItem>
</template>

<style scoped>
@import "tailwindcss";
</style>