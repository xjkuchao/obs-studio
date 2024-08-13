import { listen } from '@tauri-apps/api/event';
import { debug, error, info, trace, warn } from '@tauri-apps/plugin-log';

function formatMessage(...data: any[]): string {
    let str = '';
    for (let i = 0; i < data.length; i++) {
        if (typeof data[i] === 'object') {
            str += JSON.stringify(data[i]) + ' ';
        } else {
            str += data[i] + ' ';
        }
    }
    return str;
}

export async function setupConsole() {
    const original_console_log = console.log;
    const original_console_debug = console.debug;
    const original_console_info = console.info;
    const original_console_warn = console.warn;
    const original_console_error = console.error;

    console.log = (...data: any[]) => {
        original_console_log(...data);
        trace(formatMessage(...data));
    };

    console.debug = (...data: any[]) => {
        original_console_debug(...data);
        debug(formatMessage(...data));
    };

    console.info = (...data: any[]) => {
        original_console_info(...data);
        info(formatMessage(...data));
    };

    console.warn = (...data: any[]) => {
        original_console_warn(...data);
        warn(formatMessage(...data));
    };

    console.error = (...data: any[]) => {
        original_console_error(...data);
        error(formatMessage(...data));
    };

    await listen('log://log', (event) => {
        const payload = event.payload as any;
        const level = payload.level as number;
        let message = payload.message as string;
        // Strip ANSI escape codes
        message = message.replace(
            // eslint-disable-next-line no-control-regex
            /[\u001b\u009b][[()#;?]*(?:[0-9]{1,4}(?:;[0-9]{0,4})*)?[0-9A-ORZcf-nqry=><]/g,
            '',
        );

        switch (level) {
            case 1:
                original_console_log(message);
                break;
            case 2:
                original_console_debug(message);
                break;
            case 3:
                original_console_info(message);
                break;
            case 4:
                original_console_warn(message);
                break;
            case 5:
                original_console_error(message);
                break;
            default:
                // eslint-disable-next-line @typescript-eslint/restrict-template-expressions
                throw new Error(`unknown log level ${level}`);
        }
    });
}
