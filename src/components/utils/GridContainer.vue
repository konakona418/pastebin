<script setup lang="ts">
import { onMounted, onUnmounted, Ref, ref } from 'vue';
import throttle from 'lodash/throttle';


const intersectionObserver: Ref<IntersectionObserver | null> = ref(null);

const emit = defineEmits({
    scrollEnd(_: any) {},
    scrollStart(_: any) {}
})

const endObserver = ref<HTMLDivElement | null>(null);

onMounted(() => {
    intersectionObserver.value = new IntersectionObserver((entries) => {
        entries.forEach((entry) => {
            if (entry && entry.isIntersecting) {
                emit("scrollEnd", true);
            }
        });
    }, { threshold: 1.0,  root: null });
    intersectionObserver.value?.observe(endObserver.value!);
});

onUnmounted(() => {
    intersectionObserver.value?.disconnect();
})

const gridContainer = ref<HTMLDivElement | null>(null);

const isEndObserverInvisible = () => {
    let boundingRect = endObserver.value?.getBoundingClientRect();
    if (boundingRect) {
        console.log(boundingRect.bottom, window.innerHeight);
        return boundingRect.bottom - 20.0 > window.innerHeight;
    }
    return false;
}

document.addEventListener("wheel", throttle((e) => {
    if (isEndObserverInvisible()) return;
    let boundingRect = gridContainer.value?.getBoundingClientRect();
    if (boundingRect && 
        e.pageX < boundingRect.x && 
            e.pageX > boundingRect.x + boundingRect.width &&
            e.pageY < boundingRect.y && 
            e.pageY > boundingRect.y + boundingRect.height ) return;
    if (e.deltaY > 0) {
        emit("scrollStart", true);
    }
}, 400, { trailing: false, leading: true }));

defineProps({
    shouldWrap: { type: Boolean, default:true },
    shouldScrollH: { type: Boolean, default:true },
    shouldScrollV: { type: Boolean, default:true },
})

</script>

<template>
    <div>
        <div class="flex flex-row flex-auto w-auto h-full gap-y-4 gap-x-4
            transition-all duration-500 ease-in-out p-8" 
            :class="[
                {'overflow-x-auto': shouldScrollH, 'overflow-x-hidden': !shouldScrollH},
                {'overflow-y-auto': shouldScrollV, 'overflow-y-hidden': !shouldScrollV},
                {'flex-wrap': shouldWrap, 'flex-nowrap': !shouldWrap}
            ]"
            ref="gridContainer">
            <slot></slot>
            <div class="flex w-full h-8" ref="endObserver"></div>
        </div>
    </div>
</template>

<style scoped>
@import "tailwindcss";
</style>