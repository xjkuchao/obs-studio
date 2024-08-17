import { invoke } from '@tauri-apps/api/core';
import { createApp } from 'vue';
import { createI18n } from 'vue-i18n';

import App from './App.vue';
import { setupConsole } from './utils/log';

await setupConsole();

const localeMessages: Record<string, Record<string, string>> = await invoke('get_locale_messages');
const supportedLocales: string[] = Object.keys(localeMessages);
const defaultLocale: string = await invoke('get_default_locale');

const i18n = createI18n({
    locale: defaultLocale,
    fallbackLocale: 'en-US',
    availableLocales: supportedLocales,
    messages: localeMessages,
});

const app = createApp(App);
app.use(i18n);
app.mount('#app');
