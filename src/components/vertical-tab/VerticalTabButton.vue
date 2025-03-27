<script setup lang="ts">

defineProps({
    name: { type: String, required: true },
    idx: { type: Number, required: true },
})

const isSelected = defineModel<boolean>('isSelected', { type: Boolean, default: false });

defineEmits({
    trySwitch(_: {name: string, idx: number}) {},
});

</script>

<template>
    <div class="flex flex-col items-center justify-center 
        aspect-square w-full h-full p-4 rounded-lg shadow-lg border-2 border-blue-200
        transition-colors duration-300 hover:border-blue-400
        cursor-default" 
        :class="[
            {'bg-blue-400 border-blue-400 hover:bg-blue-200': isSelected, 'bg-white hover:bg-gray-50': !isSelected},
            {'text-white hover:text-black': isSelected, 'text-black': !isSelected},
        ]"
        @click="$emit('trySwitch', {name: name, idx: idx})">
        <div>
            <slot></slot>
        </div>
    </div>
</template>

<style scoped>
@import "tailwindcss";
</style>