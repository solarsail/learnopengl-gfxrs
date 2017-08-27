use gfx;
use gfx::format::Formatted;
use image;

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];


gfx_defines! {
    vertex Vertex {
        pos: [f32; 3] = "aPos",
        uv: [f32; 2] = "aTexCoord",
    }

    constant Transfrom {
        trans: [[f32; 4]; 4] = "transform",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        transform: gfx::ConstantBuffer<Transform> = "Transform",
        out: gfx::RenderTarget<ColorFormat> = "FragColor",
    }
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32, u: f32, v: f32) -> Vertex {
        Vertex {
            pos: [x, y, z],
            uv: [u, v],
        }
    }
}

pub fn load_texture<F, R>(
    factory: &mut F,
    path: &str,
) -> gfx::handle::ShaderResourceView<R, <ColorFormat as Formatted>::View>
where
    F: gfx::Factory<R>,
    R: gfx::Resources,
{
    let img = image::open(path).unwrap().to_rgba();
    let (width, height) = img.dimensions();
    let kind = gfx::texture::Kind::D2(width as u16, height as u16, gfx::texture::AaMode::Single);
    let (_, view) = factory
        .create_texture_immutable_u8::<ColorFormat>(kind, &[&img])
        .unwrap();
    view
}
