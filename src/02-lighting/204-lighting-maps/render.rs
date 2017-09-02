use gfx;
use image;
use find_folder::Search;
use gfx::handle::ShaderResourceView;
use gfx::format::Formatted;

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

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

    constant Light {
        ambient: [f32; 3] = "light_ambient",
        padding1: f32 = "pad1", // prevents the shader/code offset mismatcherror
        diffuse: [f32; 3] = "light_diffuse",
        padding2: f32 = "pad2", // prevents the shader/code offset mismatch error
        specular: [f32; 3] = "light_specular",
        padding3: f32 = "pad3", // prevents the shader/code offset mismatch error
        pos: [f32; 3] = "light_pos",
    }

    constant Material {
        shininess: f32 = "material_shininess",
    }

    pipeline obj_pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        transform: gfx::ConstantBuffer<Transform> = "Transform",
        light: gfx::ConstantBuffer<Light> = "Light",
        material: gfx::ConstantBuffer<Material> = "Material",
        // TextureSampler cannot reside in constants? 'Copy trait not implemented'
        diffuse: gfx::TextureSampler<<ColorFormat as Formatted>::View> = "material_diffuse",
        specular: gfx::TextureSampler<<ColorFormat as Formatted>::View> = "material_specular",
        out: gfx::RenderTarget<ColorFormat> = "FragColor",
        out_depth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }

    pipeline light_pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        transform: gfx::ConstantBuffer<Transform> = "Transform",
        out: gfx::RenderTarget<ColorFormat> = "FragColor",
        out_depth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

impl Vertex {
    pub fn new(pos: [f32; 3], normal: [f32; 3], uv: [f32; 2]) -> Vertex {
        Vertex { pos, normal, uv }
    }
}

impl Light {
    pub fn new(ambient: [f32; 3], diffuse: [f32; 3], specular: [f32; 3], pos: [f32; 3]) -> Light {
        Light {
            ambient,
            padding1: 0.0,
            diffuse,
            padding2: 0.0,
            specular,
            padding3: 0.0,
            pos,
        }
    }
}

impl Material {
    pub fn new(shininess: f32) -> Material {
        Material {
            shininess,
        }
    }
}

pub fn load_texture<F, R>(
    factory: &mut F,
    path: &str,
) -> ShaderResourceView<R, <ColorFormat as Formatted>::View>
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
