/// 导入所需的模块和类型
use std::{borrow::Cow, sync::Arc};

use anyhow::Ok;
use tauri::{AppHandle, PhysicalSize, Window};
use wgpu::{
    Device, DeviceDescriptor, Features, FragmentState, Instance, Limits, MemoryHints,
    MultisampleState, PipelineLayoutDescriptor, PowerPreference, PresentMode, PrimitiveState,
    Queue, RenderPipeline, RenderPipelineDescriptor, RequestAdapterOptions, ShaderModuleDescriptor,
    ShaderSource, Surface, SurfaceConfiguration, TextureUsages, VertexState,
};

use crate::{
    // graphics::device::Device,
    Result,
};

/// 定义图形上下文结构体
pub struct Context {
    app: Arc<AppHandle>,
    window: Arc<Window>,

    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    render_pipeline: RenderPipeline,
    config: SurfaceConfiguration,
}

impl Context {
    /// 创建新的图形上下文
    pub fn new(app: &AppHandle, window: &Window) -> Result<Self> {
        let size = window.inner_size()?;

        // 创建WGPU实例
        let instance = Instance::default();

        // 创建渲染表面
        let surface = instance.create_surface(window.clone())?;

        // 异步请求适配器
        let adapter = tauri::async_runtime::block_on(async {
            instance
                .request_adapter(&RequestAdapterOptions {
                    power_preference: PowerPreference::default(),
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
                    &DeviceDescriptor {
                        label: None,
                        required_features: Features::empty(),
                        required_limits: Limits::downlevel_webgl2_defaults()
                            .using_resolution(adapter.limits()),
                        memory_hints: MemoryHints::Performance,
                    },
                    None,
                )
                .await
                .expect("Failed to create device")
        });

        // 创建着色器模块
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: None,
            source: ShaderSource::Wgsl(Cow::Borrowed(
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

        // 创建管线布局
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        // 获取交换链能力并选择格式
        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        // 创建渲染管线
        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fs_main",
                compilation_options: Default::default(),
                targets: &[Some(swapchain_format.into())],
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        // 配置表面
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: size.width,
            height: size.height,
            present_mode: PresentMode::Fifo,
            desired_maximum_frame_latency: 2,
            alpha_mode: swapchain_capabilities.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        // 返回创建的Context实例
        Ok(Self {
            app: Arc::new(app.clone()),
            window: Arc::new(window.clone()),

            surface,
            device,
            queue,
            render_pipeline,
            config,
        })
    }

    /// 调整渲染尺寸
    pub fn resize(&mut self, size: &PhysicalSize<u32>) {
        self.config.width = size.width;
        self.config.height = size.height;
        self.surface.configure(&self.device, &self.config);
    }

    /// 执行渲染
    pub fn render(&self) {
        // 获取当前帧纹理
        let frame = self
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // 创建命令编码器
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        // 开始渲染通道
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
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.draw(0..3, 0..1);
        }

        // 提交命令并呈现帧
        self.queue.submit(std::iter::once(encoder.finish()));
        frame.present();
    }
}
