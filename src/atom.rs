 use nalgebra::Vector3;

#[derive(Clone, Copy, Default, Debug)]
pub struct Atom {
    pub position: Vector3<f32>,
}

impl Atom {
    pub fn new(position: Vector3<f32>) -> Self {
        Self { position }
    }
}
