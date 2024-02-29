use super::*;

#[doc=include_str!("readme.md")]
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

impl TailwindInstance for TailwindRounded {}

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
    fn parse_inner(pattern: &[&str], kind: RoundedKind, arbitrary: &TailwindArbitrary) -> Result<Self> {
        if arbitrary.is_some() {
            return Ok(Self { kind, size: arbitrary.as_length_or_fraction()? });
        }
        let rem = |n| Ok(Self { kind, size: LengthUnit::rem(n) });
        let px = |n| Ok(Self { kind, size: LengthUnit::px(n) });
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
