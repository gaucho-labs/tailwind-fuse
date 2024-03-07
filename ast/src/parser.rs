use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1, take_while1},
    character::complete::char,
    combinator::opt,
    multi::many0,
    sequence::{delimited, tuple},
    IResult,
};

use crate::{ASTVariant, AstElements, AstParseOptions, AstStyle};

pub fn parse_tailwind<'a>(
    class: &[&'a str],
    options: AstParseOptions<'a>,
) -> Vec<Result<AstStyle<'a>, &'a str>> {
    class
        .iter()
        .flat_map(|s| s.split_whitespace())
        .map(|c| match parse_style(c, &options) {
            Ok(("", style)) => Ok(style),
            _ => Err(c),
        })
        .collect()
}

#[inline]
fn parse_style<'a>(
    input: &'a str,
    options: &AstParseOptions<'a>,
) -> IResult<&'a str, AstStyle<'a>> {
    let (rest, (variants, important, negative, elements, arbitrary)) = tuple((
        many0(|s| parse_variant(options.separator, s)),
        opt(char('!')),
        opt(char('-')),
        opt(|s| parse_elements(options.prefix, s)),
        opt(parse_arbitrary),
    ))(input)?;

    let source = &input[..input.len() - rest.len()];

    let variants = variants
        .into_iter()
        .map(|v| match v {
            ASTVariant::Normal(v) => v,
            ASTVariant::DataAttribute(v) => v,
            ASTVariant::ArbitraryAttribute(v) => v,
        })
        .collect();

    Ok((
        rest,
        AstStyle {
            source,
            important: important.is_some(),
            negative: negative.is_some(),
            variants,
            elements: elements.unwrap_or_default().elements,
            arbitrary,
        },
    ))
}

#[inline]
fn parse_elements<'a>(prefix: &'a str, input: &'a str) -> IResult<&'a str, AstElements<'a>> {
    #[inline]
    fn parse_head(input: &str) -> IResult<&str, &str> {
        let stop = |c: char| -> bool {
            // space
            matches!(c, ' ' | '\n' | '\r' | '-' | '[' | ']' | '(' | ')')
        };
        take_till1(stop)(input)
    }
    #[inline]
    fn parse_rest(input: &str) -> IResult<&str, &str> {
        let (rest, (_, out)) = tuple((char('-'), parse_head))(input)?;
        Ok((rest, out))
    }

    let (rest, (_, first, other)) = tuple((tag(prefix), parse_head, many0(parse_rest)))(input)?;
    let mut out = vec![first];
    out.extend(other);
    Ok((rest, AstElements { elements: out }))
}

#[inline]
fn parse_variant<'a>(separator: &'a str, input: &'a str) -> IResult<&'a str, ASTVariant<'a>> {
    let parser = alt((
        parse_data_attribute_variant,
        parse_arbitrary_attribute_variant,
        parse_normal_variant,
    ));

    let (rest, (v, _)) = tuple((parser, tag(separator)))(input)?;
    Ok((rest, v))
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

#[inline]
fn parse_arbitrary(input: &str) -> IResult<&str, &str> {
    let parser = delimited(tag("["), take_until_unbalanced('[', ']'), tag("]"));
    let (rest, (_, arbitrary)) = tuple((opt(char('-')), parser))(input)?;
    Ok((rest, arbitrary))
}

// https://stackoverflow.com/questions/70630556/parse-allowing-nested-parentheses-in-nom
pub fn take_until_unbalanced(
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
                '\\' => {
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

    fn parse_tailwind(class: &str) -> Vec<Result<AstStyle, &str>> {
        let options = AstParseOptions::default();
        let split = class.split_whitespace().collect::<Vec<_>>();
        super::parse_tailwind(split.as_slice(), options)
    }

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
    fn test_with_options() {
        let class = "dark|hover|tw-flex";
        let class = [class];
        let options = AstParseOptions {
            prefix: "tw-",
            separator: "|",
        };
        let result = super::parse_tailwind(&class, options);
        let expected = vec![Ok(AstStyle {
            source: "dark|hover|tw-flex",
            important: false,
            negative: false,
            variants: vec!["dark", "hover"],
            elements: vec!["flex"],
            arbitrary: None,
        })];

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
        let (rest, variant) = parse_data_attribute_variant("data-[open]:flex-col").unwrap();
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
        let mut parser = many0(|s| parse_variant(":", s));
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
        let (rest, result) = parse_arbitrary_attribute_variant(input).unwrap();
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
