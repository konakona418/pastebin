<script setup lang="ts">
import { nextTick, reactive, ref } from "vue";
import ImageGrid from "../image-container/ImageGrid.vue";
import { invoke } from "@tauri-apps/api/core";
import GridContainer from "../utils/GridContainer.vue";
import LoadingNotice from "../utils/LoadingNotice.vue";

import { ParsedImage } from "../image-container/parsed-image.ts"

import { getConfig } from "../../config.ts"

const imageList = reactive<Array<ParsedImage>>([]);
let pageSize = (await getConfig()).loadImageCnt;
let page = 0;

const loading = ref(false)

const props = defineProps({
    directory: String,
})

const clearImages = () => {
    imageList.splice(0, imageList.length);
    page = 0;
}

const forceLoad = async () => {
    await nextTick();
    await loadMore();
}

defineExpose({
    clearImages,
    forceLoad
})

const loadMore = async () => {
    if (!props.directory) return;
    const fileList: string[] = await invoke("get_directory_files_page", { dir: props.directory, page: page++, pageSize: pageSize });
    loading.value = true;
    let parsed = await Promise.all(await ParsedImage.fromDirFileList(props.directory, fileList));
    imageList.push(...parsed);
    loading.value = false;
}

</script>

<template>
    <div class="flex flex-auto bg-blue-300 text-white font-bold text-xl font-mono
    justify-center item-center place-self-center h-8 w-auto rounded-lg" v-if="imageList.length === 0">
        <div class="flex flex-col justify-center items-center px-4">
            no existing images
        </div>
    </div>
    <div class="flex flex-col flex-auto text-blue-900 font-bold text-xl font-mono
            justify-center item-center place-self-center h-4 w-auto rounded-lg my-4" >
        <div class="flex justify-center items-center">
            {{ directory }}
        </div>
    </div>
    <GridContainer 
        @scroll-end="async () => {
            console.log('scroll end')
            await loadMore()
        }"
        @scroll-start="async () => {
            console.log('scroll start')
            await loadMore()
        }">
        <ImageGrid v-for="imageUrl in imageList" 
            :image-url="imageUrl.blobUrl" 
            :parsed-image="imageUrl"/>
    </GridContainer>
    <LoadingNotice :loading="loading"/>
</template>

<style scoped>
@import "tailwindcss";
</style>