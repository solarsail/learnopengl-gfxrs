use gfx;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];


gfx_defines! {
    vertex Vertex {
        pos: [f32; 3] = "aPos",
        color: [f32; 3] = "aColor",
    }

    constant Modifier { // will appear to shaders as a uniform struct
        color_mod: [f32; 3] = "colorMod",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        modifier: gfx::ConstantBuffer<Modifier> = "Modifier",
        out: gfx::RenderTarget<ColorFormat> = "FragColor",
    }
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32, r: f32, g: f32, b: f32) -> Vertex {
        Vertex {
            pos: [x, y, z],
            color: [r, g, b],
        }
    }
}
