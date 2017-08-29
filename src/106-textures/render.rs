use gfx;
use gfx::format::Formatted;
use image;
use find_folder::Search;

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];


gfx_defines! {
    vertex Vertex {
        pos: [f32; 3] = "aPos",
        color: [f32; 3] = "aColor",
        uv: [f32; 2] = "aTexCoord",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        texture1: gfx::TextureSampler<<ColorFormat as Formatted>::View> = "ourTexture1",
        texture2: gfx::TextureSampler<<ColorFormat as Formatted>::View> = "ourTexture2",
        out: gfx::RenderTarget<ColorFormat> = "FragColor",
    }
}

impl Vertex {
    pub fn new(pos: [f32; 3], color: [f32; 3], uv: [f32; 2]) -> Vertex {
        Vertex { pos, color, uv }
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
    let path = Search::ParentsThenKids(3, 3).for_folder(path).unwrap();
    let img = image::open(path).unwrap().to_rgba();
    let (width, height) = img.dimensions();
    let kind = gfx::texture::Kind::D2(width as u16, height as u16, gfx::texture::AaMode::Single);
    let (_, view) = factory
        .create_texture_immutable_u8::<ColorFormat>(kind, &[&img])
        .unwrap();
    view
}
