<script setup lang="ts">
import { nextTick, Ref, ref } from 'vue'

const textBoxFocus = ref(false)

const textBox: Ref<HTMLInputElement | null> = ref(null)

async function focusTextBox() {
    await nextTick()
    textBox.value?.focus()
    console.log(textBox.value)
}

defineExpose({ focusTextBox })

</script>

<template>
    <div class="absolute w-full h-full z-98 bg-white/50 backdrop-blur-sm"></div>
    <div class="absolute top-12 flex w-full justify-center items-center z-99">
        <div class="flex justify-center items-center basis-128 shadow-md
            text-gray-700 bg-white
            border-blue-100 border-2 rounded-lg
            hover:outline-none focus:ring
            transition-colors duration-300"
            :class="{ 'border-blue-400': textBoxFocus }">
            <input
                type="text"
                class="w-full px-4 py-2"
                placeholder="Search"
                aria-label="Search"
                ref="textBox"
                @focusin="textBoxFocus = true"
                @focusout="textBoxFocus = false"
            />
        </div>
    </div>
</template>

<style scoped>
@import "tailwindcss"
</style>