use super::*;

#[doc=include_str!("readme.md")]
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
    fn inlineable(&self) -> bool {
        false
    }
}
