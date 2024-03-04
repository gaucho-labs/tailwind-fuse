use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1},
    character::complete::{alphanumeric1, char, multispace1},
    combinator::opt,
    error::Error,
    multi::{many0, separated_list0},
    sequence::{delimited, tuple},
    Err, IResult,
};

/// `not-variant:pseudo::-ast-element-[arbitrary]`
#[derive(Clone, Debug, PartialEq, Default)]
pub struct AstStyle<'a> {
    pub source: &'a str,
    /// Is a `!important` style
    pub important: bool,
    /// Is a negative style
    pub negative: bool,
    /// `hover:`, `focus:`, etc.
    pub variants: Vec<ASTVariant<'a>>,
    /// parts of style separated by `-`
    pub elements: Vec<&'a str>,
    /// Is a arbitrary value
    pub arbitrary: Option<&'a str>,
}

/// `-[.+]`
#[derive(Clone, Debug, PartialEq)]
pub struct AstArbitrary<'a> {
    /// The arbitrary value text
    pub arbitrary: &'a str,
}

/// `ast-elements`
#[derive(Clone, Debug, PartialEq, Default)]
pub struct AstElements<'a> {
    /// `name-space`
    pub elements: Vec<&'a str>,
}

/// `!`
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AstImportant {}

/// `(not-)?variant:pseudo::`
#[derive(Clone, Debug, PartialEq)]
pub struct ASTVariant<'a> {
    /// `not-`
    pub not: bool,
    /// `::`
    pub pseudo: bool,
    /// `name-space`
    pub names: Vec<&'a str>,
}

//
// Parsing
//

pub fn parse_tailwind(class: &str) -> Result<Vec<AstStyle<'_>>, Err<Error<&str>>> {
    let styles = many0(tuple((multispace1, AstStyle::parse)));
    let mut styles = tuple((AstStyle::parse, styles));
    match styles(class.trim()) {
        Ok((string, (first, mut rest))) => {
            rest.insert(0, (string, first));
            let result = rest.into_iter().map(|(_, style)| style).collect();
            Ok(result)
        }
        Err(e) => Err(e),
    }
}

impl<'a> AstStyle<'a> {
    /// `v:v::-?a-a-a-[A]`
    #[inline]
    pub fn parse(input: &'a str) -> IResult<&'a str, Self> {
        let (rest, (variants, important, negative, elements, arbitrary)) = tuple((
            many0(ASTVariant::parse),
            opt(char('!')),
            opt(char('-')),
            opt(AstElements::parse),
            opt(AstArbitrary::parse),
        ))(input)?;

        let source = &input[..input.len() - rest.len()];

        Ok((
            rest,
            Self {
                source,
                important: important.is_some(),
                negative: negative.is_some(),
                variants,
                elements: elements.unwrap_or_default().elements,
                arbitrary: arbitrary.map(|s| s.arbitrary),
            },
        ))
    }
}

impl<'a> AstElements<'a> {
    /// `a(-a)*`
    #[inline]
    pub fn parse(input: &'a str) -> IResult<&'a str, Self> {
        let (rest, (first, other)) = tuple((Self::parse_head, many0(Self::parse_rest)))(input)?;
        let mut out = vec![first];
        out.extend(other.into_iter());
        Ok((rest, Self { elements: out }))
    }
    #[inline]
    fn parse_head(input: &'a str) -> IResult<&'a str, &'a str> {
        let stop = |c: char| -> bool {
            // space
            matches!(c, ' ' | '\n' | '\r' | '-' | '[' | ']' | '(' | ')')
        };
        take_till1(stop)(input)
    }
    #[inline]
    fn parse_rest(input: &'a str) -> IResult<&'a str, &'a str> {
        let (rest, (_, out)) = tuple((char('-'), Self::parse_head))(input)?;
        Ok((rest, out))
    }
}

impl<'a> ASTVariant<'a> {
    /// `(not-)?variant:pseudo::`
    ///
    /// ## Reference
    /// -
    #[inline]
    pub fn parse(input: &'a str) -> IResult<&'a str, Self> {
        let (rest, (mut v, s)) = tuple((Self::parse_one, alt((tag("::"), tag(":")))))(input)?;
        if s == "::" {
            v.pseudo = true
        } else {
            v.pseudo = Self::check_pseudo(&v.names.iter().map(<_>::as_ref).collect::<Vec<_>>());
        }
        Ok((rest, v))
    }
    /// `(not-)?(ALPHA)(-ALPHA)*`
    ///
    /// eg:
    /// - `not-focus`
    /// - `not-last-child`
    #[inline]
    fn parse_one(input: &'a str) -> IResult<&'a str, Self> {
        let not = opt(tuple((tag("not"), tag("-"))));
        let vs = separated_list0(tag("-"), alphanumeric1);
        let (rest, (not, names)) = tuple((not, vs))(input)?;
        Ok((
            rest,
            Self {
                not: not.is_some(),
                pseudo: false,
                names,
            },
        ))
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/Pseudo-elements#index
    #[inline]
    fn check_pseudo(names: &[&str]) -> bool {
        matches!(
            names,
            ["after"]
                | ["before"]
                | ["backdrop"]
                | ["marker"]
                | ["placeholder"]
                | ["selection"]
                | ["first", "line"]
                | ["first", "litter"]
                | ["first", "selector", "button"]
                | ["target", "text"]
        )
    }
}

impl<'a> AstArbitrary<'a> {
    /// `-[ANY+]`
    #[inline]
    pub fn parse(input: &'a str) -> IResult<&'a str, Self> {
        let pair = delimited(char('['), take_till1(|c| c == ']'), char(']'));
        let (rest, (_, arbitrary)) = tuple((char('-'), pair))(input)?;
        Ok((rest, Self { arbitrary }))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn basic_parse() {
        let class = "flex items-center justify-between";
        let result = parse_tailwind(class);
        let expected = Ok(vec![
            AstStyle {
                source: "flex",
                important: false,
                negative: false,
                variants: vec![],
                elements: vec!["flex"],
                arbitrary: None,
            },
            AstStyle {
                source: "items-center",
                important: false,
                negative: false,
                variants: vec![],
                elements: vec!["items", "center"],
                arbitrary: None,
            },
            AstStyle {
                source: "justify-between",
                important: false,
                negative: false,
                variants: vec![],
                elements: vec!["justify", "between"],
                arbitrary: None,
            },
        ]);

        assert_eq!(result, expected)
    }

    #[test]
    fn parse_with_negative() {
        let class = "-my-2";
        let result = parse_tailwind(class);
        let expected = Ok(vec![AstStyle {
            source: "-my-2",
            important: false,
            negative: true,
            variants: vec![],
            elements: vec!["my", "2"],
            arbitrary: None,
        }]);
        assert_eq!(result, expected)
    }

    #[test]
    fn test_with_important() {
        let class = "!bg-blue-500";
        let result = parse_tailwind(class);
        let expected = Ok(vec![AstStyle {
            source: "!bg-blue-500",
            important: true,
            negative: false,
            variants: vec![],
            elements: vec!["bg", "blue", "500"],
            arbitrary: None,
        }]);
        assert_eq!(result, expected)
    }

    #[test]
    fn non_tailwind() {
        let class = "not_any_tailwind123123!s12312314141";
        let result = parse_tailwind(class);
        println!("{:?}", result);
    }
}
