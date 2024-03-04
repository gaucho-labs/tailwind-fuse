use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1, take_while1},
    character::complete::char,
    combinator::{map, opt},
    multi::many0,
    sequence::{delimited, tuple},
    IResult,
};

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

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AstElements<'a> {
    /// `name-space`
    pub elements: Vec<&'a str>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ASTVariant<'a> {
    // hover, focus, aria-checked
    Normal(&'a str),
    // data-[size=large]
    // supports-[display: grid]
    DataAttribute(&'a str),
    // [&:nth-child(3)]
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
            Self::parse_data_attribute_variant,
            Self::parse_arbitrary_attribute_variant,
            Self::parse_normal_variant,
        ))(input)
    }

    // https://tailwindcss.com/docs/hover-focus-and-other-states#using-arbitrary-variants
    #[inline]
    fn parse_normal_variant(input: &str) -> IResult<&str, ASTVariant> {
        let parser = take_while1(|c: char| c.is_alphanumeric() || c == '-');
        let (rest, result) = parser(input)?;
        Ok((rest, ASTVariant::Normal(result)))
    }

    // https://tailwindcss.com/docs/hover-focus-and-other-states#data-attributes
    // https://tailwindcss.com/docs/hover-focus-and-other-states#supports-rules
    #[inline]
    fn parse_data_attribute_variant(input: &str) -> IResult<&str, ASTVariant> {
        let tag_prefix = alt((tag("data-"), tag("supports-")));
        let mut parser = delimited(tag_prefix, take_till1(|c| c == ']'), tag("]"));
        let (rest, _) = parser(input)?;
        let entire_variant = &input[..input.len() - rest.len()];
        Ok((rest, ASTVariant::DataAttribute(entire_variant)))
    }

    // https://tailwindcss.com/docs/hover-focus-and-other-states#using-arbitrary-variants
    #[inline]
    fn parse_arbitrary_attribute_variant(input: &str) -> IResult<&str, ASTVariant> {
        let mut parser = delimited(tag("["), take_until_unbalanced('[', ']'), tag("]"));
        let (rest, _) = parser(input)?;
        let entire_variant = &input[..input.len() - rest.len()];
        Ok((rest, ASTVariant::ArbitraryAttribute(entire_variant)))
    }
}

#[inline]
fn parse_arbitrary(input: &str) -> IResult<&str, &str> {
    let parser = delimited(tag("["), take_until_unbalanced('[', ']'), tag("]"));
    let (rest, (_, arbitrary)) = tuple((opt(char('-')), parser))(input)?;
    Ok((rest, arbitrary))
}

// https://stackoverflow.com/questions/70630556/parse-allowing-nested-parentheses-in-nom
fn take_until_unbalanced(
    opening_bracket: char,
    closing_bracket: char,
) -> impl Fn(&str) -> IResult<&str, &str> {
    move |i: &str| {
        let mut index = 0;
        let mut bracket_counter = 0;
        while let Some(n) = &i[index..].find(&[opening_bracket, closing_bracket, '\\'][..]) {
            index += n;
            let mut it = i[index..].chars();
            match it.next().unwrap_or_default() {
                c if c == '\\' => {
                    // Skip the escape char `\`.
                    index += '\\'.len_utf8();
                    // Skip also the following char.
                    let c = it.next().unwrap_or_default();
                    index += c.len_utf8();
                }
                c if c == opening_bracket => {
                    bracket_counter += 1;
                    index += opening_bracket.len_utf8();
                }
                c if c == closing_bracket => {
                    // Closing bracket.
                    bracket_counter -= 1;
                    index += closing_bracket.len_utf8();
                }
                // Can not happen.
                _ => unreachable!(),
            };
            // We found the unmatched closing bracket.
            if bracket_counter == -1 {
                // We do not consume it.
                index -= closing_bracket.len_utf8();
                return Ok((&i[index..], &i[0..index]));
            };
        }

        if bracket_counter == 0 {
            Ok(("", i))
        } else {
            let error = nom::error::Error::new(i, nom::error::ErrorKind::TakeUntil);
            let error = nom::Err::Error(error);
            Err(error)
        }
    }
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
            variants: vec!["[&:nth-child(3)]"],
            elements: vec!["underline"],
            arbitrary: None,
        })];
        assert_eq!(result, expected)
    }

    #[test]
    fn test_data_attribute() {
        let (rest, variant) =
            ASTVariant::parse_data_attribute_variant("data-[open]:flex-col").unwrap();
        assert_eq!(":flex-col", rest);
        assert_eq!(ASTVariant::DataAttribute("data-[open]"), variant);

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
    fn test_variants() {
        let class = "dark:lg:hover:[&>*]:line-through";
        let result = parse_tailwind(class);
        let expected = vec![Ok(AstStyle {
            source: "dark:lg:hover:[&>*]:line-through",
            important: false,
            negative: false,
            variants: vec!["dark", "lg", "hover", "[&>*]"],
            elements: vec!["line", "through"],
            arbitrary: None,
        })];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_arbitrary_variant_parse() {
        let class = "dark:lg:hover:[&>*]:line-through";
        let mut parser = many0(ASTVariant::parse);
        let (str, variant) = parser(class).unwrap();

        assert_eq!(str, "line-through");
        let expected = vec![
            ASTVariant::Normal("dark"),
            ASTVariant::Normal("lg"),
            ASTVariant::Normal("hover"),
            ASTVariant::ArbitraryAttribute("[&>*]"),
        ];
        assert_eq!(variant, expected)
    }

    #[test]
    fn test_take_until_unbalanced() {
        let input = "[&:nth-child(3)]:underline";
        let (rest, result) = ASTVariant::parse_arbitrary_attribute_variant(input).unwrap();
        assert_eq!(rest, ":underline");
        assert_eq!(result, ASTVariant::ArbitraryAttribute("[&:nth-child(3)]"));
    }

    #[test]
    fn test_nested_variants() {
        let class = "[&[data-open]]:line-through";
        let result = parse_tailwind(class);
        let expected = vec![Ok(AstStyle {
            source: "[&[data-open]]:line-through",
            important: false,
            negative: false,
            variants: vec!["[&[data-open]]"],
            elements: vec!["line", "through"],
            arbitrary: None,
        })];

        assert_eq!(result, expected);
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

    #[test]
    fn test_double_arbitrary() {
        let class = "[&>*]:[color:blue]";
        let result = parse_tailwind(class);
        let expected = vec![Ok(AstStyle {
            source: "[&>*]:[color:blue]",
            important: false,
            negative: false,
            variants: vec!["[&>*]"],
            elements: vec![],
            arbitrary: Some("color:blue"),
        })];
        assert_eq!(result, expected)
    }
}
