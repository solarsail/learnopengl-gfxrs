use render::Vertex;

pub fn vertices() -> Vec<Vertex> {
    vec![
        Vertex::new(-0.5, -0.5, -0.5),
        Vertex::new(0.5, -0.5, -0.5),
        Vertex::new(0.5, 0.5, -0.5),
        Vertex::new(0.5, 0.5, -0.5),
        Vertex::new(-0.5, 0.5, -0.5),
        Vertex::new(-0.5, -0.5, -0.5),

        Vertex::new(-0.5, -0.5, 0.5),
        Vertex::new(0.5, -0.5, 0.5),
        Vertex::new(0.5, 0.5, 0.5),
        Vertex::new(0.5, 0.5, 0.5),
        Vertex::new(-0.5, 0.5, 0.5),
        Vertex::new(-0.5, -0.5, 0.5),

        Vertex::new(-0.5, 0.5, 0.5),
        Vertex::new(-0.5, 0.5, -0.5),
        Vertex::new(-0.5, -0.5, -0.5),
        Vertex::new(-0.5, -0.5, -0.5),
        Vertex::new(-0.5, -0.5, 0.5),
        Vertex::new(-0.5, 0.5, 0.5),

        Vertex::new(0.5, 0.5, 0.5),
        Vertex::new(0.5, 0.5, -0.5),
        Vertex::new(0.5, -0.5, -0.5),
        Vertex::new(0.5, -0.5, -0.5),
        Vertex::new(0.5, -0.5, 0.5),
        Vertex::new(0.5, 0.5, 0.5),

        Vertex::new(-0.5, -0.5, -0.5),
        Vertex::new(0.5, -0.5, -0.5),
        Vertex::new(0.5, -0.5, 0.5),
        Vertex::new(0.5, -0.5, 0.5),
        Vertex::new(-0.5, -0.5, 0.5),
        Vertex::new(-0.5, -0.5, -0.5),

        Vertex::new(-0.5, 0.5, -0.5),
        Vertex::new(0.5, 0.5, -0.5),
        Vertex::new(0.5, 0.5, 0.5),
        Vertex::new(0.5, 0.5, 0.5),
        Vertex::new(-0.5, 0.5, 0.5),
        Vertex::new(-0.5, 0.5, -0.5),
    ]
}