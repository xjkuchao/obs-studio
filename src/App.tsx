import { getCurrentWindow } from '@tauri-apps/api/window';
import { useEffect } from 'react';
import { useTranslation } from 'react-i18next';

import './App.css';
import reactLogo from './assets/react.svg';

function App() {
    const { t } = useTranslation();

    useEffect(() => {
        getCurrentWindow().show();
    }, []);

    return (
        <div className="container">
            <h1>{t('About.Info')}</h1>

            <div className="row">
                <a href="https://vitejs.dev" target="_blank" rel="noreferrer">
                    <img src="/vite.svg" className="logo vite" alt="Vite logo" />
                </a>
                <a href="https://tauri.app" target="_blank" rel="noreferrer">
                    <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
                </a>
                <a href="https://reactjs.org" target="_blank" rel="noreferrer">
                    <img src={reactLogo} className="logo react" alt="React logo" />
                </a>
            </div>
        </div>
    );
}

export default App;
