use render::Vertex;

pub fn vertices() -> Vec<Vertex> {
    vec![
        Vertex::new(0.5, 0.5, 0.0, 1.0, 0.0),
        Vertex::new(0.5, -0.5, 0.0, 1.0, 1.0),
        Vertex::new(-0.5, -0.5, 0.0, 0.0, 1.0),
        Vertex::new(-0.5, 0.5, 0.0, 0.0, 0.0),
    ]
}

pub fn indices() -> Vec<u16> {
    vec![0, 1, 3, 1, 2, 3]
}
