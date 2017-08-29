use cgmath::Vector3;
use render::Vertex;

pub fn vertices() -> Vec<Vertex> {
    vec![
        Vertex::new(-0.5, -0.5, -0.5, 0.0, 0.0),
        Vertex::new(0.5, -0.5, -0.5, 1.0, 0.0),
        Vertex::new(0.5, 0.5, -0.5, 1.0, 1.0),
        Vertex::new(0.5, 0.5, -0.5, 1.0, 1.0),
        Vertex::new(-0.5, 0.5, -0.5, 0.0, 1.0),
        Vertex::new(-0.5, -0.5, -0.5, 0.0, 0.0),

        Vertex::new(-0.5, -0.5, 0.5, 0.0, 0.0),
        Vertex::new(0.5, -0.5, 0.5, 1.0, 0.0),
        Vertex::new(0.5, 0.5, 0.5, 1.0, 1.0),
        Vertex::new(0.5, 0.5, 0.5, 1.0, 1.0),
        Vertex::new(-0.5, 0.5, 0.5, 0.0, 1.0),
        Vertex::new(-0.5, -0.5, 0.5, 0.0, 0.0),

        Vertex::new(-0.5, 0.5, 0.5, 1.0, 0.0),
        Vertex::new(-0.5, 0.5, -0.5, 1.0, 1.0),
        Vertex::new(-0.5, -0.5, -0.5, 0.0, 1.0),
        Vertex::new(-0.5, -0.5, -0.5, 0.0, 1.0),
        Vertex::new(-0.5, -0.5, 0.5, 0.0, 0.0),
        Vertex::new(-0.5, 0.5, 0.5, 1.0, 0.0),

        Vertex::new(0.5, 0.5, 0.5, 1.0, 0.0),
        Vertex::new(0.5, 0.5, -0.5, 1.0, 1.0),
        Vertex::new(0.5, -0.5, -0.5, 0.0, 1.0),
        Vertex::new(0.5, -0.5, -0.5, 0.0, 1.0),
        Vertex::new(0.5, -0.5, 0.5, 0.0, 0.0),
        Vertex::new(0.5, 0.5, 0.5, 1.0, 0.0),

        Vertex::new(-0.5, -0.5, -0.5, 0.0, 1.0),
        Vertex::new(0.5, -0.5, -0.5, 1.0, 1.0),
        Vertex::new(0.5, -0.5, 0.5, 1.0, 0.0),
        Vertex::new(0.5, -0.5, 0.5, 1.0, 0.0),
        Vertex::new(-0.5, -0.5, 0.5, 0.0, 0.0),
        Vertex::new(-0.5, -0.5, -0.5, 0.0, 1.0),

        Vertex::new(-0.5, 0.5, -0.5, 0.0, 1.0),
        Vertex::new(0.5, 0.5, -0.5, 1.0, 1.0),
        Vertex::new(0.5, 0.5, 0.5, 1.0, 0.0),
        Vertex::new(0.5, 0.5, 0.5, 1.0, 0.0),
        Vertex::new(-0.5, 0.5, 0.5, 0.0, 0.0),
        Vertex::new(-0.5, 0.5, -0.5, 0.0, 1.0),
    ]
}

pub fn cube_pos() -> Vec<Vector3<f32>> {
    vec![
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(2.0, 5.0, -15.0),
        Vector3::new(-1.5, -2.2, -2.5),
        Vector3::new(-3.8, -2.0, -12.3),
        Vector3::new(2.4, -0.4, -3.5),
        Vector3::new(-1.7, 3.0, -7.5),
        Vector3::new(1.3, -2.0, -2.5),
        Vector3::new(1.5, 2.0, -2.5),
        Vector3::new(1.5, 0.2, -1.5),
        Vector3::new(-1.3, 1.0, -1.5),
    ]
}
