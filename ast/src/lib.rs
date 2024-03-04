use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1, take_while1},
    character::complete::char,
    combinator::{map, opt},
    multi::many0,
    sequence::{delimited, tuple},
    IResult,
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
    pub variants: Vec<&'a str>,
    /// parts of style separated by `-`
    pub elements: Vec<&'a str>,
    /// Is a arbitrary value
    pub arbitrary: Option<&'a str>,
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

// hover, focus, aria-checked
//
// data attributes:
// data-[size=large]
//
// arbitrary
// [&:nth-child(3)]-)?variant:pseudo::
#[derive(Clone, Debug, PartialEq)]
pub enum ASTVariant<'a> {
    Normal(&'a str),
    DataAttribute(&'a str),
    ArbitraryAttribute(&'a str),
}

//
// Parsing
//

type Error<'a> = nom::error::Error<&'a str>;

pub fn parse_tailwind(class: &str) -> Vec<Result<AstStyle, &str>> {
    class
        .split_whitespace()
        .map(|c| {
            // Attempt to parse each class segment individually.
            match parse_class(c) {
                Ok((rest, Ok(style))) if rest.is_empty() => Ok(style),
                _ => Err(c),
            }
        })
        .collect()
}

fn parse_class(input: &str) -> IResult<&str, Result<AstStyle, Error>> {
    map(AstStyle::parse, Ok)(input)
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
            opt(parse_arbitrary),
        ))(input)?;

        let source = &input[..input.len() - rest.len()];

        Ok((
            rest,
            Self {
                source,
                important: important.is_some(),
                negative: negative.is_some(),
                variants: variants.into_iter().map(ASTVariant::into_str).collect(),
                elements: elements.unwrap_or_default().elements,
                arbitrary,
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
    pub fn into_str(self) -> &'a str {
        match self {
            Self::Normal(v) => v,
            Self::DataAttribute(v) => v,
            Self::ArbitraryAttribute(v) => v,
        }
    }

    #[inline]
    pub fn parse(input: &'a str) -> IResult<&'a str, Self> {
        let (rest, (v, _)) = tuple((Self::parse_one, tag(":")))(input)?;
        Ok((rest, v))
    }

    #[inline]
    fn parse_one(input: &'a str) -> IResult<&'a str, Self> {
        alt((
            Self::parse_arbitrary_attribute_variant,
            Self::parse_data_attribute_variant,
            Self::parse_normal_variant,
        ))(input)
    }

    // https://tailwindcss.com/docs/hover-focus-and-other-states#using-arbitrary-variants
    #[inline]
    fn parse_normal_variant(input: &str) -> IResult<&str, ASTVariant> {
        map(
            take_while1(|c: char| c.is_alphanumeric() || c == '-'),
            ASTVariant::Normal,
        )(input)

        // map(alphanumeric1, ASTVariant::Normal)(input)
    }

    // https://tailwindcss.com/docs/hover-focus-and-other-states#data-attributes
    #[inline]
    fn parse_data_attribute_variant(input: &str) -> IResult<&str, ASTVariant> {
        println!("INPUT DATA: {input}");
        map(
            delimited(tag("data-["), take_till1(|c| c == ']'), tag("]")),
            ASTVariant::DataAttribute,
        )(input)
    }

    // https://tailwindcss.com/docs/hover-focus-and-other-states#using-arbitrary-variants
    #[inline]
    fn parse_arbitrary_attribute_variant(input: &str) -> IResult<&str, ASTVariant> {
        map(
            delimited(tag("["), take_till1(|c| c == ']'), tag("]")),
            |_| ASTVariant::ArbitraryAttribute(input),
        )(input)
    }
}

#[inline]
pub fn parse_arbitrary(input: &str) -> IResult<&str, &str> {
    let pair = delimited(char('['), take_till1(|c| c == ']'), char(']'));
    let (rest, (_, arbitrary)) = tuple((char('-'), pair))(input)?;
    Ok((rest, arbitrary))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_parse() {
        let class = "flex items-center justify-between";
        let result = parse_tailwind(class);
        let expected = vec![
            Ok(AstStyle {
                source: "flex",
                important: false,
                negative: false,
                variants: vec![],
                elements: vec!["flex"],
                arbitrary: None,
            }),
            Ok(AstStyle {
                source: "items-center",
                important: false,
                negative: false,
                variants: vec![],
                elements: vec!["items", "center"],
                arbitrary: None,
            }),
            Ok(AstStyle {
                source: "justify-between",
                important: false,
                negative: false,
                variants: vec![],
                elements: vec!["justify", "between"],
                arbitrary: None,
            }),
        ];

        assert_eq!(result, expected)
    }

    #[test]
    fn parse_with_negative() {
        let class = "-my-2";
        let result = parse_tailwind(class);
        let expected = vec![Ok(AstStyle {
            source: "-my-2",
            important: false,
            negative: true,
            variants: vec![],
            elements: vec!["my", "2"],
            arbitrary: None,
        })];
        assert_eq!(result, expected)
    }

    #[test]
    fn test_with_important() {
        let class = "!bg-blue-500";
        let result = parse_tailwind(class);
        let expected = vec![Ok(AstStyle {
            source: "!bg-blue-500",
            important: true,
            negative: false,
            variants: vec![],
            elements: vec!["bg", "blue", "500"],
            arbitrary: None,
        })];
        assert_eq!(result, expected)
    }

    #[test]
    fn multiple_variants() {
        let class = "hover:md:flex";
        let result = parse_tailwind(class);
        let expected = vec![Ok(AstStyle {
            source: "hover:md:flex",
            important: false,
            negative: false,
            variants: vec!["hover", "md"],
            elements: vec!["flex"],
            arbitrary: None,
        })];
        assert_eq!(result, expected)
    }

    #[test]
    fn aria_attributes() {
        let class = "aria-checked:true";
        let result = parse_tailwind(class);
        let expected = vec![Ok(AstStyle {
            source: "aria-checked:true",
            important: false,
            negative: false,
            variants: vec!["aria-checked"],
            elements: vec!["true"],
            arbitrary: None,
        })];
        assert_eq!(result, expected)
    }

    #[test]
    fn arbitrary_variants() {
        let class = "[&:nth-child(3)]:underline";
        let result = parse_tailwind(class);
        let expected = vec![Ok(AstStyle {
            source: "[&:nth-child(3)]:underline",
            important: false,
            negative: false,
            variants: vec!["&:nth-child(3)"],
            elements: vec!["underline"],
            arbitrary: None,
        })];
        assert_eq!(result, expected)
    }

    #[test]
    fn test_stuff() {
        let class = "data-[open]:flex-col data-[close]:flex-row";
        let result = parse_tailwind(class);

        let expected = vec![
            Ok(AstStyle {
                source: "data-[open]:flex-col",
                important: false,
                negative: false,
                variants: vec!["data-[open]"],
                elements: vec!["flex", "col"],
                arbitrary: None,
            }),
            Ok(AstStyle {
                source: "data-[close]:flex-row",
                important: false,
                negative: false,
                variants: vec!["data-[close]"],
                elements: vec!["flex", "row"],
                arbitrary: None,
            }),
        ];

        assert_eq!(result, expected)
    }

    #[test]
    fn non_tailwind() {
        let class = "data-[key=value flex";
        let result = parse_tailwind(class);
        let expected = vec![
            Err("data-[key=value"),
            Ok(AstStyle {
                source: "flex",
                important: false,
                negative: false,
                variants: vec![],
                elements: vec!["flex"],
                arbitrary: None,
            }),
        ];
        assert_eq!(result, expected)
    }
}
