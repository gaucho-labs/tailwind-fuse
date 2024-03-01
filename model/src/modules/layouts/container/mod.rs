use super::*;

#[derive(Copy, Clone, Debug, Default)]
pub struct TailwindContainer {}

impl Display for TailwindContainer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "container",)
    }
}
/// .container {
//     width: 100%
// }
//

impl TailwindInstance for TailwindContainer {
    fn collision_id(&self) -> String {
        "container".into()
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}
