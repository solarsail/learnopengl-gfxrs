use gfx;


pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

gfx_defines! {
    vertex Vertex {
        pos: [f32; 3] = "aPos",
        normal: [f32; 3] = "aNormal",
    }

    constant Transform {
        model: [[f32; 4]; 4] = "model",
        view: [[f32; 4]; 4] = "view",
        projection: [[f32; 4]; 4] = "projection",
    }

    constant Lighting {
        object: [f32; 3] = "objectColor",
        padding1: f32 = "pad1", // prevents the shader/code offset mismatch error
        light: [f32; 3] = "lightColor",
        padding2: f32 = "pad2", // prevents the shader/code offset mismatch error
        pos: [f32; 3] = "lightPos",
        padding3: f32 = "pad3", // prevents the shader/code offset mismatch error
        view: [f32; 3] = "viewPos",
    }

    pipeline obj_pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        transform: gfx::ConstantBuffer<Transform> = "Transform",
        light: gfx::ConstantBuffer<Lighting> = "Lighting",
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
    pub fn new(pos: [f32; 3], normal: [f32; 3]) -> Vertex {
        Vertex { pos, normal }
    }
}

impl Lighting {
    pub fn new(object: [f32; 3], light: [f32; 3], pos: [f32; 3], view: [f32; 3]) -> Lighting {
        Lighting {
            object,
            padding1: 0.0,
            light,
            padding2: 0.0,
            pos,
            padding3: 0.0,
            view,
        }
    }
}
