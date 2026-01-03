#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum Tool {
    Pick,
    Draw,
    Typing,
}

impl Default for Tool {
    fn default() -> Self {
        Self::Pick
    }
}
