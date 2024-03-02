use std::fmt::{Display, Formatter};

mod traits;

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
