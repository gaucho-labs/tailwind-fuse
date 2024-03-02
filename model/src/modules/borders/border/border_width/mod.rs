use super::*;

#[derive(Copy, Clone, Debug)]
pub struct TailwindBorderWidth {
    kind: BorderKind,
    width: LengthUnit,
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
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        match pattern {
            ["t", rest @ ..] => Self::parse_inner(rest, BorderKind::BorderT, arbitrary),
            ["r", rest @ ..] => Self::parse_inner(rest, BorderKind::BorderR, arbitrary),
            ["b", rest @ ..] => Self::parse_inner(rest, BorderKind::BorderB, arbitrary),
            ["l", rest @ ..] => Self::parse_inner(rest, BorderKind::BorderL, arbitrary),
            ["x", rest @ ..] => Self::parse_inner(rest, BorderKind::BorderX, arbitrary),
            ["y", rest @ ..] => Self::parse_inner(rest, BorderKind::BorderY, arbitrary),
            _ => Self::parse_inner(pattern, BorderKind::Border, arbitrary),
        }
    }
    fn parse_inner(
        pattern: &[&str],
        kind: BorderKind,
        arbitrary: &TailwindArbitrary,
    ) -> Result<Self> {
        if arbitrary.is_some() {
            Ok(Self {
                kind,
                width: arbitrary.as_length_or_fraction()?,
            })
        } else {
            let width = pattern.first().unwrap_or(&"1");
            Ok(Self {
                kind,
                width: LengthUnit::px(width.parse()?),
            })
        }
    }
}
