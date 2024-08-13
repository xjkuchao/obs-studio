import { invoke } from '@tauri-apps/api/core';
import i18n from 'i18next';
import React from 'react';
import ReactDOM from 'react-dom/client';
import { initReactI18next } from 'react-i18next';

import App from './App';
import { setupConsole } from './utils/log';

await setupConsole();

const localeMessages: Record<string, Record<string, Record<string, string>>> = await invoke(
    'get_locale_messages',
);
const supportedLocales: string[] = Object.keys(localeMessages);
let defaultLocale: string = await invoke('get_default_locale');
if (defaultLocale === 'zh-Hans-CN') {
    defaultLocale = 'zh-CN';
}

// console.log('localeMessages:', localeMessages);
// console.log('supportedLocales:', supportedLocales);
// console.log('defaultLocale:', defaultLocale);

i18n.use(initReactI18next).init({
    lng: defaultLocale,
    fallbackLng: 'en-US',
    resources: localeMessages,
    supportedLngs: supportedLocales,
});

ReactDOM.createRoot(document.getElementById('root')!).render(
    <React.StrictMode>
        <App />
    </React.StrictMode>,
);
