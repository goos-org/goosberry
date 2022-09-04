use crate::ecs::components::Transform2;
use crate::ecs::entity::Entity;
use crate::rendering::camera::{Camera2d, Vertex};
use crate::rendering::sprite::Sprite;
use crate::rendering::texture::Texture;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::RwLockReadGuard;
use wgpu::{
    BlendState, ColorTargetState, ColorWrites, Face, FragmentState, FrontFace, MultisampleState,
    PolygonMode, PrimitiveState, PrimitiveTopology, RenderPipelineDescriptor,
    ShaderModuleDescriptor, ShaderSource, VertexState,
};

pub struct RenderObject<'a> {
    pub pipeline: u64,
    pub texture: &'a Box<dyn Texture>,
    pub transform: Option<&'a Transform2<f32>>,
}
impl<'a> RenderObject<'a> {
    pub fn from_entities(
        entities: &'a Vec<RwLockReadGuard<Entity>>,
        camera: &mut Camera2d,
    ) -> Vec<RenderObject<'a>> {
        let mut sprites = Vec::new();
        #[allow(clippy::needless_range_loop)] // Must be a range to avoid creating a new variable
        for i in 0..entities.len() {
            sprites.push((
                entities[i].get_component::<Sprite>().unwrap(),
                entities[i].get_component::<Transform2<f32>>(),
            ));
        }
        sprites
            .iter()
            .map(|(sprite, transform)| {
                let mut hasher = DefaultHasher::new();
                sprite.shader.hash(&mut hasher);
                sprite.shader_label.hash(&mut hasher);
                let shader_hash = hasher.finish();
                let pipeline = if camera.pipelines.contains_key(&shader_hash) {
                    shader_hash
                } else {
                    let shader = camera.device.create_shader_module(ShaderModuleDescriptor {
                        label: Some(sprite.shader_label.as_str()),
                        source: ShaderSource::Wgsl(sprite.shader.into()),
                    });
                    let render_pipeline_layout =
                        camera
                            .device
                            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                                label: Some(sprite.shader_label.as_str()),
                                bind_group_layouts: &[],
                                push_constant_ranges: &[],
                            });
                    let render_pipeline =
                        camera
                            .device
                            .create_render_pipeline(&RenderPipelineDescriptor {
                                label: Some(sprite.shader_label.as_str()),
                                layout: Some(&render_pipeline_layout),
                                vertex: VertexState {
                                    module: &shader,
                                    entry_point: "vs_main",
                                    buffers: &[Vertex::desc()],
                                },
                                fragment: Some(FragmentState {
                                    module: &shader,
                                    entry_point: "fs_main",
                                    targets: &[Some(ColorTargetState {
                                        format: camera.config.format,
                                        blend: Some(BlendState::ALPHA_BLENDING),
                                        write_mask: ColorWrites::ALL,
                                    })],
                                }),
                                primitive: PrimitiveState {
                                    topology: PrimitiveTopology::TriangleList,
                                    strip_index_format: None,
                                    front_face: FrontFace::Ccw,
                                    cull_mode: Some(Face::Back),
                                    polygon_mode: PolygonMode::Fill,
                                    unclipped_depth: false,
                                    conservative: false,
                                },
                                depth_stencil: None,
                                multisample: MultisampleState {
                                    count: 1,
                                    mask: !0,
                                    alpha_to_coverage_enabled: false,
                                },
                                multiview: None,
                            });
                    camera.pipelines.insert(shader_hash, render_pipeline);
                    shader_hash
                };
                RenderObject {
                    pipeline,
                    texture: &sprite.texture,
                    transform: transform.clone(),
                }
            })
            .collect()
    }
}
