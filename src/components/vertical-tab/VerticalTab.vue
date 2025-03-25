<script setup lang="tsx">
import { ref } from 'vue';
import VerticalTabButton from './VerticalTabButton.vue'
import { HomeIcon, Cog6ToothIcon, FolderIcon } from '@heroicons/vue/16/solid';

const tabPage = ref(0);

const tabButtons = [
    {
        name: 'Home',
        icon: HomeIcon
    },
    {
        name: 'Folders',
        icon: FolderIcon
    },
    {
        name: 'Settings',
        icon: Cog6ToothIcon
    },
]

defineEmits({trySwitch: (payload: {name: string, idx: number}) => {}, })

</script>

<template>
    <div class="flex flex-row basis-64 relative">
        <div>
            <div id="tab-buttons" class="flex flex-col basis-auto gap-8">
                <VerticalTabButton v-for="(tabButton, index) in tabButtons" 
                    :name="tabButton.name" :idx="index" :is-selected="tabPage === index"
                    @try-switch="(payload) => {
                        tabPage = payload.idx
                        console.log(payload)
                        $emit('trySwitch', payload)
                    }"
                    >
                    <div class="flex flex-row">
                        <div class="flex flex-col gap-1 justify-center items-center">
                            <component :is="tabButton.icon" class="w-6 h-6" />
                            <div class="text-sm">{{tabButton.name}}</div>
                        </div>
                    </div>
                </VerticalTabButton>
            </div>
        </div>
    </div>
</template>

<style scoped>
@import "tailwindcss"
</style>