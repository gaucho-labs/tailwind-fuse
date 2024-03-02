use super::*;

pub(crate) mod content_align;

#[derive(Debug, Clone)]
pub struct TailwindContent {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindContent => "content", ["none"]);

impl TailwindContent {
    pub fn adapt(
        pattern: &[&str],
        arbitrary: &TailwindArbitrary,
    ) -> Result<Box<dyn TailwindInstance>> {
        let instance = match pattern {
            // this is a hack.
            // https://tailwindcss.com/docs/content#arbitrary-values
            [] if arbitrary.is_some() => TailwindContent { kind: "arbitrary" }.boxed(),
            [] => syntax_error!("No content value provided")?,
            ["none"] => TailwindContent::try_from("none")?.boxed(),
            pattern => {
                // https://tailwindcss.com/docs/align-content
                TailwindContentAlign::try_from(pattern)?.boxed()
            }
        };
        Ok(instance)
    }
}
