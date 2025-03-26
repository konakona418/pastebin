<script setup lang="ts">
import VerticalTab from './vertical-tab/VerticalTab.vue';
import ImageGridContainer from './image-container/ImageGridContainer.vue';
import TabPageContainer from './utils/TabPageContainer.vue';
import SearchBox from './search-box/SearchBox.vue';

import { ref, watch } from 'vue';
import FolderGridContainer from './folder-container/FolderGridContainer.vue';

const showSearchBox = ref(false);

window.addEventListener('keydown', (e) => {
    if (e.key === 'k' && e.ctrlKey) {
        showSearchBox.value = !showSearchBox.value;
    }

    if (e.key === 'Escape') {
        showSearchBox.value = false;
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