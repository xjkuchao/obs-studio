<script setup lang="ts"></script>

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
                <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
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

:root {
    background-color: #f6f6f6;

    color: #0f0f0f;
    font-weight: 400;
    font-size: 16px;
    line-height: 24px;
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
}

.container {
    display: flex;
    flex-direction: column;
    justify-content: center;
    margin: 0;
    padding-top: 10vh;
    text-align: center;
}

.logo {
    transition: 0.75s;
    will-change: filter;
    padding: 1.5em;
    height: 6em;
}

.logo.tauri:hover {
    filter: drop-shadow(0 0 2em #24c8db);
}

.row {
    display: flex;
    justify-content: center;
}

a {
    color: #646cff;
    font-weight: 500;
    text-decoration: inherit;
}

a:hover {
    color: #535bf2;
}

h1 {
    text-align: center;
}

input,
button {
    transition: border-color 0.25s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
    border: 1px solid transparent;
    border-radius: 8px;
    background-color: #ffffff;
    padding: 0.6em 1.2em;
    color: #0f0f0f;
    font-weight: 500;
    font-size: 1em;
    font-family: inherit;
}

button {
    cursor: pointer;
}

button:hover {
    border-color: #396cd8;
}
button:active {
    border-color: #396cd8;
    background-color: #e8e8e8;
}

input,
button {
    outline: none;
}

#greet-input {
    margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
    :root {
        background-color: #2f2f2f;
        color: #f6f6f6;
    }

    a:hover {
        color: #24c8db;
    }

    input,
    button {
        background-color: #0f0f0f98;
        color: #ffffff;
    }
    button:active {
        background-color: #0f0f0f69;
    }
}
</style>
