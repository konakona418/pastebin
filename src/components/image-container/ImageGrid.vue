<script setup lang="ts">
import GridItem from "../utils/GridItem.vue";

import { ParsedImage } from "./parsed-image";

const props = defineProps({
    imageUrl: String,
    parsedImage: { type: ParsedImage }
});

const parsedImage = props.parsedImage;
async function copyToClipboard(imageUrl: string | undefined) {
    if (!imageUrl) return;
    const buf = await fetch(imageUrl).then(response => response.blob());
    if (!parsedImage) return;
    const mime: string = parsedImage.mimeType;
    const clipboardItem = new ClipboardItem(
        { [mime]: buf }
    );
    await navigator.clipboard.write([clipboardItem]);
}

</script>

<template>
    <GridItem>
        <div class="flex justfy-center item-center w-full h-full aspect-square">
            <img 
                :src="imageUrl" 
                class="min-w-24 place-self-center object-contain w-full h-full p-4"
                @click="copyToClipboard(imageUrl)"
        />
        </div>
    </GridItem>
</template>

<style scoped>
@import "tailwindcss";
</style>