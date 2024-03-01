use super::*;

#[derive(Copy, Clone, Debug)]
pub struct TailwindRounded {
    kind: RoundedKind,
    size: LengthUnit,
}

#[derive(Copy, Clone, Debug)]
enum RoundedKind {
    Rounded,
    RoundedT,
    RoundedR,
    RoundedB,
    RoundedL,
    RoundedTL,
    RoundedTR,
    RoundedBL,
    RoundedBR,
}

impl RoundedKind {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Rounded => "rounded",
            Self::RoundedT => "rounded-t",
            Self::RoundedR => "rounded-r",
            Self::RoundedB => "rounded-b",
            Self::RoundedL => "rounded-l",
            Self::RoundedTL => "rounded-tl",
            Self::RoundedTR => "rounded-tr",
            Self::RoundedBL => "rounded-bl",
            Self::RoundedBR => "rounded-br",
        }
    }
}

impl Display for RoundedKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rounded => write!(f, "rounded"),
            Self::RoundedT => write!(f, "rounded-t"),
            Self::RoundedR => write!(f, "rounded-r"),
            Self::RoundedB => write!(f, "rounded-b"),
            Self::RoundedL => write!(f, "rounded-l"),
            Self::RoundedTL => write!(f, "rounded-tl"),
            Self::RoundedTR => write!(f, "rounded-tr"),
            Self::RoundedBL => write!(f, "rounded-bl"),
            Self::RoundedBR => write!(f, "rounded-br"),
        }
    }
}

impl Display for TailwindRounded {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-[{}]", self.kind, self.size)
    }
}

impl TailwindInstance for TailwindRounded {
    fn collision_id(&self) -> String {
        self.kind.to_string()
    }

    // TODO: CONFIRM
    fn get_collisions(&self) -> Vec<&'static str> {
        let collisions = match self.kind {
            RoundedKind::Rounded => vec![
                RoundedKind::RoundedT,
                RoundedKind::RoundedR,
                RoundedKind::RoundedB,
                RoundedKind::RoundedL,
                RoundedKind::RoundedTL,
                RoundedKind::RoundedTR,
                RoundedKind::RoundedBL,
                RoundedKind::RoundedBR,
            ],
            RoundedKind::RoundedT => vec![RoundedKind::RoundedT],
            RoundedKind::RoundedR => vec![RoundedKind::RoundedR],
            RoundedKind::RoundedB => vec![RoundedKind::RoundedB],
            RoundedKind::RoundedL => vec![RoundedKind::RoundedL],
            RoundedKind::RoundedTL => vec![RoundedKind::RoundedTL],
            RoundedKind::RoundedTR => vec![RoundedKind::RoundedTR],
            RoundedKind::RoundedBL => vec![RoundedKind::RoundedBL],
            RoundedKind::RoundedBR => vec![RoundedKind::RoundedBR],
        };

        collisions.into_iter().map(|x| x.as_str()).collect()
    }
}

impl TailwindRounded {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        match pattern {
            ["t" | "8", rest @ ..] => Self::parse_inner(rest, RoundedKind::RoundedT, arbitrary),
            ["r" | "6", rest @ ..] => Self::parse_inner(rest, RoundedKind::RoundedR, arbitrary),
            ["b" | "2", rest @ ..] => Self::parse_inner(rest, RoundedKind::RoundedB, arbitrary),
            ["l" | "4", rest @ ..] => Self::parse_inner(rest, RoundedKind::RoundedL, arbitrary),
            ["tl" | "7", rest @ ..] => Self::parse_inner(rest, RoundedKind::RoundedTL, arbitrary),
            ["tr" | "9", rest @ ..] => Self::parse_inner(rest, RoundedKind::RoundedTR, arbitrary),
            ["bl" | "3", rest @ ..] => Self::parse_inner(rest, RoundedKind::RoundedBL, arbitrary),
            ["br" | "1", rest @ ..] => Self::parse_inner(rest, RoundedKind::RoundedBR, arbitrary),
            _ => Self::parse_inner(pattern, RoundedKind::Rounded, arbitrary),
        }
    }
    fn parse_inner(
        pattern: &[&str],
        kind: RoundedKind,
        arbitrary: &TailwindArbitrary,
    ) -> Result<Self> {
        if arbitrary.is_some() {
            return Ok(Self {
                kind,
                size: arbitrary.as_length_or_fraction()?,
            });
        }
        let rem = |n| {
            Ok(Self {
                kind,
                size: LengthUnit::rem(n),
            })
        };
        let px = |n| {
            Ok(Self {
                kind,
                size: LengthUnit::px(n),
            })
        };
        match pattern {
            ["none"] => px(0.0),
            ["sm"] => rem(0.125),
            [] => rem(0.25),
            ["md"] => rem(0.375),
            ["lg"] => rem(0.5),
            ["xl"] => rem(0.75),
            ["2xl"] => rem(1.0),
            ["3xl"] => rem(1.5),
            ["full"] => px(9999.0),
            _ => syntax_error!(""),
        }
    }
}
