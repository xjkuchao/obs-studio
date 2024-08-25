use image::GenericImageView;

use crate::Result;

/// 表示一个纹理对象，包含 WGPU 纹理、视图和采样器
#[allow(dead_code)]
pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {
    /// 从字节数组创建纹理
    ///
    /// # 参数
    /// * `device` - WGPU 设备
    /// * `queue` - WGPU 队列
    /// * `bytes` - 包含图像数据的字节数组
    /// * `label` - 纹理的标签
    ///
    /// # 返回
    /// 返回 `Result<Self>`，成功时包含创建的 `Texture` 对象
    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        label: &str,
    ) -> Result<Self> {
        let img = image::load_from_memory(bytes)?;
        Self::from_image(device, queue, &img, Some(label))
    }

    /// 从图像创建纹理
    ///
    /// # 参数
    /// * `device` - WGPU 设备
    /// * `queue` - WGPU 队列
    /// * `img` - 动态图像对象
    /// * `label` - 可选的纹理标签
    ///
    /// # 返回
    /// 返回 `Result<Self>`，成功时包含创建的 `Texture` 对象
    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &image::DynamicImage,
        label: Option<&str>,
    ) -> Result<Self> {
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();

        // 创建纹理大小描述符
        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        // 创建 WGPU 纹理
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        // 将图像数据写入纹理
        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );

        // 创建纹理视图
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        // 创建采样器
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Ok(Self {
            texture,
            view,
            sampler,
        })
    }
}
