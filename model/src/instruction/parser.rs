use super::*;

impl<'a> TailwindInstruction<'a> {
    #[inline(never)]
    pub fn get_instance(&self) -> Result<Box<dyn TailwindInstance>> {
        let pattern = self.view_elements();
        let arbitrary = &self.view_arbitrary();
        let neg = self.negative();
        let instance = match pattern {
            // Layout System
            ["aspect", rest @ ..] => TailwindAspect::parse(rest, arbitrary)?.boxed(),
            ["container"] => TailwindContainer::default().boxed(),
            ["columns", rest @ ..] => TailwindColumns::parse(rest, arbitrary)?.boxed(),
            ["break", rest @ ..] => TailwindBreak::parse(rest, arbitrary)?,
            ["box", rest @ ..] => Self::box_adaptor(rest, arbitrary)?,
            // begin https://tailwindcss.com/docs/display
            // skip [flex, table]
            ["block"] => TailwindDisplay::try_from("block")?.boxed(),
            ["inline", "block"] => TailwindDisplay::try_from("inline-block")?.boxed(),
            ["inline"] => TailwindDisplay::try_from("inline")?.boxed(),
            ["inline", "flex"] => TailwindDisplay::try_from("inline-flex")?.boxed(),
            ["inline", "table"] => TailwindDisplay::try_from("inline-table")?.boxed(),
            ["flow", "root"] => TailwindDisplay::try_from("flow-root")?.boxed(),
            ["grid"] => TailwindDisplay::try_from("grid")?.boxed(),
            ["inline", "grid"] => TailwindDisplay::try_from("inline-grid")?.boxed(),
            ["contents"] => TailwindDisplay::try_from("contents")?.boxed(),
            ["list", "item"] => TailwindDisplay::try_from("inline-grid")?.boxed(),
            ["hidden"] => TailwindDisplay::try_from("hidden")?.boxed(),
            // https://tailwindcss.com/docs/float
            ["float", ..] => TailwindFloat::try_from("float")?.boxed(),
            ["clear", ..] => TailwindClear::try_from("clear")?.boxed(),
            ["isolate"] => TailwindIsolation::try_from("isolate")?.boxed(),
            ["isolation", "auto"] => TailwindIsolation::try_from("isolation-auto")?.boxed(),
            ["object", rest @ ..] => object_adaptor(rest, arbitrary)?,
            ["overflow", rest @ ..] => TailwindOverflow::parse(rest).boxed(),
            ["overscroll", rest @ ..] => TailwindOverscroll::parse(rest).boxed(),
            // https://tailwindcss.com/docs/position#header
            [s @ ("static" | "fixed" | "absolute" | "relative" | "sticky")] => {
                TailwindPosition::try_from(*s)?.boxed()
            }
            // https://tailwindcss.com/docs/top-right-bottom-left
            ["inset", rest @ ..] => TailwindInset::parse(rest)?.boxed(),
            ["top", ..] => TailwindTop {}.boxed(),
            ["right", ..] => TailwindRight {}.boxed(),
            ["bottom", ..] => TailwindBottom {}.boxed(),
            ["left", ..] => TailwindLeft {}.boxed(),
            // https://tailwindcss.com/docs/visibility
            [visibility @ ("visible" | "invisible" | "collapse")] => {
                TailwindVisibility::try_from(*visibility)?.boxed()
            }
            // https://tailwindcss.com/docs/z-index
            ["z", ..] => TailwindZIndex {}.boxed(),
            // Flexbox & Grid
            ["basis", ..] => TailwindBasis {}.boxed(),
            ["flex", rest @ ..] => TailwindFlex::adapt(rest, arbitrary)?,
            ["grow", ..] => TailWindGrow {}.boxed(),
            ["shrink", ..] => TailWindShrink {}.boxed(),
            ["order", ..] => TailWindOrder {}.boxed(),
            ["grid", rest @ ..] => TailwindGrid::adapt(rest, arbitrary)?,
            // https://tailwindcss.com/docs/grid-column
            ["col", rest @ ..] => TailwindColumn::parse(rest, arbitrary)?.boxed(),
            ["row", rest @ ..] => TailwindRow::parse(rest, arbitrary)?.boxed(),
            ["auto", rest @ ..] => TailwindGridAuto::parse(rest)?.boxed(),
            ["gap", rest @ ..] => TailwindGap::parse(rest, arbitrary)?.boxed(),
            ["justify", rest @ ..] => justify_adaptor(rest)?,
            ["content", rest @ ..] => TailwindContent::adapt(rest, arbitrary)?,
            ["items", rest @ ..] => TailwindItems::try_from(rest)?.boxed(),
            ["self", rest @ ..] => TailwindSelf::try_from(rest)?.boxed(),
            ["place", rest @ ..] => TailwindPlace::adapt(rest)?,
            // justify catched
            // Spacing System
            ["p" | "pl" | "pr" | "pb" | "pt" | "px" | "py", ..] => {
                TailwindPadding::parse(pattern, arbitrary, neg)?.boxed()
            }
            ["m" | "ml" | "mr" | "mb" | "mt" | "mx" | "my", ..] => {
                TailwindMargin::parse(pattern, arbitrary, neg)?.boxed()
            }
            ["space", rest @ ..] => TailwindSpace::parse(rest, arbitrary, neg)?,
            // Sizing System
            ["w", rest @ ..] => TailwindSizing::parse_width(rest, arbitrary)?.boxed(),
            ["min", "w", rest @ ..] => TailwindSizing::parse_width_min(rest, arbitrary)?.boxed(),
            ["max", "w", rest @ ..] => TailwindSizing::parse_width_max(rest, arbitrary)?.boxed(),
            ["h", rest @ ..] => TailwindSizing::parse_height(rest, arbitrary)?.boxed(),
            ["min", "h", rest @ ..] => TailwindSizing::parse_height_min(rest, arbitrary)?.boxed(),
            ["max", "h", rest @ ..] => TailwindSizing::parse_height_max(rest, arbitrary)?.boxed(),
            // Typography System
            ["font", rest @ ..] => font_adaptor(rest, arbitrary)?,
            ["text", rest @ ..] => text_adaptor(rest, arbitrary)?,
            // begin https://tailwindcss.com/docs/font-variant-numeric
            ["antialiased"] => TailwindFontSmoothing::from("todo").boxed(),
            ["subpixel", "antialiased"] => TailwindFontSmoothing::from("todo").boxed(),
            ["italic"] => TailwindFontStyle::try_from("italic")?.boxed(),
            ["not", "italic"] => TailwindFontStyle::try_from("not-italic")?.boxed(),
            // https://tailwindcss.com/docs/font-variant-numeric
            ["normal", "nums"] => TailwindFontVariantNumeric::from("normal-nums").boxed(),
            ["ordinal"] => TailwindFontVariantNumeric::from("ordinal").boxed(),
            ["slashed", "zero"] => TailwindFontVariantNumeric::from("slashed-zero").boxed(),
            ["lining", "nums"] => TailwindFontVariantNumeric::from("lining-nums").boxed(),
            ["oldstyle", "nums"] => TailwindFontVariantNumeric::from("oldstyle-nums").boxed(),
            ["proportional", "nums"] => {
                TailwindFontVariantNumeric::from("proportional-nums").boxed()
            }
            ["tabular", "nums"] => TailwindFontVariantNumeric::from("tabular-nums").boxed(),
            ["diagonal", "fractions"] => {
                TailwindFontVariantNumeric::from("diagonal-fractions").boxed()
            }
            ["stacked", "fractions"] => {
                TailwindFontVariantNumeric::from("stacked-fractions").boxed()
            }
            // https://tailwindcss.com/docs/letter-spacing
            ["tracking", rest @ ..] => TailwindTracking::parse(rest, arbitrary)?.boxed(),
            ["leading", rest @ ..] => TailwindLeading::parse(rest, arbitrary)?.boxed(),
            ["list", rest @ ..] => list_adaptor(rest, arbitrary)?,
            // https://tailwindcss.com/docs/text-decoration
            ["underline"] => TailwindDecorationLine::try_from("underline")?.boxed(),
            ["overline"] => TailwindDecorationLine::try_from("overline")?.boxed(),
            ["line", "through"] => TailwindDecorationLine::try_from("line-through")?.boxed(),
            ["no", "underline"] => TailwindDecorationLine::try_from("none")?.boxed(),
            // https://tailwindcss.com/docs/text-decoration-color
            ["decoration", rest @ ..] => TailwindDecoration::adapt(rest, arbitrary)?,
            ["underline", "offset", ..] => TailwindUnderlineOffset {}.boxed(),
            // https://tailwindcss.com/docs/text-transform
            ["uppercase"] => TailwindTextTransform::try_from("uppercase")?.boxed(),
            ["lowercase"] => TailwindTextTransform::try_from("lowercase")?.boxed(),
            ["capitalize"] => TailwindTextTransform::try_from("capitalize")?.boxed(),
            ["normal", "case"] => TailwindTextTransform::try_from("normal-case")?.boxed(),
            // https://tailwindcss.com/docs/text-overflow
            ["truncate"] => TailwindTextOverflow::try_from("truncate")?.boxed(),
            ["indent", rest @ ..] => TailwindIndent::parse(rest, arbitrary)?.boxed(),
            ["align", rest @ ..] => TailwindAlign::parse(rest, arbitrary)?.boxed(),
            ["whitespace", rest @ ..] => TailwindWhiteSpace::try_from(rest)?.boxed(),
            // break catched
            // content catched
            // Typography System Extension
            ["prose"] => todo!(),
            // Backgrounds System
            ["bg", rest @ ..] => Self::bg_adaptor(rest, arbitrary)?,
            ["from", rest @ ..] => TailwindFrom::parse(rest, arbitrary)?.boxed(),
            ["via", rest @ ..] => TailwindVia::parse(rest, arbitrary)?.boxed(),
            ["to", rest @ ..] => TailwindTo::parse(rest, arbitrary)?.boxed(),
            // Borders System
            ["rounded", rest @ ..] => TailwindRounded::parse(rest, arbitrary)?.boxed(),
            ["border", rest @ ..] => Self::border_adaptor(rest, arbitrary)?,
            ["divide", rest @ ..] => TailwindDivide::adapt(rest, arbitrary)?,
            ["outline", rest @ ..] => outline_adaptor(rest, arbitrary)?,
            ["ring", rest @ ..] => TailwindRing::adapt(rest, arbitrary)?,
            // Effects System
            ["shadow", rest @ ..] => Self::shadow_adaptor(rest, arbitrary)?,
            ["opacity", rest @ ..] => TailwindOpacity::parse(rest, arbitrary, false)?.boxed(),
            ["mix", "blend", rest @ ..] => TailwindBlend::try_from(rest)?.boxed(),
            // Filters System
            ["blur", rest @ ..] => TailwindBlur::parse(rest, arbitrary, false)?.boxed(),
            ["brightness", rest @ ..] => TailwindBrightness::parse(rest, arbitrary, false)?.boxed(),
            ["contrast", rest @ ..] => TailwindContrast::parse(rest, arbitrary, false)?.boxed(),
            ["drop", "shadow", rest @ ..] => TailwindShadow::parse(rest, arbitrary, true)?.boxed(),
            ["grayscale", rest @ ..] => TailwindGrayscale::parse(rest, arbitrary, false)?.boxed(),
            ["hue", "rotate", rest @ ..] => {
                TailwindHueRotate::parse(rest, arbitrary, false, neg)?.boxed()
            }
            ["invert", rest @ ..] => TailwindInvert::parse(rest, arbitrary, false)?.boxed(),
            ["saturate", rest @ ..] => TailwindSaturate::parse(rest, arbitrary, false)?.boxed(),
            ["sepia", rest @ ..] => TailwindSepia::parse(rest, arbitrary, false)?.boxed(),
            ["backdrop", rest @ ..] => Self::backdrop_adaptor(rest, arbitrary, neg)?,
            // Tables System
            ["table", rest @ ..] => Self::table_adaptor(rest, arbitrary)?,
            // Transitions System
            ["transition", rest @ ..] => TailwindTransition::parse(rest, arbitrary)?.boxed(),
            ["duration", rest @ ..] => TailwindDuration::parse(rest, arbitrary)?.boxed(),
            //TODO: maybe check rest is not empty?
            ["ease", ..] => TailwindEase {}.boxed(),
            ["delay", rest @ ..] => TailwindDelay::parse(rest, arbitrary)?.boxed(),
            ["animate", rest @ ..] => TailwindAnimate::parse(rest, arbitrary)?.boxed(),
            // Transforms System
            ["scale", rest @ ..] => TailwindScale::parse(rest, arbitrary, neg)?.boxed(),
            ["rotate", rest @ ..] => TailwindRotate::parse(rest, arbitrary, neg)?.boxed(),
            ["translate", rest @ ..] => TailwindTranslate::parse(rest, arbitrary, neg)?.boxed(),
            ["skew", rest @ ..] => TailwindSkew::parse(rest, arbitrary, neg)?.boxed(),
            ["origin", rest @ ..] => TailwindOrigin::parse(rest, arbitrary)?.boxed(),
            // Interactivity System
            ["accent", rest @ ..] => TailwindAccentColor::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/appearance
            ["appearance", rest @ ..] => TailwindAppearance::try_from(rest)?.boxed(),
            ["cursor", ..] => TailwindCursor {}.boxed(),
            ["caret", rest @ ..] => TailwindCaretColor::parse(rest, arbitrary)?.boxed(),
            ["pointer", "events", rest @ ..] => TailwindPointerEvents::try_from(rest)?.boxed(),
            ["resize", rest @ ..] => TailwindResize::try_from(rest)?.boxed(),
            ["scroll", rest @ ..] => scroll_adaptor(rest, arbitrary, neg)?,
            ["snap", rest @ ..] => snap_adaptor(rest)?,
            ["touch", rest @ ..] => TailwindTorch::try_from(rest)?.boxed(),
            ["select", rest @ ..] => TailwindSelect::try_from(rest)?.boxed(),
            ["will", "change", ..] => TailwindWillChange {}.boxed(),
            // SVG System
            ["fill", rest @ ..] => TailwindFillColor::parse(rest, arbitrary)?.boxed(),
            ["stroke", rest @ ..] => TailwindStroke::parse(rest, arbitrary)?,
            // Accessibility System
            ["sr", "only"] => TailwindScreenReader::new(true).boxed(),
            ["not", "sr", "only"] => TailwindScreenReader::new(false).boxed(),
            // Form System Extension
            _ => return syntax_error!("Unknown instructions: {}", self.ast.to_string(),),
        };
        Ok(instance)
    }
    #[inline]
    fn bg_adaptor(
        pattern: &[&str],
        arbitrary: &TailwindArbitrary,
    ) -> Result<Box<dyn TailwindInstance>> {
        let out = match pattern {
            // https://tailwindcss.com/docs/background-attachment
            [s @ ("fixed" | "local" | "scroll")] => {
                TailwindBackgroundAttachment::try_from(*s)?.boxed()
            }
            // https://tailwindcss.com/docs/background-clip
            ["clip", rest @ ..] => TailwindBackgroundClip::try_from(rest)?.boxed(),
            // https://tailwindcss.com/docs/background-origin
            ["origin", rest @ ..] => TailwindBackgroundOrigin::try_from(rest)?.boxed(),
            // https://tailwindcss.com/docs/background-repeat
            ["no", "repeat"] => TailwindBackgroundRepeat::try_from("no-repeat")?.boxed(),
            ["repeat", rest @ ..] => TailwindBackgroundRepeat::try_from(rest)?.boxed(),
            // https://tailwindcss.com/docs/background-size
            [s @ ("auto" | "cover" | "contain")] => TailwindBackgroundSize::try_from(*s)?.boxed(),
            // TODO: Handle override?
            // ["size", rest @ ..] => TailwindBackgroundSize::try_from(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/background-blend-mode
            ["blend", rest @ ..] => TailwindBackgroundBlend::parse(rest)?.boxed(),
            _ => TailwindBackgroundColor::parse(pattern, arbitrary)?.boxed(),
        };
        Ok(out)
    }
    #[inline]
    fn border_adaptor(
        pattern: &[&str],
        arbitrary: &TailwindArbitrary,
    ) -> Result<Box<dyn TailwindInstance>> {
        let color = |color| TailwindBorderColor::from(color).boxed();
        let out = match pattern {
            // https://tailwindcss.com/docs/border-style
            [s @ ("solid" | "dashed" | "dotted" | "double" | "hidden" | "none")] => {
                TailwindBorderStyle::try_from(*s)?.boxed()
            }
            // https://tailwindcss.com/docs/border-collapse
            [s @ ("collapse" | "separate")] => TailwindBorderCollapse::try_from(*s)?.boxed(),
            // https://tailwindcss.com/docs/border-width
            [] => TailwindBorderWidth::parse(pattern)?.boxed(), // e.g. border-[3px]
            // e.g. border-4
            ["0" | "2" | "4" | "8", ..] if arbitrary.is_none() => {
                TailwindBorderWidth::parse(pattern)?.boxed()
            }
            // e.g. border-x-2
            ["x" | "y" | "t" | "r" | "b" | "l", ..] => TailwindBorderWidth::parse(pattern)?.boxed(),
            // https://tailwindcss.com/docs/border-color
            ["black"] => color(TailwindColor::Black),
            ["white"] => color(TailwindColor::White),
            _ => TailwindBorderColor::parse(pattern, arbitrary)?.boxed(),
        };
        Ok(out)
    }

