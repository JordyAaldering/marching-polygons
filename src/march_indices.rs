#[derive(Default)]
pub struct MarchIndices {
    indices: Vec<Triangle>,
}

impl MarchIndices {
    pub(crate) fn filled(n: u8) -> Self {
        Self { indices: (0..n - 2).map(|i| Triangle::at(0, i + 1, i + 2)).collect() }
    }

    pub(crate) fn add_triangle(&mut self, a: MarchPoint, b: MarchPoint, c: MarchPoint) {
        self.indices.push(Triangle::new(a, b, c));
    }

    pub fn iter(&self) -> impl Iterator<Item = &Triangle> {
        self.indices.iter()
    }
}

impl std::fmt::Debug for MarchIndices {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.indices)
    }
}

#[derive(Clone)]
pub struct Triangle {
    pub indices: [MarchPoint; 3],
}

impl Triangle {
    pub(crate) const fn new(a: MarchPoint, b: MarchPoint, c: MarchPoint) -> Self {
        Self { indices: [a, b, c] }
    }

    pub fn map(&self, vertices: &Vec<(f32, f32)>) -> [(f32, f32); 3] {
        [
            self.indices[0].map(vertices),
            self.indices[1].map(vertices),
            self.indices[2].map(vertices),
        ]
    }

    const fn at(a: u8, b: u8, c: u8) -> Self {
        Self { indices: [MarchPoint::At(a), MarchPoint::At(b), MarchPoint::At(c)] }
    }
}

impl std::fmt::Debug for Triangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.indices)
    }
}

/// Represents an index in the corresponding vertex array of this marching polygon.
#[derive(Copy, Clone, Debug)]
pub enum MarchPoint {
    // A vertex should be added to the triangle at the position of the vertex at this index.
    At(u8),
    // A vertex should be added to the triangle at the position in between the vertices of these two indices.
    Between(u8, u8),
}

impl MarchPoint {
    fn map(self, vertices: &Vec<(f32, f32)>) -> (f32, f32) {
        match self {
            MarchPoint::At(idx) => vertices[idx as usize],
            MarchPoint::Between(i, j) => (
                (vertices[i as usize].0 + vertices[j as usize].0) * 0.5,
                (vertices[i as usize].1 + vertices[j as usize].1) * 0.5,
            ),
        }
    }
}
