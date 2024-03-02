#[derive(Copy, Clone, Debug)]
pub enum AxisXY {
    X,
    Y,
    N,
}

impl AxisXY {
    pub fn split_xy<'a, 'b>(pattern: &'a [&'b str]) -> (Self, &'a [&'b str]) {
        match pattern {
            ["x", rest @ ..] => (AxisXY::X, rest),
            ["y", rest @ ..] => (AxisXY::Y, rest),
            _ => unreachable!(),
        }
    }

    pub fn split_xyn<'a, 'b>(pattern: &'a [&'b str]) -> (Self, &'a [&'b str]) {
        match pattern {
            ["x", rest @ ..] => (AxisXY::X, rest),
            ["y", rest @ ..] => (AxisXY::Y, rest),
            _ => (AxisXY::N, pattern),
        }
    }
}

impl From<bool> for AxisXY {
    fn from(s: bool) -> Self {
        match s {
            true => Self::X,
            false => Self::Y,
        }
    }
}

impl From<Option<bool>> for AxisXY {
    fn from(s: Option<bool>) -> Self {
        match s {
            Some(true) => Self::X,
            Some(false) => Self::Y,
            None => Self::N,
        }
    }
}
