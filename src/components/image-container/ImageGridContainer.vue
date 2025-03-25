<script setup lang="ts">
import { onMounted, ref } from "vue";
import ImageGrid from "./ImageGrid.vue";
import { invoke } from "@tauri-apps/api/core";

const imageList = ref<string[]>(await invoke("get_file_list"));

function parseImageType(filePath: string): string {
    const fileExtension = filePath.split(".").pop();
    switch (fileExtension) {
        case "jpg":
        case "jpeg":
            return "image/jpeg";
        case "png":
            return "image/png";
        case "gif":
            return "image/gif";
        default:
            throw new Error(`Unsupported file type: ${fileExtension}`);
    }
}

async function copyToClipboard(imageUrl: string) {
    const buf = await fetch(imageUrl).then(response => response.blob());
    const clipboardItem = new ClipboardItem(
        { "image/png" : buf }
    );
    await navigator.clipboard.write([clipboardItem]);
}

onMounted(async () => {
    let filePaths: string[] = await invoke("get_file_list");
    const imageUrls = await Promise.all(filePaths.map(async (filePath) => {
        const buf: number[] = await invoke("read_file", { fileName: filePath });
        const imageUrl = URL.createObjectURL(new Blob([new Uint8Array(buf)], { type: parseImageType(filePath) }));
        return imageUrl;
    }));
    imageList.value = imageUrls;
    // console.log(imageList.value);
    
});

</script>

<template>
    <div class="flex flex-row flex-auto flex-wrap w-full h-auto gap-y-8 gap-x-8
        transition-all duration-500 ease-in-out">
        <ImageGrid v-for="imageUrl in imageList">
            <img 
                :src="imageUrl" 
                class="min-w-24 justfy-center item-center w-full h-full p-4"
                @click="copyToClipboard(imageUrl)"
            />
        </ImageGrid>
    </div>
</template>

<style scoped>
@import "tailwindcss";
</style>