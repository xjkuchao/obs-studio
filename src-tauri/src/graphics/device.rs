use std::sync::Arc;

use tauri::Window;

use crate::Result;

/// 表示图形设备的结构体
pub struct Device {
    /// 渲染表面
    surface: wgpu::Surface<'static>,
    /// 逻辑设备
    device: wgpu::Device,
    /// 命令队列
    queue: wgpu::Queue,
    /// 表面配置
    config: wgpu::SurfaceConfiguration,
}

impl Device {
    /// 创建新的图形设备
    ///
    /// # 参数
    ///
    /// * `window` - 与设备关联的窗口
    ///
    /// # 返回值
    ///
    /// 返回 `Result<Self>`，成功时包含新创建的 `Device` 实例
    pub fn new(window: Arc<Window>) -> Result<Self> {
        // 获取窗口大小
        let size = window.inner_size()?;

        // 创建 wgpu 实例
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // 创建渲染表面
        let surface = instance.create_surface(window.clone())?;

        // 异步请求适配器
        let adapter = tauri::async_runtime::block_on(async {
            instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::default(),
                    force_fallback_adapter: false,
                    compatible_surface: Some(&surface),
                })
                .await
                .expect("Failed to find an appropriate adapter")
        });

        // 异步创建逻辑设备和命令队列
        let (device, queue) = tauri::async_runtime::block_on(async {
            adapter
                .request_device(
                    &wgpu::DeviceDescriptor {
                        label: None,
                        required_features: wgpu::Features::empty(),
                        required_limits: if cfg!(target_arch = "wasm32") {
                            wgpu::Limits::downlevel_webgl2_defaults()
                        } else {
                            wgpu::Limits::default()
                        },
                        memory_hints: wgpu::MemoryHints::Performance,
                    },
                    None,
                )
                .await
                .expect("Failed to create device")
        });

        // 获取表面能力并创建表面配置
        let caps = surface.get_capabilities(&adapter);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: caps.formats[0],
            width: size.width,
            height: size.height,
            present_mode: caps.present_modes[0],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        // 配置表面
        surface.configure(&device, &config);

        // 返回新创建的 Device 实例
        Ok(Self {
            surface,
            device,
            queue,
            config,
        })
    }
}
