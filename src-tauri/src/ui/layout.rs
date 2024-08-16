use std::{borrow::Cow, sync::OnceLock};

use anyhow::Ok;
use tauri::{
    webview::WebviewBuilder, window::WindowBuilder, AppHandle, LogicalPosition, LogicalSize,
    Manager, RunEvent, WebviewUrl, Window,
};

use crate::{
    utils::{config::get_config, trans::t},
    Result,
};

struct PreviewContext<'window> {
    surface: wgpu::Surface<'window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    // shader: wgpu::ShaderModule,
    // pipeline_layout: wgpu::PipelineLayout,
    render_pipeline: wgpu::RenderPipeline,
    config: wgpu::SurfaceConfiguration,
}

static mut PREVIEW_CONTEXT: OnceLock<PreviewContext> = OnceLock::new();

fn wgpu_setup(_app: &AppHandle, window: &Window) -> Result<()> {
    let size = window.inner_size()?;

    let instance = wgpu::Instance::default();

    let surface = instance.create_surface(window.clone()).unwrap();

    let adapter = tauri::async_runtime::block_on(async {
        instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                // Request an adapter which can render to our surface
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropriate adapter")
    });

    // Create the logical device and command queue
    let (device, queue) = tauri::async_runtime::block_on(async {
        adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                    memory_hints: wgpu::MemoryHints::Performance,
                },
                None,
            )
            .await
            .expect("Failed to create device")
    });

    // Load the shaders from disk
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(
            r#"
    @vertex
    fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
        let x = f32(i32(in_vertex_index) - 1);
        let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
        return vec4<f32>(x, y, 0.0, 1.0);
    }

    @fragment
    fn fs_main() -> @location(0) vec4<f32> {
        return vec4<f32>(1.0, 0.0, 0.0, 1.0);
    }
    "#,
        )),
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let swapchain_format = swapchain_capabilities.formats[0];

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            compilation_options: Default::default(),
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            compilation_options: Default::default(),
            targets: &[Some(swapchain_format.into())],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None,
    });

    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
        desired_maximum_frame_latency: 2,
        alpha_mode: swapchain_capabilities.alpha_modes[0],
        view_formats: vec![],
    };

    surface.configure(&device, &config);
    unsafe {
        PREVIEW_CONTEXT.get_or_init(|| PreviewContext {
            surface,
            device,
            queue,
            // shader,
            // pipeline_layout,
            render_pipeline,
            config,
        });
    }

    Ok(())
}

pub fn layout_event(_app: &AppHandle, event: &RunEvent) -> Result<()> {
    match event {
        tauri::RunEvent::WindowEvent { event, .. } => match event {
            // tauri::WindowEvent::CloseRequested { api, .. } => {
            //     if label == "main" {
            //         api.prevent_close();

            //         app.get_window("main").unwrap().hide().unwrap();
            //     }
            // }
            tauri::WindowEvent::Resized(size) => {
                let mut config = unsafe { PREVIEW_CONTEXT.get().unwrap().config.clone() };
                config.width = size.width;
                config.height = size.height;
                unsafe {
                    PREVIEW_CONTEXT
                        .get()
                        .unwrap()
                        .surface
                        .configure(&PREVIEW_CONTEXT.get().unwrap().device, &config)
                };
            }
            _ => {}
        },
        tauri::RunEvent::MainEventsCleared => {
            let preview_context = unsafe { PREVIEW_CONTEXT.get().unwrap() };
            let frame = preview_context
                .surface
                .get_current_texture()
                .expect("Failed to acquire next swap chain texture");
            let view = frame
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());
            let mut encoder = preview_context
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });
                render_pass.set_pipeline(&preview_context.render_pipeline);
                render_pass.draw(0..3, 0..1);
            }
            preview_context
                .queue
                .submit(std::iter::once(encoder.finish()));
            frame.present();
        }
        _ => {}
    }

    Ok(())
}

pub fn setup_layout(app: &AppHandle) -> Result<()> {
    // let window = app.get_window("main").unwrap();
    let width = 1086.;
    let height = 729.;
    let window = WindowBuilder::new(app, "main")
        .inner_size(width, height)
        .build()?;

    wgpu_setup(app, &window)?;

    let size: LogicalSize<f64> = window.inner_size()?.to_logical(window.scale_factor()?);
    let total_width = size.width;
    let total_height = size.height;

    // preview 窗口高度占比
    let preview_height = 0.64;

    let docks_x = 0.;
    let docks_y = total_height * preview_height;
    let docks_width = total_width;
    let docks_height = total_height * (1.0 - preview_height);
    let docks = window.add_child(
        WebviewBuilder::new("docks", WebviewUrl::App(Default::default())).auto_resize(),
        LogicalPosition::new(docks_x, docks_y),
        LogicalSize::new(docks_width, docks_height),
    )?;

    // `main` is the first window from tauri.conf.json without an explicit label
    #[cfg(debug_assertions)]
    docks.open_devtools();

    update_title(app)?;

    Ok(())
}

pub fn update_title(app: &AppHandle) -> Result<()> {
    let window = app.get_window("main").unwrap();

    let profile = get_config("Basic", "Profile");
    let scene_collection = get_config("Basic", "SceneCollection");

    let mut title = "OBS ".to_string();
    title += &app.package_info().version.to_string();

    title += " - ";
    title += t("TitleBar.Profile")?.as_str();
    title += ": ";
    title += &profile.unwrap_or("".to_string());

    title += " - ";
    title += t("TitleBar.Scenes")?.as_str();
    title += ": ";
    title += &scene_collection.unwrap_or("".to_string());

    window.set_title(&title)?;

    Ok(())
}
