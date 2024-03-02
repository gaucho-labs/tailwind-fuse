use super::*;

impl TailwindInstance for TailwindSizing {
    fn collision_id(&self) -> &'static str {
        match self.kind {
            TailwindSizingKind::Width => "width",
            TailwindSizingKind::MinWidth => "min-width",
            TailwindSizingKind::MaxWidth => "max-width",
            TailwindSizingKind::Height => "height",
            TailwindSizingKind::MinHeight => "min-height",
            TailwindSizingKind::MaxHeight => "max-height",
        }
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}
