use super::*;

#[derive(Clone, Debug)]
pub struct TailwindFrom {
    color: TailwindColor,
}

#[derive(Clone, Debug)]
pub struct TailwindVia {
    color: TailwindColor,
}

#[derive(Clone, Debug)]
pub struct TailwindTo {
    color: TailwindColor,
}
crate::macros::color_instance!(TailwindFrom);
crate::macros::color_instance!(TailwindVia);
crate::macros::color_instance!(TailwindTo);

impl Display for TailwindFrom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "from-{}", self.color)
    }
}
impl Display for TailwindVia {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "via-{}", self.color)
    }
}
impl Display for TailwindTo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "to-{}", self.color)
    }
}
impl TailwindInstance for TailwindFrom {
    fn collision_id(&self) -> String {
        "from-".to_string()
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}
impl TailwindInstance for TailwindVia {
    fn collision_id(&self) -> String {
        "via-".to_string()
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindInstance for TailwindTo {
    fn collision_id(&self) -> String {
        "to-".to_string()
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}
