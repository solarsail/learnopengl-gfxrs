use gfx;
use render::{ColorFormat, DepthFormat};


gfx_defines! {
    vertex Vertex {
        pos: [f32; 3] = "aPos",
    }

    constant Transform { // will appear to shaders as a uniform struct
        model: [[f32; 4]; 4] = "model",
        view: [[f32; 4]; 4] = "view",
        projection: [[f32; 4]; 4] = "projection",
    }

    constant Lighting {
        object: [f32; 3] = "objectColor",
        padding1: f32 = "pad1",
        light: [f32; 3] = "lightColor",
        padding2: f32 = "pad2",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        transform: gfx::ConstantBuffer<Transform> = "Transform",
        light: gfx::ConstantBuffer<Lighting> = "Lighting",
        out: gfx::RenderTarget<ColorFormat> = "FragColor",
        out_depth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Vertex {
        Vertex {
            pos: [x, y, z],
        }
    }
}
