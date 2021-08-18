use crate::{
    create_render_pipeline,
    rendering::render_utils,
    texture,
};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VoxelVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub color_diffuse: [f32; 3],
}

impl VoxelVertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<VoxelVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                // position
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
                // normal
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                },
                // diffuse color
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 2,
                },
            ],
        }
    }
}

pub fn create_voxel_pipeline(
    device: &wgpu::Device,
    texture_format: wgpu::TextureFormat,
    light_bind_group_layout: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {
    let visibility = wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT;
    let camera_bind_group_layout =
        render_utils::create_bind_group_layout(&device, "camera_bind_layout", 0, visibility);

    let shader_module = render_utils::create_shader_module(
        &device,
        include_str!("voxel.wgsl"),
        "voxel_shader_module",
    );

    let bind_group_layouts = &[&camera_bind_group_layout, &light_bind_group_layout];
    let pipeline_layout =
        render_utils::create_pipeline_layout(&device, "voxel_pipeline", bind_group_layouts);

    println!("creating pipeline");
    let render_pipeline = create_render_pipeline(
        &device,
        &pipeline_layout,
        texture_format,
        Some(texture::Texture::DEPTH_FORMAT),
        &[VoxelVertex::desc()],
        shader_module,
        "voxel_pipeline",
    );
    render_pipeline
}
