import { attachConsole, debug, error, info, trace, warn } from '@tauri-apps/plugin-log';

function forwardConsole(
    fnName: 'log' | 'debug' | 'info' | 'warn' | 'error',
    logger: (message: string) => Promise<void>,
) {
    const original = console[fnName];
    console[fnName] = (...data: any[]) => {
        original(...data);

        let str = '';
        for (let i = 0; i < data.length; i++) {
            if (typeof data[i] === 'object') {
                str += JSON.stringify(data[i]) + ' ';
            } else {
                str += data[i] + ' ';
            }
        }

        logger(str);
    };
}

export async function setupConsole() {
    forwardConsole('log', trace);
    forwardConsole('debug', debug);
    forwardConsole('info', info);
    forwardConsole('warn', warn);
    forwardConsole('error', error);

    await attachConsole();
}