    #[inline]
    fn shadow_adaptor(
        pattern: &[&str],
        arbitrary: &TailwindArbitrary,
    ) -> Result<Box<dyn TailwindInstance>> {
        let out = match pattern {
            // https://tailwindcss.com/docs/box-shadow
            ["black" | "white" | "current" | "transparent"] => {
                TailwindShadowColor::parse(pattern, arbitrary)?.boxed()
            }
            ["color", rest @ ..] => TailwindShadowColor::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/box-shadow-color
            _ => TailwindShadow::parse(pattern, arbitrary, false)?.boxed(),
        };
        Ok(out)
    }

    #[inline]
    fn box_adaptor(str: &[&str], _: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/box-decoration-break
            ["clone"] => TailwindBoxDecoration::try_from("clone")?.boxed(),
            ["slice"] => TailwindBoxDecoration::try_from("slice")?.boxed(),
            // https://tailwindcss.com/docs/box-sizing
            ["border"] => TailwindBoxSizing::try_from("border")?.boxed(),
            ["content"] => TailwindBoxSizing::try_from("content")?.boxed(),
            _ => return syntax_error!("Unknown box instructions: {}", str.join("-")),
        };
        Ok(out)
    }

    #[inline]
    fn backdrop_adaptor(
        str: &[&str],
        arbitrary: &TailwindArbitrary,
        negative: Negative,
    ) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/backdrop-blur
            ["blur", rest @ ..] => TailwindBlur::parse(rest, arbitrary, true)?.boxed(),
            // https://tailwindcss.com/docs/backdrop-brightness
            ["brightness", rest @ ..] => TailwindBrightness::parse(rest, arbitrary, true)?.boxed(),
            // https://tailwindcss.com/docs/backdrop-contrast
            ["contrast", rest @ ..] => TailwindContrast::parse(rest, arbitrary, true)?.boxed(),
            // https://tailwindcss.com/docs/backdrop-grayscale
            ["grayscale", rest @ ..] => TailwindGrayscale::parse(rest, arbitrary, true)?.boxed(),
            // https://tailwindcss.com/docs/backdrop-hue-rotate
            ["hue", "rotate", rest @ ..] => {
                TailwindHueRotate::parse(rest, arbitrary, true, negative)?.boxed()
            }
            // https://tailwindcss.com/docs/backdrop-invert
            ["invert", rest @ ..] => TailwindInvert::parse(rest, arbitrary, true)?.boxed(),
            // https://tailwindcss.com/docs/backdrop-opacity
            ["opacity", rest @ ..] => TailwindOpacity::parse(rest, arbitrary, true)?.boxed(),
            // https://tailwindcss.com/docs/backdrop-saturate
            ["saturate", rest @ ..] => TailwindSaturate::parse(rest, arbitrary, true)?.boxed(),
            // https://tailwindcss.com/docs/backdrop-sepia
            ["sepia", rest @ ..] => TailwindSepia::parse(rest, arbitrary, true)?.boxed(),
            _ => return syntax_error!("Unknown backdrop instructions: {}", str.join("-")),
        };
        Ok(out)
    }

    #[inline]
    fn table_adaptor(
        pattern: &[&str],
        arbitrary: &TailwindArbitrary,
    ) -> Result<Box<dyn TailwindInstance>> {
        let out = match pattern {
            // https://tailwindcss.com/docs/display#flex
            [] if arbitrary.is_none() => TailwindDisplay::try_from("table")?.boxed(),
            ["caption"] => TailwindDisplay::try_from("table-caption")?.boxed(),
            ["cell"] => TailwindDisplay::try_from("table-cell")?.boxed(),
            ["column"] => TailwindDisplay::try_from("table-column")?.boxed(),
            ["column", "group"] => TailwindDisplay::try_from("table-column-group")?.boxed(),
            ["footer", "group"] => TailwindDisplay::try_from("table-footer-group")?.boxed(),
            ["header", "group"] => TailwindDisplay::try_from("table-header-group")?.boxed(),
            ["row", "group"] => TailwindDisplay::try_from("table-row-group")?.boxed(),
            ["row"] => TailwindDisplay::try_from("table-row")?.boxed(),
            // https://tailwindcss.com/docs/table-layout
            _ => TailwindTableLayout::try_from(pattern)?.boxed(),
        };
        Ok(out)
    }
}
