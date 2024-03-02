use super::*;

#[derive(Copy, Clone, Debug)]
pub struct TailwindBorderWidth {
    kind: BorderKind,
}

#[derive(Copy, Clone, Debug)]
enum BorderKind {
    Border,
    BorderX,
    BorderY,
    BorderT,
    BorderR,
    BorderB,
    BorderL,
}

impl BorderKind {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Border => "border",
            Self::BorderX => "border-x",
            Self::BorderY => "border-y",
            Self::BorderT => "border-t",
            Self::BorderR => "border-r",
            Self::BorderB => "border-b",
            Self::BorderL => "border-l",
        }
    }
}

impl TailwindInstance for TailwindBorderWidth {
    fn collision_id(&self) -> &'static str {
        self.kind.as_str()
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        let collisions = match self.kind {
            BorderKind::Border => vec![
                BorderKind::BorderX,
                BorderKind::BorderY,
                BorderKind::BorderT,
                BorderKind::BorderR,
                BorderKind::BorderB,
                BorderKind::BorderL,
            ],
            BorderKind::BorderX => vec![BorderKind::BorderL, BorderKind::BorderR],
            BorderKind::BorderY => vec![BorderKind::BorderT, BorderKind::BorderB],
            BorderKind::BorderT => vec![BorderKind::BorderT],
            BorderKind::BorderR => vec![BorderKind::BorderR],
            BorderKind::BorderB => vec![BorderKind::BorderB],
            BorderKind::BorderL => vec![BorderKind::BorderL],
        };

        collisions.into_iter().map(|x| x.as_str()).collect()
    }
}

impl TailwindBorderWidth {
    pub fn parse(pattern: &[&str]) -> Result<Self> {
        match pattern {
            ["t", rest @ ..] => Self::parse_inner(rest, BorderKind::BorderT),
            ["r", rest @ ..] => Self::parse_inner(rest, BorderKind::BorderR),
            ["b", rest @ ..] => Self::parse_inner(rest, BorderKind::BorderB),
            ["l", rest @ ..] => Self::parse_inner(rest, BorderKind::BorderL),
            ["x", rest @ ..] => Self::parse_inner(rest, BorderKind::BorderX),
            ["y", rest @ ..] => Self::parse_inner(rest, BorderKind::BorderY),
            _ => Self::parse_inner(pattern, BorderKind::Border),
        }
    }
    fn parse_inner(_: &[&str], kind: BorderKind) -> Result<Self> {
        Ok(Self { kind })
    }
}
