use gfx;
use image;
use find_folder::Search;
use gfx::handle::{Buffer, DepthStencilView, RenderTargetView, Sampler, ShaderResourceView};
use gfx::format::Formatted;
use gfx::traits::FactoryExt;
use cgmath::{Matrix4, Vector3};
use camera::Camera;

pub type ColorFormat = gfx::format::Srgba8;
pub type ShaderType = <ColorFormat as Formatted>::View;
pub type DepthFormat = gfx::format::DepthStencil;

pub const BG: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

gfx_defines! {
    vertex Vertex {
        pos: [f32; 3] = "aPos",
        normal: [f32; 3] = "aNormal",
        uv: [f32; 2] = "aTexCoord",
    }

    constant Transform {
        model: [[f32; 4]; 4] = "model",
        view: [[f32; 4]; 4] = "view",
        projection: [[f32; 4]; 4] = "projection",
    }

    constant DirLight {
        ambient: [f32; 4] = "ambient", // align with 4 * 32
        diffuse: [f32; 4] = "diffuse",
        specular: [f32; 4] = "specular",
        dir: [f32; 4] = "dir",
    }

    constant PointLight {
        ambient: [f32; 4] = "ambient", // align with 4 * 32
        diffuse: [f32; 4] = "diffuse",
        specular: [f32; 4] = "specular",
        pos: [f32; 4] = "pos",
        a0: f32 = "a0",
        a1: f32 = "a1",
        a2: f32 = "a2",
        pad: f32 = "pad",
    }

    constant LightArgs {
        num_dir: i32 = "num_dir",
        num_point: i32 = "num_point",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        transform: gfx::ConstantBuffer<Transform> = "Transform",
        dir_lights: gfx::ConstantBuffer<DirLight> = "u_dirLights",
        point_lights: gfx::ConstantBuffer<PointLight> = "u_pointLights",
        light_args: gfx::ConstantBuffer<LightArgs> = "u_lightArgs",
        // TextureSampler cannot reside in constants? 'Copy trait not implemented'
        shininess: gfx::Global<f32> = "material_shininess",
        diffuse: gfx::TextureSampler<ShaderType> = "material_diffuse",
        specular: gfx::TextureSampler<ShaderType> = "material_specular",
        view_pos: gfx::Global<[f32; 3]> = "viewPos",
        out: gfx::RenderTarget<ColorFormat> = "FragColor",
        out_depth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }

    pipeline lamp_pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        transform: gfx::ConstantBuffer<Transform> = "Transform",
        color: gfx::Global<[f32; 3]> = "light_color",
        out: gfx::RenderTarget<ColorFormat> = "FragColor",
        out_depth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

impl Vertex {
    pub fn new(pos: [f32; 3], normal: [f32; 3], uv: [f32; 2]) -> Vertex {
        Vertex { pos, normal, uv }
    }
}

impl DirLight {
    pub fn new(
        ambient: Vector3<f32>,
        diffuse: Vector3<f32>,
        specular: Vector3<f32>,
        dir: Vector3<f32>,
    ) -> DirLight {
        DirLight {
            ambient: ambient.extend(1.0).into(),
            diffuse: diffuse.extend(1.0).into(),
            specular: specular.extend(1.0).into(),
            dir: dir.extend(0.0).into(),
        }
    }
}

impl PointLight {
    pub fn new(
        ambient: Vector3<f32>,
        diffuse: Vector3<f32>,
        specular: Vector3<f32>,
        pos: Vector3<f32>,
    ) -> PointLight {
        PointLight {
            ambient: ambient.extend(1.0).into(),
            diffuse: diffuse.extend(1.0).into(),
            specular: specular.extend(1.0).into(),
            pos: pos.extend(1.0).into(),
            a0: 1.0,
            a1: 0.09,
            a2: 0.032,
            pad: 0.0,
        }
    }
}

pub fn load_texture<F, R>(factory: &mut F, path: &str) -> ShaderResourceView<R, ShaderType>
where
    F: gfx::Factory<R>,
    R: gfx::Resources,
{
    let path = Search::ParentsThenKids(4, 4).for_folder(path).unwrap();
    let img = image::open(path).unwrap().to_rgba();
    let (width, height) = img.dimensions();
    let kind = gfx::texture::Kind::D2(width as u16, height as u16, gfx::texture::AaMode::Single);
    let (_, view) = factory
        .create_texture_immutable_u8::<ColorFormat>(kind, &[&img])
        .unwrap();
    view
}

pub struct ObjectBrush<R: gfx::Resources> {
    transform: Buffer<R, Transform>,
    dir_lights: Buffer<R, DirLight>,
    point_lights: Buffer<R, PointLight>,
    light_args: Buffer<R, LightArgs>,
    pso: gfx::pso::PipelineState<R, pipe::Meta>,
    sampler: Sampler<R>,
}

impl<R: gfx::Resources> ObjectBrush<R> {
    pub fn new<F>(factory: &mut F) -> ObjectBrush<R>
    where
        F: gfx::Factory<R>,
    {
        let transform = factory.create_constant_buffer(1);
        let dir_lights = factory.create_constant_buffer(16);
        let point_lights = factory.create_constant_buffer(64);
        let light_args = factory.create_constant_buffer(1);
        let pso = factory
            .create_pipeline_simple(
                include_bytes!("shader/vertex.glsl"),
                include_bytes!("shader/fragment.glsl"),
                pipe::new(),
            )
            .expect("Cannot create PSO for object");
        let sampler = factory.create_sampler_linear();
        ObjectBrush {
            transform,
            dir_lights,
            point_lights,
            light_args,
            pso,
            sampler,
        }
    }

    pub fn draw<C>(
        &self,
        object: &Object<R>,
        dir_lights: &Vec<DirLight>,
        point_lights: &Vec<PointLight>,
        light_args: &LightArgs,
        camera: &Camera,
        render_target: &RenderTargetView<R, ColorFormat>,
        depth: &DepthStencilView<R, DepthFormat>,
        encoder: &mut gfx::Encoder<R, C>,
    ) where
        C: gfx::CommandBuffer<R>,
    {
        encoder.update_constant_buffer(
            &self.transform,
            &Transform {
                model: object.model_mat.into(),
                view: camera.view_matrix().into(),
                projection: camera.projection_matrix().into(),
            },
        );
        encoder
            .update_buffer(&self.dir_lights, &dir_lights[..], 0)
            .unwrap();
        encoder
            .update_buffer(&self.point_lights, &point_lights[..], 0)
            .unwrap();
        encoder.update_constant_buffer(&self.light_args, &light_args);
        encoder.draw(
            &object.slice,
            &self.pso,
            &pipe::Data {
                vbuf: object.vertex_buffer.clone(),
                transform: self.transform.clone(),
                dir_lights: self.dir_lights.clone(),
                point_lights: self.point_lights.clone(),
                light_args: self.light_args.clone(),
                shininess: object.material.shininess,
                diffuse: (object.material.diffuse.clone(), self.sampler.clone()),
                specular: (object.material.specular.clone(), self.sampler.clone()),
                view_pos: camera.pos().into(),
                out: render_target.clone(),
                out_depth: depth.clone(),
            },
        );
    }
}

#[derive(Clone)]
pub struct Material<R: gfx::Resources> {
    pub diffuse: ShaderResourceView<R, ShaderType>,
    pub specular: ShaderResourceView<R, ShaderType>,
    pub shininess: f32,
}

impl<R: gfx::Resources> Material<R> {
    pub fn new<F>(
        factory: &mut F,
        diffuse_texture_path: &str,
        specular_texture_path: &str,
        shininess: f32,
    ) -> Material<R>
    where
        F: gfx::Factory<R>,
    {
        let diffuse = load_texture(factory, diffuse_texture_path);
        let specular = load_texture(factory, specular_texture_path);
        Material {
            diffuse,
            specular,
            shininess,
        }
    }
}

pub struct Object<R: gfx::Resources> {
    pub vertex_buffer: Buffer<R, Vertex>,
    pub slice: gfx::Slice<R>,
    pub model_mat: Matrix4<f32>,
    pub material: Material<R>,
}

impl<R: gfx::Resources> Object<R> {
    pub fn new<F>(
        factory: &mut F,
        vertices: Vec<Vertex>,
        model_mat: Matrix4<f32>,
        material: Material<R>,
    ) -> Object<R>
    where
        F: gfx::Factory<R>,
    {
        let (vertex_buffer, slice) =
            factory.create_vertex_buffer_with_slice(vertices.as_slice(), ());
        Object {
            vertex_buffer,
            slice,
            model_mat,
            material,
        }
    }
}

pub struct LampBrush<R: gfx::Resources> {
    transform: Buffer<R, Transform>,
    pso: gfx::pso::PipelineState<R, lamp_pipe::Meta>,
}

impl<R: gfx::Resources> LampBrush<R> {
    pub fn new<F>(factory: &mut F) -> LampBrush<R>
    where
        F: gfx::Factory<R>,
    {
        let transform = factory.create_constant_buffer(1);
        let pso = factory
            .create_pipeline_simple(
                include_bytes!("shader/light_vertex.glsl"),
                include_bytes!("shader/light_fragment.glsl"),
                lamp_pipe::new(),
            )
            .expect("Cannot create PSO for lamp");
        LampBrush { transform, pso }
    }

    pub fn draw<C>(
        &self,
        lamp: &Lamp<R>,
        camera: &Camera,
        render_target: &RenderTargetView<R, ColorFormat>,
        depth: &DepthStencilView<R, DepthFormat>,
        encoder: &mut gfx::Encoder<R, C>,
    ) where
        C: gfx::CommandBuffer<R>,
    {
        encoder.update_constant_buffer(
            &self.transform,
            &Transform {
                model: lamp.model_mat.into(),
                view: camera.view_matrix().into(),
                projection: camera.projection_matrix().into(),
            },
        );
        encoder.draw(
            &lamp.slice,
            &self.pso,
            &lamp_pipe::Data {
                vbuf: lamp.vertex_buffer.clone(),
                transform: self.transform.clone(),
                color: lamp.color.into(),
                out: render_target.clone(),
                out_depth: depth.clone(),
            },
        );
    }
}

pub struct Lamp<R: gfx::Resources> {
    pub vertex_buffer: Buffer<R, Vertex>,
    pub slice: gfx::Slice<R>,
    pub model_mat: Matrix4<f32>,
    pub color: Vector3<f32>,
}

impl<R: gfx::Resources> Lamp<R> {
    pub fn new<F>(
        factory: &mut F,
        vertices: Vec<Vertex>,
        model_mat: Matrix4<f32>,
        color: Vector3<f32>,
    ) -> Lamp<R>
    where
        F: gfx::Factory<R>,
    {
        let (vertex_buffer, slice) =
            factory.create_vertex_buffer_with_slice(vertices.as_slice(), ());
        Lamp {
            vertex_buffer,
            slice,
            model_mat,
            color,
        }
    }
}
