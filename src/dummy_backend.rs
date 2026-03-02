//! A no-op rendering backend for use with skip_graphics_context.
//! All rendering operations are silently ignored. Texture/shader/buffer/pipeline
//! creation returns dummy IDs. This allows macroquad's Context to initialize
//! without a real GL context.

use crate::graphics::*;

pub struct DummyBackend {
    next_id: usize,
    dummy_attachments: Vec<TextureId>,
}

impl DummyBackend {
    pub fn new() -> Self {
        DummyBackend {
            next_id: 1,
            dummy_attachments: Vec::new(),
        }
    }

    fn next_texture_id(&mut self) -> TextureId {
        let id = self.next_id;
        self.next_id += 1;
        TextureId::from_raw_id(RawId::OpenGl(id as u32))
    }
}

impl RenderingBackend for DummyBackend {
    fn info(&self) -> ContextInfo {
        ContextInfo {
            backend: Backend::OpenGl,
            gl_version_string: String::new(),
            glsl_support: GlslSupport::default(),
            features: Features::default(),
        }
    }

    fn new_shader(
        &mut self,
        _shader: ShaderSource,
        _meta: ShaderMeta,
    ) -> Result<ShaderId, ShaderError> {
        let id = self.next_id;
        self.next_id += 1;
        // ShaderId is ShaderId(usize) but not pub - we need to create one.
        // Since we can't construct it directly, return an error that macroquad handles.
        // Actually, macroquad unwraps this, so we need to succeed.
        // ShaderId(usize) - we need a way to create it. Let's use transmute since it's repr transparent.
        Ok(unsafe { std::mem::transmute::<usize, ShaderId>(id) })
    }

    fn new_texture(
        &mut self,
        _access: TextureAccess,
        _data: TextureSource,
        _params: TextureParams,
    ) -> TextureId {
        self.next_texture_id()
    }

    fn new_pipeline(
        &mut self,
        _buffer_layout: &[BufferLayout],
        _attributes: &[VertexAttribute],
        _shader: ShaderId,
        _params: PipelineParams,
    ) -> Pipeline {
        let id = self.next_id;
        self.next_id += 1;
        unsafe { std::mem::transmute::<usize, Pipeline>(id) }
    }

    fn new_buffer(
        &mut self,
        _type_: BufferType,
        _usage: BufferUsage,
        _data: BufferSource,
    ) -> BufferId {
        let id = self.next_id;
        self.next_id += 1;
        unsafe { std::mem::transmute::<usize, BufferId>(id) }
    }

    fn new_render_pass_mrt(
        &mut self,
        _color_img: &[TextureId],
        _resolve_img: Option<&[TextureId]>,
        _depth_img: Option<TextureId>,
    ) -> RenderPass {
        let id = self.next_id;
        self.next_id += 1;
        unsafe { std::mem::transmute::<usize, RenderPass>(id) }
    }

    fn texture_params(&self, _texture: TextureId) -> TextureParams {
        TextureParams::default()
    }

    unsafe fn texture_raw_id(&self, _texture: TextureId) -> RawId {
        RawId::OpenGl(0)
    }

    fn texture_update_part(
        &mut self,
        _texture: TextureId,
        _x_offset: i32,
        _y_offset: i32,
        _width: i32,
        _height: i32,
        _bytes: &[u8],
    ) {
    }

    fn texture_set_min_filter(
        &mut self,
        _texture: TextureId,
        _filter: FilterMode,
        _mipmap_filter: MipmapFilterMode,
    ) {
    }

    fn texture_set_mag_filter(&mut self, _texture: TextureId, _filter: FilterMode) {}

    fn texture_set_wrap(
        &mut self,
        _texture: TextureId,
        _wrap_x: TextureWrap,
        _wrap_y: TextureWrap,
    ) {
    }

    fn texture_generate_mipmaps(&mut self, _texture: TextureId) {}

    fn texture_resize(
        &mut self,
        _texture: TextureId,
        _width: u32,
        _height: u32,
        _bytes: Option<&[u8]>,
    ) {
    }

    fn texture_read_pixels(&mut self, _texture: TextureId, _bytes: &mut [u8]) {}

    fn render_pass_color_attachments(&self, _render_pass: RenderPass) -> &[TextureId] {
        &self.dummy_attachments
    }

    fn delete_render_pass(&mut self, _render_pass: RenderPass) {}

    fn apply_pipeline(&mut self, _pipeline: &Pipeline) {}

    fn delete_pipeline(&mut self, _pipeline: Pipeline) {}

    fn buffer_update(&mut self, _buffer: BufferId, _data: BufferSource) {}

    fn buffer_size(&mut self, _buffer: BufferId) -> usize {
        0
    }

    fn delete_buffer(&mut self, _buffer: BufferId) {}

    fn delete_texture(&mut self, _texture: TextureId) {}

    fn delete_shader(&mut self, _program: ShaderId) {}

    fn apply_viewport(&mut self, _x: i32, _y: i32, _w: i32, _h: i32) {}

    fn apply_scissor_rect(&mut self, _x: i32, _y: i32, _w: i32, _h: i32) {}

    fn apply_bindings_from_slice(
        &mut self,
        _vertex_buffers: &[BufferId],
        _index_buffer: BufferId,
        _textures: &[TextureId],
    ) {
    }

    fn clear(
        &mut self,
        _color: Option<(f32, f32, f32, f32)>,
        _depth: Option<f32>,
        _stencil: Option<i32>,
    ) {
    }

    fn apply_uniforms_from_bytes(&mut self, _uniform_ptr: *const u8, _size: usize) {}

    fn begin_default_pass(&mut self, _action: PassAction) {}

    fn begin_pass(&mut self, _pass: Option<RenderPass>, _action: PassAction) {}

    fn end_render_pass(&mut self) {}

    fn commit_frame(&mut self) {}

    fn draw(&self, _base_element: i32, _num_elements: i32, _num_instances: i32) {}
}
