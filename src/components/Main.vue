<script setup lang="ts">
import VerticalTab from './vertical-tab/VerticalTab.vue';
import ImageGridContainer from './image-container/ImageGridContainer.vue';
import TabPageContainer from './utils/TabPageContainer.vue';
import SearchBox from './search-box/SearchBox.vue';

import { ref, watch } from 'vue';
import FolderGridContainer from './folder-container/FolderGridContainer.vue';
import { invoke } from '@tauri-apps/api/core';
import { throttle } from 'lodash';
import Preferences from './preferences/Preferences.vue';

import { app } from '../main';

const showSearchBox = ref(false);

const readClipboard = async () => {
    let item = await navigator.clipboard.read()
    if (item.length === 0) return null;
    return Array.from(new Uint8Array(await (await item[0].getType('image/png')).arrayBuffer()))
}

const generateFileName = () => {
    return `clipboard_${Date.now()}.png`;
}

const emitCopyEvent = () => {
    app.config.globalProperties.$eventBus.emit('copy', {});
}

const throttled = throttle(async () => {
    let buf = await readClipboard();
    if (buf === null) return;
    let name = generateFileName();
    await invoke("write_file", { fileName: name, buffer: buf });
    console.log(name);
    
    emitCopyEvent();
}, 1000)

window.addEventListener('keydown', async (e) => {
    if (e.key === 'k' && e.ctrlKey) {
        showSearchBox.value = !showSearchBox.value;
        console.log('show search box');
    }

    if (e.key === 'Escape') {
        showSearchBox.value = false;
    }

    if (e.key === 'v' && e.ctrlKey) {
        throttled();
    }
})

const searchBox: any = ref(null);
watch(showSearchBox, (newVal) => {
    if (newVal) {
        searchBox.value?.focusTextBox();
    }
})

const tabPages = [
    ImageGridContainer,
    FolderGridContainer,
    Preferences,
]

const tabIndex = ref(0);

</script>

<template>
    <div :class="{ 'invisible': !showSearchBox }">
        <SearchBox ref="searchBox"></SearchBox>
    </div>
    <div class="size-full flex flex-column h-screen">
        <div class="relative p-8 w-auto h-auto bg-slate-100">
            <VerticalTab @try-switch="(payload) => {
                let idx = payload.idx
                console.log(idx)
                if (idx < 0 || idx >= tabPages.length) {
                    console.log('invalid index')
                    return;
                }
                tabIndex = idx
            }"></VerticalTab>
        </div>
        <div class="overflow-y-auto bg-white w-full">
            <TabPageContainer >
                <Suspense>
                    <component :is="tabPages[tabIndex]"></component>
                </Suspense>
            </TabPageContainer>
        </div>
    </div>
</template>

<style scoped>
@import "tailwindcss";

.visible {
    visibility: visible;
}

.invisible {
    visibility: hidden;
}

</style>