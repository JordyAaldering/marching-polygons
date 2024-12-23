/// Represents an index in the corresponding vertex array of this marching polygon.
#[derive(Copy, Clone, Debug)]
pub enum MarchPoint {
    // A vertex should be added to the triangle at the position of the vertex at this index.
    At(u8),
    // A vertex should be added to the triangle at the position in between the vertices of these two indices.
    Between(u8, u8),
}

impl MarchPoint {
    pub fn transform(self, vertices: &Vec<(f32, f32)>) -> (f32, f32) {
        match self {
            MarchPoint::At(idx) => vertices[idx as usize],
            MarchPoint::Between(i, j) => (
                (vertices[i as usize].0 + vertices[j as usize].0) * 0.5,
                (vertices[i as usize].1 + vertices[j as usize].1) * 0.5,
            ),
        }
    }
}
