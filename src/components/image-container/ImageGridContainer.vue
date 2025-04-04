<script setup lang="ts">
import { onMounted, reactive, ref } from "vue";
import ImageGrid from "./ImageGrid.vue";
import { invoke } from "@tauri-apps/api/core";
import GridContainer from "../utils/GridContainer.vue";
import LoadingNotice from "../utils/LoadingNotice.vue";

import { ParsedImage } from "./parsed-image.ts"
import { getConfig } from "../../config.ts";

import { app } from "../../main.ts";
import { throttle } from "lodash";

const imageList = reactive<Array<ParsedImage>>([]);
let pageSize = (await getConfig()).loadImageCnt;
let page = 0;

const loading = ref(false)

const loadMore = async () => {
    console.log(pageSize)
    const fileList: string[] = await invoke("get_file_list_page", { page: page++, pageSize: pageSize });
    loading.value = true;
    let parsed = await Promise.all(await ParsedImage.fromFileList(fileList));
    imageList.push(...parsed);
    loading.value = false;
}

onMounted(() => {
    app.config.globalProperties.$eventBus.on('copy', throttle(() => {
        console.log('copy event')
        imageList.splice(0, imageList.length)
        page = 0;
        loadMore();
    }, 500))
})

</script>

<template>
    <div class="flex flex-auto bg-blue-300 text-white font-bold text-xl font-mono
    justify-center item-center place-self-center h-8 w-auto rounded-lg" v-if="imageList.length === 0">
        <div class="flex flex-col justify-center items-center px-4">
            no existing images
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