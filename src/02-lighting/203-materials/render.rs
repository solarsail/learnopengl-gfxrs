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
        ambient: [f32; 3] = "material_ambient",
        shininess: f32 = "material_shininess",
        diffuse: [f32; 3] = "material_diffuse",
        padding1: f32 = "pad1", // prevents the shader/code offset mismatch error
        specular: [f32; 3] = "material_specular",
    }

    pipeline obj_pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        transform: gfx::ConstantBuffer<Transform> = "Transform",
        light: gfx::ConstantBuffer<Light> = "Light",
        material: gfx::ConstantBuffer<Material> = "Material",
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
    pub fn new(
        ambient: [f32; 3],
        diffuse: [f32; 3],
        specular: [f32; 3],
        shininess: f32,
    ) -> Material {
        Material {
            ambient,
            shininess,
            diffuse,
            padding1: 0.0,
            specular,
        }
    }
}
