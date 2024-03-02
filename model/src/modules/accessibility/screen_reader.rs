use crate::TailwindInstance;

#[derive(Debug)]
pub struct TailwindScreenReader {
    sr_only: bool,
}

impl TailwindInstance for TailwindScreenReader {
    fn collision_id(&self) -> &'static str {
        "screen-reader"
    }
    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindScreenReader {
    /// https://tailwindcss.com/docs/screen-readers
    #[inline]
    pub fn new(sr_only: bool) -> Self {
        Self { sr_only }
    }
}
