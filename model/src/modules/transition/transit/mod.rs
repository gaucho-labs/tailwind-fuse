use super::*;

#[derive(Clone, Debug)]
enum Transition {
    None,
    All,
    Default,
    Colors,
    Opacity,
    Shadow,
    Transform,
    Arbitrary,
}

#[derive(Clone, Debug)]
pub struct TailwindTransition {
    kind: Transition,
}

impl TailwindInstance for TailwindTransition {
    fn collision_id(&self) -> &'static str {
        "transition-property".into()
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindTransition {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self {
            kind: Transition::parse(pattern, arbitrary)?,
        })
    }
}

impl Transition {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let t = match pattern {
            [] if arbitrary.is_none() => Self::Default,
            [] => Self::Arbitrary,
            ["none"] => Self::None,
            ["all"] => Self::All,
            ["colors"] => Self::Colors,
            ["opacity"] => Self::Opacity,
            ["shadow"] => Self::Shadow,
            ["transform"] => Self::Transform,
            _ => return syntax_error!("Unknown transition instructions: {}", pattern.join("-")),
        };
        Ok(t)
    }
}
