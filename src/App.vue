<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window';
import { onMounted } from 'vue';

onMounted(() => {
    getCurrentWindow().show();
});
</script>

<template>
    <div class="container">
        <h1>{{ $t('About.Info') }}</h1>

        <div class="row">
            <a href="https://vitejs.dev" target="_blank">
                <img src="/vite.svg" class="logo vite" alt="Vite logo" />
            </a>
            <a href="https://tauri.app" target="_blank">
                <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
            </a>
            <a href="https://vuejs.org/" target="_blank">
                <img src="./assets/images/vue.svg" class="logo vue" alt="Vue logo" />
            </a>
        </div>
    </div>
</template>

<style scoped>
.logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
    filter: drop-shadow(0 0 2em #249b73);
}
</style>
