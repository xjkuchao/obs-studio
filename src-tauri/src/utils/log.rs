use chrono::Local;
use tauri::AppHandle;
use tauri::Manager;

use crate::Result;

const LOG_LEVEL: log::LevelFilter = log::LevelFilter::Debug;

pub fn setup_logger(app: &AppHandle) -> Result<()> {
    let logs = app.path().app_log_dir()?;
    if !logs.exists() {
        std::fs::create_dir_all(&logs)?;
    }

    println!("logs: {}", logs.display());

    let _ = fern::Dispatch::new()
        .filter(|metadata| metadata.level() <= LOG_LEVEL)
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{date} {level} {target}] {message}",
                date = Local::now().format("%m.%d %H:%M:%S"),
                level = record.level(),
                target = record.target(),
                message = message,
            ))
        })
        // .level(log::LevelFilter::from_str(settings.log_level.as_str())?)
        .chain(std::io::stdout())
        .chain(fern::DateBased::new(logs, "/%Y.%m.%d.log"))
        .apply()?;

    Ok(())
}
