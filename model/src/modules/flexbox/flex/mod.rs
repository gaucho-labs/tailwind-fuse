use super::*;

pub(crate) mod flex_direction;
pub(crate) mod flex_wrap;

#[derive(Debug, Clone)]
pub struct TailwindFlex {}

// TODO: CONFIRM, this is prolly ok?
impl TailwindInstance for TailwindFlex {
    fn collision_id(&self) -> &'static str {
        "base-flex"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindFlex {
    pub fn adapt(
        pattern: &[&str],
        arbitrary: &TailwindArbitrary,
    ) -> Result<Box<dyn TailwindInstance>> {
        let out = match pattern {
            // https://tailwindcss.com/docs/display#flex
            [] if arbitrary.is_none() => TailwindDisplay::try_from("flex")?.boxed(),
            // https://tailwindcss.com/docs/flex#arbitrary-values
            [] => TailwindFlex {}.boxed(),
            // https://tailwindcss.com/docs/flex-direction
            ["row"] => TailwindFlexDirection::try_from("row")?.boxed(),
            ["row", "reverse"] => TailwindFlexDirection::try_from("row-reverse")?.boxed(),
            ["col"] => TailwindFlexDirection::try_from("column")?.boxed(),
            ["col", "reverse"] => TailwindFlexDirection::try_from("column-reverse")?.boxed(),
            // https://tailwindcss.com/docs/flex-wrap
            [s @ ("wrap" | "wrap-reverse" | "nowrap")] => TailwindFlexWrap::try_from(*s)?.boxed(),
            // https://tailwindcss.com/docs/flex
            // flex-1, flex-auto, flex-initial, flex-none + arbitrary values
            [s] => TailwindFlex {}.boxed(),
            _ => syntax_error!("Invalid decoration pattern")?,
        };
        Ok(out)
    }
}
