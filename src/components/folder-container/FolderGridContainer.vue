<script setup lang="ts">
import { reactive, Ref, ref } from "vue";
import FolderGrid from "./FolderGrid.vue";
import { invoke } from "@tauri-apps/api/core";
import GridContainer from "../utils/GridContainer.vue";
// import LoadingNotice from "../utils/LoadingNotice.vue";

import { ParsedFolder } from "./parsed-folder.ts";
import ImagePreviewer from "./ImagePreviewer.vue";
import { config } from "../../config.ts";

const folderList = reactive<Array<ParsedFolder>>([]);
let pageSize = config.loadImageCnt;
let page = 0;

const loading = ref(false)

const loadMore = async () => {
    const folders: string[] = await invoke("get_directory_list_page", { page: page++, pageSize: pageSize });
    loading.value = true;
    let parsed = await Promise.all(await ParsedFolder.fromFolderList(folders));
    folderList.push(...parsed);
    loading.value = false;
}

const openFolderHandler = async (folder: string) => {
    imagePreviewer.value?.clearImages();
    selectedFolder.value = folder;
    shouldDisplayImageViewer.value = true;
    imagePreviewer.value?.forceLoad();
}

const selectedFolder = ref("");
const shouldDisplayImageViewer = ref(false);

const imagePreviewer: Ref<InstanceType<typeof ImagePreviewer> | null> = ref(null);

</script>

<template>
    <div class="flex flex-auto bg-blue-300 text-white font-bold text-xl font-mono
    justify-center item-center place-self-center h-8 w-auto rounded-lg" v-if="folderList.length === 0">
        <div class="flex flex-col justify-center items-center px-4">
            no existing folders
        </div>
    </div>
    <div class="flex flex-col" :class="{'min-h-screen max-h-screen': shouldDisplayImageViewer}">
        <GridContainer class="flex bg-white"
            :should-wrap="!shouldDisplayImageViewer"
            :should-scroll-v="false"
            :should-scroll-h="true"
            :class="{ 'min-h-54 max-h-54': shouldDisplayImageViewer }"
            @scroll-end="async () => {
                console.log('scroll end')
                await loadMore()
            }" @scroll-start="async () => {
            console.log('scroll start')
            await loadMore()
        }">
            <FolderGrid v-for="folder in folderList" :parsed-folder="folder" @open-folder="openFolderHandler" />
        </GridContainer>
        <div v-if="shouldDisplayImageViewer" class="relative overflow-y-auto scrollbar-hidden mb-4">
            <ImagePreviewer class="flex flex-auto overflow-y-auto" ref="imagePreviewer" v-if="shouldDisplayImageViewer"
                :directory="selectedFolder" />
        </div>
    </div>
</template>

<style scoped>
@import "tailwindcss";
</style>