/// CLI 相关功能模块
use std::sync::OnceLock;

use tauri::AppHandle;
use tauri_plugin_cli::CliExt;

use crate::Result;

/// CLI 选项结构体
#[derive(Debug, Clone)]
pub struct CliOptions {
    pub portable_mode: bool,
    pub safe_mode: bool,
    pub disable_3p_plugins: bool,
    pub disable_shutdown_check: bool,
    pub multi: bool,
    pub log_verbose: bool,
    pub unfiltered_log: bool,
    pub opt_start_streaming: bool,
    pub opt_start_recording: bool,
    pub opt_studio_mode: bool,
    pub opt_start_replaybuffer: bool,
    pub opt_start_virtualcam: bool,
    pub opt_minimize_tray: bool,
    pub opt_allow_opengl: bool,
    pub opt_always_on_top: bool,
    pub opt_disable_updater: bool,
    pub opt_disable_missing_files_check: bool,
    pub opt_starting_collection: Option<String>,
    pub opt_starting_profile: Option<String>,
    pub opt_starting_scene: Option<String>,
}

/// 全局静态 CLI 选项
static CLI: OnceLock<CliOptions> = OnceLock::new();

/// 设置 CLI 选项
///
/// # 参数
///
/// * `app` - 应用程序句柄
///
/// # 返回值
///
/// 返回 `Result<()>`，表示操作是否成功
pub fn setup_cli(app: &AppHandle) -> Result<()> {
    let mut portable_mode: bool = false;
    let mut safe_mode: bool = false;
    let mut disable_3p_plugins: bool = false;
    let mut disable_shutdown_check: bool = false;
    let mut multi: bool = false;
    let mut log_verbose: bool = false;
    let mut unfiltered_log: bool = false;
    let mut opt_start_streaming: bool = false;
    let mut opt_start_recording: bool = false;
    let mut opt_studio_mode: bool = false;
    let mut opt_start_replaybuffer: bool = false;
    let mut opt_start_virtualcam: bool = false;
    let mut opt_minimize_tray: bool = false;
    let mut opt_allow_opengl: bool = false;
    let mut opt_always_on_top: bool = false;
    let mut opt_disable_updater: bool = false;
    let mut opt_disable_missing_files_check: bool = false;
    let mut opt_starting_collection: Option<String> = None;
    let mut opt_starting_profile: Option<String> = None;
    let mut opt_starting_scene: Option<String> = None;

    match app.cli().matches() {
        Ok(matches) => {
            for (arg, value) in matches.args {
                if value.occurrences >= 1 {
                    match arg.as_str() {
                        "version" => {
                            println!("OBS Studio - {}", env!("CARGO_PKG_VERSION"));
                            std::process::exit(0);
                        }
                        "portable" => {
                            portable_mode = true;
                        }
                        "safe-mode" => {
                            safe_mode = true;
                        }
                        "only-bundled-plugins" => {
                            disable_3p_plugins = true;
                        }
                        "disable-shutdown-check" => {
                            disable_shutdown_check = true;
                        }
                        "multi" => {
                            multi = true;
                            disable_shutdown_check = true;
                        }
                        "verbose" => {
                            log_verbose = true;
                        }
                        "unfiltered_log" => {
                            unfiltered_log = true;
                        }
                        "startstreaming" => {
                            opt_start_streaming = true;
                        }
                        "startrecording" => {
                            opt_start_recording = true;
                        }
                        "studio-mode" => {
                            opt_studio_mode = true;
                        }
                        "startreplaybuffer" => {
                            opt_start_replaybuffer = true;
                        }
                        "startvirtualcam" => {
                            opt_start_virtualcam = true;
                        }
                        "minimize-to-tray" => {
                            opt_minimize_tray = true;
                        }
                        "allow-opengl" => {
                            opt_allow_opengl = true;
                        }
                        "always-on-top" => {
                            opt_always_on_top = true;
                        }
                        "disable-updater" => {
                            opt_disable_updater = true;
                        }
                        "disable-missing-files-check" => {
                            opt_disable_missing_files_check = true;
                        }
                        "collection" => {
                            opt_starting_collection = value.value.as_str().map(|s| s.to_string());
                        }
                        "profile" => {
                            opt_starting_profile = value.value.as_str().map(|s| s.to_string());
                        }
                        "scene" => {
                            opt_starting_scene = value.value.as_str().map(|s| s.to_string());
                        }
                        _ => {}
                    }
                }
            }
        }
        Err(_) => {}
    }

    CLI.get_or_init(|| CliOptions {
        portable_mode,
        safe_mode,
        disable_3p_plugins,
        disable_shutdown_check,
        multi,
        log_verbose,
        unfiltered_log,
        opt_start_streaming,
        opt_start_recording,
        opt_studio_mode,
        opt_start_replaybuffer,
        opt_start_virtualcam,
        opt_minimize_tray,
        opt_allow_opengl,
        opt_always_on_top,
        opt_disable_updater,
        opt_disable_missing_files_check,
        opt_starting_collection,
        opt_starting_profile,
        opt_starting_scene,
    });

    Ok(())
}

/// 获取 CLI 选项
///
/// # 返回值
///
/// 返回 `Result<CliOptions>`，表示获取的 CLI 选项
pub fn cli() -> Result<CliOptions> {
    Ok(CLI.get().unwrap().clone())
}
