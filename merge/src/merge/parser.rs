use regex_lite::Regex;

pub type Result<T> = std::result::Result<T, &'static str>;

pub fn parse(classes: &[&str], arbitrary: &str) -> Result<&'static str> {
    match classes {
        // https://tailwindcss.com/docs/aspect-ratio
        ["aspect", rest @ ..] => {
            let result = match rest {
                ["auto"] | ["square"] | ["video"] => Some("aspect"),
                [n] => parse_fraction(n).map(|_| "aspect"),
                [] => parse_fraction(arbitrary).map(|_| "aspect"),
                _ => None,
            };
            result.ok_or("Invalid aspect ratio")
        }
        // https://tailwindcss.com/docs/container
        ["container"] => Ok("container"),

        // https://tailwindcss.com/docs/columns
        ["columns", "auto"] => Ok("columns"),
        ["columns", rest] if is_t_shirt_size(rest) || rest.parse::<usize>().is_ok() => {
            Ok("columns")
        }
        ["columns"] if is_t_shirt_size(arbitrary) || arbitrary.parse::<usize>().is_ok() => {
            Ok("columns")
        }
        // https://tailwindcss.com/docs/break-after
        ["break", "after", rest] if valid_break_after(rest) => Ok("break-after"),

        // https://tailwindcss.com/docs/break-before
        ["break", "before", rest] if valid_break_after(rest) => {
            Ok("break-before")
        }
        // https://tailwindcss.com/docs/break-inside
        ["break", "inside", rest] => {
            if valid_break_after(rest) {
                Ok("break-inside")
            } else {
                Err("Invalid break-inside")
            }
        }
        // https://tailwindcss.com/docs/box-decoration-break
        ["box", "decoration", "clone"] | ["box", "decoration", "slice"] => {
            Ok("box-decoration-break")
        }
        // https://tailwindcss.com/docs/box-sizing
        ["box", "border"] | ["box" | "content"] => Ok("box-sizing"),

        // https://tailwindcss.com/docs/display
        ["block"]
        | ["inline", "block"]
        | ["inline"]
        | ["flex"]
        | ["inline","flex"]
        | ["table"]
        | ["inline","table"]
        | ["table","caption"]
        | ["table","cell"]
        | ["table","column"]
        | ["table","column","group"]
        | ["table","footer","group"]
        | ["table","header","group"]
        | ["table","row","group"]
        | ["table","row"]
        | ["flow","root"]
        | ["grid"]
        | ["inline","grid"]
        | ["contents"]
        | ["hidden"]
            if arbitrary.is_empty() =>
        {
            Ok("display")
        }

        // https://tailwindcss.com/docs/float
        ["float", "start"] | ["float", "end"] | ["float", "right"] | ["float", "none"] => {
            Ok("float")
        }

        // https:: //tailwindcss.com/docs/clear
        ["clear", "start"]
        | ["clear", "end"]
        | ["clear", "right"]
        | ["clear", "both"]
        | ["clear", "none"] => Ok("clear"),

        // https://tailwindcss.com/docs/isolation
        ["isolation"] | ["isolation", "auto"] => Ok("isolation"),

        // https://tailwindcss.com/docs/object-fit
        ["object", "contain"]
        | ["object", "cover"]
        | ["object", "fill"]
        | ["object", "none"]
        | ["object", "scale", "down"] => Ok("object-fit"),

        // https://tailwindcss.com/docs/object-position
        ["object", "bottom"]
        | ["object", "center"]
        | ["object", "left"]
        | ["object", "left", "bottom"]
        | ["object", "left", "top"]
        | ["object", "right"]
        | ["object", "right", "bottom"]
        | ["object", "right", "top"]
        | ["object", "top"] => Ok("object-position"),

        // https://tailwindcss.com/docs/overflow
        ["overflow", "auto"]
        | ["overflow", "hidden"]
        | ["overflow", "clip"]
        | ["overflow", "visible"]
        | ["overflow", "scroll"]
        | ["overflow", "x", "auto"]
        | ["overflow", "y", "auto"]
        | ["overflow","x", "hidden"]
        | ["overflow","y", "hidden"]
        | ["overflow", "x", "clip"]
        | ["overflow", "y","clip"]
        | ["overflow","x", "visible"]
        | ["overflow", "y", "visible"]
        | ["overflow","x", "scroll"]
        | ["overflow","y", "scroll"] => Ok("overflow"),

        // https://tailwindcss.com/docs/overscroll-behavior
        ["overscroll", "auto"]
        | ["overscroll", "contain"]
        | ["overscroll" | "none"]
        | ["overscroll", "y", "auto"]
        | ["overscroll", "y", "contain"]
        | ["overscroll", "y", "none"]
        | ["overscroll", "x", "auto"]
        | ["overscroll", "x", "contain"]
        | ["overscroll", "x", "none"] => Ok("overscroll-behavior"),

        // https://tailwindcss.com/docs/position
        ["static"] | ["fixed"] | ["absolute"] | ["relative"] | ["sticky"] => Ok("position"),

        // https://tailwindcss.com/docs/top-right-bottom-left
        ["inset", "x", rest @ ..] => valid_trbl(rest, arbitrary, "inset-x", "Invalid inset-x"),
        ["inset", "y", rest @ ..] => valid_trbl(rest, arbitrary, "inset-y", "Invalid inset-y"),
        ["top", rest @ ..] => valid_trbl(rest, arbitrary, "top", "Invalid top"),
        ["right", rest @ ..] => valid_trbl(rest, arbitrary, "right", "Invalid right"),
        ["bottom", rest @ ..] => valid_trbl(rest, arbitrary, "bottom", "Invalid bottom"),
        ["left", rest @ ..] => valid_trbl(rest, arbitrary, "left", "Invalid left"),
        ["start", rest @ ..] => valid_trbl(rest, arbitrary, "start", "Invalid start"),
        ["end", rest @ ..] => valid_trbl(rest, arbitrary, "end", "Invalid end"),

        // https://tailwindcss.com/docs/visibility
        ["visible"] | ["invisible"] | ["collapse"] => Ok("visibility"),

        // https://tailwindcss.com/docs/z-index
        ["z", "auto"] => Ok("z-index"),
        ["z", index] => match index.parse::<usize>().ok() {
            Some(_) => Ok("z-index"),
            None => Err("Invalid z index"),
        },
        ["z"] => match arbitrary.parse::<usize>().ok() {
            Some(_) => Ok("z-index"),
            None => Err("Invalid z index"),
        },

        // https://tailwindcss.com/docs/flex-basis
        ["basis", "full" | "auto" | "px" ] => Ok("flex-basis"),
        ["basis", rest] => {
            if parse_fraction_or_usize(rest) {
                Ok("flex-basis")
            } else {
                Err("Invalid flex-basis")
            }
        }
        ["basis"] => {
            if parse_fraction_or_usize(arbitrary) {
                Ok("flex-basis")
            } else {
                Err("Invalid flex-basis")
            }
        }

        // https://tailwindcss.com/docs/flex-direction
        ["flex", "row"]
        | ["flex", "row", "reverse"]
        | ["flex", "col"]
        | ["flex", "col", "reverse"] => Ok("flex-direction"),

        // https://tailwindcss.com/docs/flex-wrap
        ["flex", "wrap"] | ["flex", "wrap", "reverse"] | ["flex", "nowrap"] => Ok("flex-wrap"),

        // https://tailwindcss.com/docs/flex
        ["flex", "1"] | ["flex", "auto"] | ["flex", "initial"] | ["flex", "none"] => Ok("flex"),
        // TODO: check this?
        ["flex", _] => Ok("flex"),
        ["flex"] if !arbitrary.is_empty() => Ok("flex"),

        // https://tailwindcss.com/docs/flex-grow
        ["grow", ..] => Ok("flex-grow"),

        // https://tailwindcss.com/docs/flex-shrink
        ["shrink", ..] => Ok("flex-shrink"),

        // https://tailwindcss.com/docs/order
        ["order", rest] => {
            if valid_order(rest) {
                Ok("order")
            } else {
                Err("Invalid order")
            }
        }
        ["order"] => {
            if valid_order(arbitrary) {
                Ok("order")
            } else {
                Err("Invalid order")
            }
        }

        // https://tailwindcss.com/docs/grid-template-columns
        ["grid", "cols", ..] => Ok("grid-template-columns"),

        // https://tailwindcss.com/docs/grid-column
        ["col", "auto"] | ["col", "span", ..] => Ok("col-start-end"),
        ["col", "start", ..] => Ok("col-start"),
        ["col", "end", ..] => Ok("col-end"),

        // https://tailwindcss.com/docs/grid-template-rows
        ["grid", "rows", ..] => Ok("grid-template-rows"),

        // https://tailwindcss.com/docs/grid-row
        ["row", "auto"] | ["row", "span", ..] => Ok("row-start-end"),
        ["row", "start", ..] => Ok("row-start"),
        ["row", "end", ..] => Ok("row-end"),

        // https://tailwindcss.com/docs/grid-auto-flow
        ["grid", "flow", "row"]
        | ["grid", "flow", "col"]
        | ["grid", "flow", "dense"]
        | ["grid", "flow", "row", "dense"]
        | ["grid", "flow", "col", "dense"] => Ok("grid-auto-flow"),

        // https://tailwindcss.com/docs/grid-auto-columns
        ["auto", "cols", ..] => Ok("auto-cols"),

        // https://tailwindcss.com/docs/grid-auto-rows
        ["auto", "rows", ..] => Ok("auto-rows"),

        // https://tailwindcss.com/docs/gap
        ["gap", "x", ..] => Ok("gap-x"),
        ["gap", "y", ..] => Ok("gap-y"),
        ["gap", ..] => Ok("gap"),

        // https://tailwindcss.com/docs/justify-content
        ["justify", "normal"]
        | ["justify", "start"]
        | ["justify", "end"]
        | ["justify", "center"]
        | ["justify", "between"]
        | ["justify", "around"]
        | ["justify", "evenly"]
        | ["justify", "stretch"] => Ok("justify-content"),
        
        // https://tailwindcss.com/docs/justify-items
        ["justify", "items", "start"]
        | ["justify", "items", "end"]
        | ["justify", "items", "center"]
        | ["justify", "items", "stretch"] => Ok("justify-items"),

        // https://tailwindcss.com/docs/justify-self
        ["justify", "self", "auto"]
        | ["justify", "self", "start"]
        | ["justify", "self", "end"]
        | ["justify", "self", "center"]
        | ["justify", "self", "stretch"] => Ok("justify-self"),

        // https://tailwindcss.com/docs/align-content
        ["content", "normal"]
        | ["content", "center"]
        | ["content", "start"]
        | ["content", "end"]
        | ["content", "between"]
        | ["content", "around"]
        | ["content", "evenly"]
        | ["content", "baseline"]
        | ["content", "stretch"] => Ok("align-content"),

        // https://tailwindcss.com/docs/align-items
        ["items", "start"]
        | ["items", "end"]
        | ["items", "center"]
        | ["items", "baseline"]
        | ["items", "stretch"] => Ok("align-items"),

        // https://tailwindcss.com/docs/align-self
        ["self", "auto"]
        | ["self", "start"]
        | ["self", "end"]
        | ["self", "center"]
        | ["self", "stretch"]
        | ["self", "baseline"] => Ok("align-self"),
        
        // https://tailwindcss.com/docs/place-content
        ["place", "content", "center"]
        | ["place", "content", "start"]
        | ["place", "content", "end"]
        | ["place", "content", "between"]
        | ["place", "content", "around"]
        | ["place", "content", "evenly"]
        | ["place", "content", "baseline"]
        | ["place", "content", "stretch"] => Ok("place-content"),

        // https://tailwindcss.com/docs/place-items
        ["place", "items", "start"]
        | ["place", "items", "end"]
        | ["place", "items", "center"]
        | ["place", "items", "baseline"]
        | ["place", "items", "stretch"] => Ok("place-items"),

        // https://tailwindcss.com/docs/place-self
        ["place", "self", "start"]
        | ["place", "self", "end"]
        | ["place", "self", "center"]
        | ["place", "self", "baseline"]
        | ["place", "self", "stretch"] => Ok("place-items"),

        // https://tailwindcss.com/docs/padding
        ["p", ..] => Ok("padding"),
        ["pl", ..] => Ok("padding-left"),
        ["pr", ..] => Ok("padding-right"),
        ["pt", ..] => Ok("padding-top"),
        ["pb", ..] => Ok("padding-bottom"),
        ["px", ..] => Ok("padding-x"),
        ["py", ..] => Ok("padding-y"),

        // https:: //tailwindcss.com/docs/margin
        ["m", ..] => Ok("margin"),
        ["ml", ..] => Ok("margin-left"),
        ["mr", ..] => Ok("margin-right"),
        ["mt", ..] => Ok("margin-top"),
        ["mb", ..] => Ok("margin-bottom"),
        ["mx", ..] => Ok("margin-x"),
        ["my", ..] => Ok("margin-y"),
        ["ms", ..] => Ok("margin-start"),
        ["me", ..] => Ok("margin-end"),

        // https://tailwindcss.com/docs/space
        ["space", "x", "reverse"] => Ok("space-x-reverse"),
        ["space", "y", "reverse"] => Ok("space-y-reverse"),
        ["space", "x", ..] => Ok("space-x"),
        ["space", "y", ..] => Ok("space-y"),

        // https://tailwindcss.com/docs/width
        // TODO: Add validation?
        ["w", ..] => Ok("width"),

        // https://tailwindcss.com/docs/min-width
        // TODO: Add validation?
        ["min", "w", ..] => Ok("min-width"),

        // https://tailwindcss.com/docs/max-width
        ["max", "w", ..] => Ok("max-width"),

        // https://tailwindcss.com/docs/height
        ["h", ..] => Ok("height"),

        // https://tailwindcss.com/docs/min-height
        ["min", "h", ..] => Ok("min-height"),

        // https://tailwindcss.com/docs/max-height
        ["max", "h", ..] => Ok("max-height"),

        // https://tailwindcss.com/docs/size
        ["size", ..] => Ok("size"),

        // https://tailwindcss.com/docs/font-family
        // TODO: This clash is bad
        ["font", "sans"] | ["font", "serif"] | ["font", "mono"] => Ok("font-family"),

        // https://tailwindcss.com/docs/text-align
        ["text", "left"]
        | ["text", "center"]
        | ["text", "right"]
        | ["text", "justify"]
        | ["text", "start"]
        | ["text", "end"] => Ok("text-align"),

        // https://tailwindcss.com/docs/text-overflow
        ["text", "ellipsis"] | ["text", "clip"] => Ok("text-overflow"),

        // https://tailwindcss.com/docs/text-wrap
        ["text", "wrap"] | ["text", "nowrap"] | ["text", "balance"] | ["text", "pretty"] => {
            Ok("text-wrap")
        }

        // https://tailwindcss.com/docs/font-size
        ["text", rest] if valid_text_size(rest) => Ok("font-size"),
        ["text"] if valid_text_size(arbitrary) => Ok("font-size"),

        // https://tailwindcss.com/docs/text-color
        ["text", ..] => Ok("text-color"),

        // https://tailwindcss.com/docs/font-smoothing
        ["antialiased"] | ["subpixel-antialiased"] => Ok("font-smoothing"),

        // https://tailwindcss.com/docs/font-style
        ["italic"] | ["not-italic"] => Ok("font-style"),

        // https://tailwindcss.com/docs/font-weight
        ["font", ..] => Ok("font-weight"),

        // https://tailwindcss.com/docs/font-variant-numeric
        ["normal", "nums"] => Ok("fvn-normal"),
        ["ordinal"] => Ok("fvn-ordinal"),
        ["slashed", "zero"] => Ok("fvn-slashed-zero"),
        ["lining", "nums"] | ["oldstyle", "nums"] => Ok("fvn-figure"),
        ["proportional", "nums"] | ["tabular","nums"] => Ok("fvn-spacing"),
        ["diagonal", "fractions"] | ["stacked","fractions"] => Ok("fvn-fraction"),

        // https://tailwindcss.com/docs/letter-spacing
        ["tracking", ..] => Ok("letter-spacing"),

        // https://tailwindcss.com/docs/line-clamp
        ["line", "clamp", ..] => Ok("line-clamp"),

        // https://tailwindcss.com/docs/line-height
        ["leading", ..] => Ok("line-height"),

        // https://tailwindcss.com/docs/list-style-image
        ["list", "image", ..] => Ok("list-style-image"),

        // https://tailwindcss.com/docs/list-style-position
        ["list", "inside"] | ["list", "outside"] => Ok("list-style-position"),

        // https://tailwindcss.com/docs/list-style-type
        ["list", ..] => Ok("list-style-type"),

        // https://tailwindcss.com/docs/text-decoration
        ["underline"] | ["overline"] | ["line-through"] | ["no-underline"] => Ok("text-decoration"),

        // https://tailwindcss.com/docs/text-decoration-style
        ["solid"] | ["double"] | ["dotted"] | ["dashed"] | ["wavy"] => Ok("text-decoration-style"),

        // https://tailwindcss.com/docs/text-decoration-thickness
        ["decoration", "auto"] | ["decoration", "from-font"] => Ok("text-decoration-thickness"),
        ["decoration", rest] => {
            if rest.parse::<usize>().is_ok() {
                Ok("text-decoration-thickness")
            } else {
                Err("Invalid text-decoration-thickness")
            }
        }
        ["decoration"] if arbitrary.parse::<usize>().is_ok() => Ok("text-decoration-thickness"),

        // https://tailwindcss.com/docs/text-decoration-color
        ["decoration", ..] => Ok("text-decoration-color"),

        // https://tailwindcss.com/docs/text-underline-offset
        ["underline", "offset", ..] => Ok("text-underline-offset"),

        // https://tailwindcss.com/docs/text-transform
        ["uppercase"] | ["lowercase"] | ["capitalize"] | ["normal-case"] => Ok("text-transform"),

        // https://tailwindcss.com/docs/text-overflow
        ["truncate"] => Ok("text-overflow"),

        // https://tailwindcss.com/docs/text-indent
        ["indent", ..] => Ok("text-indent"),

        // https://tailwindcss.com/docs/vertical-align
        ["align", ..] => Ok("vertical-align"),

        // https://tailwindcss.com/docs/whitespace
        ["whitespace", "normal"]
        | ["whitespace", "nowrap"]
        | ["whitespace", "pre"]
        | ["whitespace", "pre", "line"]
        | ["whitespace", "pre", "wrap"]
        | ["whitespace", "break", "spaces"] => Ok("whitespace"),

        // https://tailwindcss.com/docs/word-break
        ["break", "normal"] | ["break", "words"] | ["break", "all"] | ["break", "keep"] => {
            Ok("word-break")
        }

        // https://tailwindcss.com/docs/hyphens
        ["hyphens", "none"] | ["hyphens", "manual"] | ["hyphens", "auto"] => Ok("hyphens"),

        // https://tailwindcss.com/docs/content
        ["content", "none"] => Ok("content"),

        // https://tailwindcss.com/docs/background-attachment
        ["bg", "fixed"] | ["bg", "local"] | ["bg", "scroll"] => Ok("background-attachment"),

        // https://tailwindcss.com/docs/background-clip
        ["bg", "clip", "border"]
        | ["bg", "clip", "padding"]
        | ["bg", "clip", "content"]
        | ["bg", "clip", "text"] => Ok("background-clip"),

        // https://tailwindcss.com/docs/background-origin
        ["bg", "origin", "border"] | ["bg", "origin", "padding"] | ["bg", "origin", "content"] => {
            Ok("background-origin")
        }

        // https://tailwindcss.com/docs/background-repeat
        ["bg", "repeat"]
        | ["bg", "no","repeat"]
        | ["bg", "repeat", "x"]
        | ["bg", "repeat", "y"]
        | ["bg", "repeat", "round"]
        | ["bg", "repeat", "space"] => Ok("background-repeat"),

        // https://tailwindcss.com/docs/background-position
        // TODO: Integrate arbitrary value? (e.g. bg-[center_top_1rem])
        ["bg", "bottom"]
        | ["bg", "center"]
        | ["bg", "left"]
        | ["bg", "left", "bottom"]
        | ["bg", "left", "top"]
        | ["bg", "right"]
        | ["bg", "right", "bottom"]
        | ["bg", "right", "top"]
        | ["bg", "top"] => Ok("background-position"),

        // https://tailwindcss.com/docs/background-size
        // TODO: implement arbitrary and custom values?
        ["bg", "auto"] | ["bg", "cover"] | ["bg", "contain"] => Ok("background-size"),

        // https://tailwindcss.com/docs/background-image
        // TODO: implement arbitrary and custom values?
        ["bg", "none"] | ["bg", "gradient", "to", ..] => Ok("background-image"),

        // https://tailwindcss.com/docs/background-blend-mode
        // TODO: plus-lighter not valid
        ["bg", "blend", mode @ ..] if valid_blend(mode) => Ok("background-blend-mode"),

        // https://tailwindcss.com/docs/background-color
        ["bg", ..] => Ok("background-color"),

        // https://tailwindcss.com/docs/gradient-color-stops
        // TODO: Review this?
        ["from", ..] => Ok("from"),
        ["via", ..] => Ok("via"),
        ["to", ..] => Ok("to"),

        // https://tailwindcss.com/docs/border-radius
        // TODO: Review
        ["rounded", "t", ..] => Ok("rounded-t"),
        ["rounded", "r", ..] => Ok("rounded-r"),
        ["rounded", "b", ..] => Ok("rounded-b"),
        ["rounded", "l", ..] => Ok("rounded-l"),
        ["rounded", "tl", ..] => Ok("rounded-tl"),
        ["rounded", "tr", ..] => Ok("rounded-tr"),
        ["rounded", "bl", ..] => Ok("rounded-bl"),
        ["rounded", "br", ..] => Ok("rounded-br"),

        ["rounded", "s", ..] => Ok("rounded-s"),
        ["rounded", "e", ..] => Ok("rounded-e"),

        ["rounded", "ss", ..] => Ok("rounded-ss"),
        ["rounded", "se", ..] => Ok("rounded-se"),
        ["rounded", "ee", ..] => Ok("rounded-ee"),
        ["rounded", "es", ..] => Ok("rounded-es"),

        ["rounded", ..] => Ok("rounded"),

        // https://tailwindcss.com/docs/border-width
        // TODO: need to parse and make sure integer. 
        // Border color can be applied to a side.
        ["border", "t", ..] => Ok("border-w-t"),
        ["border", "r", ..] => Ok("border-w-r"),
        ["border", "b", ..] => Ok("border-w-b"),
        ["border", "l", ..] => Ok("border-w-l"),
        ["border", "s", ..] => Ok("border-w-s"),
        ["border", "e", ..] => Ok("border-w-e"),
        ["border", rest] if rest.parse::<usize>().is_ok() => Ok("border-w"),
        ["border"] if arbitrary.parse::<usize>().is_ok() => Ok("border-w"),

        // https://tailwindcss.com/docs/border-style
        ["border", "solid"]
        | ["border", "dashed"]
        | ["border", "dotted"]
        | ["border", "double"]
        | ["border", "hidden"]
        | ["border", "none"] => Ok("border-style"),

        // https://tailwindcss.com/docs/border-collapse
        ["border", "collapse"] | ["border", "separate"] => Ok("border-collapse"),

        // https://tailwindcss.com/docs/border-spacing
        ["border", "spacing", "x", ..] => Ok("border-spacing-x"),
        ["border", "spacing", "y", ..] => Ok("border-spacing-y"),
        ["border", "spacing", ..] => Ok("border-spacing"),

        // https://tailwindcss.com/docs/border-color
        // TODO: handle sides.
        ["border", ..] => Ok("border-color"),

        // https://tailwindcss.com/docs/divide-style
        ["divide", "solid"]
        | ["divide", "dashed"]
        | ["divide", "dotted"]
        | ["divide", "double"]
        | ["divide", "none"] => Ok("divide-style"),

        // https://tailwindcss.com/docs/divide-width
        ["divide", "x", "reverse"] => Ok("divide-x-reverse"),
        ["divide", "y", "reverse"] => Ok("divide-y-reverse"),
        ["divide", "x", ..] => Ok("divide-x"),
        ["divide", "y", ..] => Ok("divide-y"),

        // https://tailwindcss.com/docs/divide-color
        ["divide", ..] => Ok("divide-color"),

        // https://tailwindcss.com/docs/outline-style
        ["outline", "none"]
        | ["outline"]
        | ["outline", "dashed"]
        | ["outline", "dotted"]
        | ["outline", "double"]
        // necessary for "outline"
            if arbitrary.is_empty() =>
        {
            Ok("outline-style")
        }

        // https://tailwindcss.com/docs/outline-width
        ["outline"] if arbitrary.parse::<usize>().is_ok() => Ok("outline-width"),
        ["outline", rest] if rest.parse::<usize>().is_ok() => Ok("outline-width"),

        // https://tailwindcss.com/docs/outline-offset
        ["outline", "offset", ..] => Ok("outline-offset"),

        // https://tailwindcss.com/docs/outline-color
        ["outline", ..] => Ok("outline-color"),

        // https://tailwindcss.com/docs/outline-offset
        ["ring", "inset"] => Ok("ring-width"),
        ["ring", rest] if rest.parse::<usize>().is_ok() => Ok("ring-width"),
        ["ring"] if arbitrary.parse::<usize>().is_ok() => Ok("ring-color"),

        // https://tailwindcss.com/docs/ring-offset-width
        ["ring", "offset", rest] if rest.parse::<usize>().is_ok() => Ok("ring-offset-width"),
        ["ring", "offset"] if arbitrary.parse::<usize>().is_ok() => Ok("ring-offset-width"),

        // https://tailwindcss.com/docs/ring-offset-color
        ["ring", "offset", ..] => Ok("ring-offset-color"),


        // https://tailwindcss.com/docs/box-shadow
        // TODO: handle conflict with color + arbitrary
        ["shadow"] | ["shadow", "inner"] | ["shadow", "none"] if arbitrary.is_empty() => Ok("box-shadow"),
        ["shadow", size] if is_t_shirt_size(size) => Ok("box-shadow"),

        // https://tailwindcss.com/docs/box-shadow-color
        ["shadow", ..] => Ok("box-shadow-color"),

        // https://tailwindcss.com/docs/opacity
        ["opacity", ..] => Ok("opacity"),

        // https://tailwindcss.com/docs/mix-blend-mode
        ["mix", "blend", mode @ ..] if valid_blend(mode)=> Ok("mix-blend-mode"),

        // https://tailwindcss.com/docs/blur
        ["blur", ..] => Ok("blur"),

        // https: //tailwindcss.com/docs/brightness
        ["brightness", ..] => Ok("brightness"),

        // https://tailwindcss.com/docs/contrast
        ["contrast", ..] => Ok("contrast"),

        // https://tailwindcss.com/docs/drop-shadow
        ["drop", "shadow", ..] => Ok("drop-shadow"),

        // https://tailwindcss.com/docs/grayscale
        ["grayscale", ..] => Ok("grayscale"),

        // https://tailwindcss.com/docs/hue-rotate
        ["hue", "rotate", ..] => Ok("hue-rotate"),

        // https://tailwindcss.com/docs/invert
        ["invert", ..] => Ok("invert"),

        // https://tailwindcss.com/docs/saturate
        ["saturate", ..] => Ok("saturate"),

        // https://tailwindcss.com/docs/sepia
        ["sepia", ..] => Ok("sepia"),

        // https://tailwindcss.com/docs/backdrop-blur
        ["backdrop", "blur", ..] => Ok("backdrop-blur"),

        // https://tailwindcss.com/docs/backdrop-brightness
        ["backdrop", "brightness", ..] => Ok("backdrop-brightness"),

        // https://tailwindcss.com/docs/backdrop-contrast
        ["backdrop", "contrast", ..] => Ok("backdrop-contrast"),

        // https://tailwindcss.com/docs/backdrop-grayscale
        ["backdrop", "grayscale", ..] => Ok("backdrop-grayscale"),

        // https://tailwindcss.com/docs/backdrop-hue-rotate
        ["backdrop", "hue", "rotate", ..] => Ok("backdrop-hue-rotate"),

        // https://tailwindcss.com/docs/backdrop-invert
        ["backdrop", "invert", ..] => Ok("backdrop-invert"),

        // https://tailwindcss.com/docs/backdrop-opacity
        ["backdrop", "opacity", ..] => Ok("backdrop-opacity"),

        // https://tailwindcss.com/docs/backdrop-saturate
        ["backdrop", "saturate", ..] => Ok("backdrop-saturate"),

        // https://tailwindcss.com/docs/backdrop-sepia
        ["backdrop", "sepia", ..] => Ok("backdrop-sepia"),

        // https://tailwindcss.com/docs/table-layout
        ["table", "auto"] => Ok("table-layout"),
        ["table", "fixed"] => Ok("table-layout"),

        // https://tailwindcss.com/docs/caption-side
        ["caption", "top"] | ["caption", "bottom"] => Ok("caption-side"),

        // https://tailwindcss.com/docs/transition-property
        ["transition", ..] => Ok("transition-property"),

        // https://tailwindcss.com/docs/transition-duration
        ["duration", rest] if rest.parse::<usize>().is_ok() => Ok("transition-duration"),
        ["duration"] if arbitrary.parse::<usize>().is_ok() => Ok("transition-duration"),

        // https:// tailwindcss.com/docs/transition-timing-function
        ["ease", ..] => Ok("transition-timing-function"),

        // https://tailwindcss.com/docs/transition-delay
        ["delay", rest] if rest.parse::<usize>().is_ok() => Ok("transition-delay"),
        ["delay"] if arbitrary.parse::<usize>().is_ok() => Ok("transition-delay"),

        // https://tailwindcss.com/docs/animate
        ["animate", ..] => Ok("animate"),

        // https://tailwindcss.com/docs/scale
        ["scale", "x", rest] if rest.parse::<usize>().is_ok() => Ok("scale-x"),
        ["scale", "x"] if arbitrary.parse::<usize>().is_ok() => Ok("scale-x"),
        ["scale", "y", rest] if rest.parse::<usize>().is_ok() => Ok("scale-y"),
        ["scale", "y"] if arbitrary.parse::<usize>().is_ok() => Ok("scale-y"),
        ["scale", rest] if rest.parse::<usize>().is_ok() => Ok("scale"),
        // [1.75] is valid
        ["scale"] if arbitrary.parse::<f32>().is_ok() => Ok("scale"),

        // https://tailwindcss.com/docs/rotate
        ["rotate", rest] if rest.parse::<usize>().is_ok() => Ok("rotate"),
        ["rotate"] if arbitrary.parse::<usize>().is_ok() => Ok("rotate"),

        // https://tailwindcss.com/docs/translate
        ["translate", "x", ..]  => Ok("translate-x"),
        ["translate", "y", ..]  => Ok("translate-y"),

        // https://tailwindcss.com/docs/skew
        ["skew", "x", ..]  => Ok("skew-x"),
        ["skew", "y", ..]  => Ok("skew-y"),

        // https://tailwindcss.com/docs/transform-origin
        ["origin", ..] => Ok("transform-origin"),
        
        // https://tailwindcss.com/docs/accent-color
        ["accent", ..] => Ok("accent-color"),

        // https://tailwindcss.com/docs/appearance
        ["appearance", "none" | "auto"] => Ok("appearance"),

        // https://tailwindcss.com/docs/cursor
        ["cursor", ..] => Ok("cursor"),

        // https://tailwindcss.com/docs/caret-color
        ["caret", ..] => Ok("caret-color"),

        // https://tailwindcss.com/docs/pointer-events
        ["pointer", "none"] | ["pointer", "auto"] => Ok("pointer-events"),

        // https://tailwindcss.com/docs/resize
        ["resize", "none"] | ["resize", "y"] | ["resize", "x"] | ["resize"] => Ok("resize"),

        // https://tailwindcss.com/docs/scroll-behavior
        ["scroll", "auto"] | ["scroll", "smooth"] => Ok("scroll-behavior"),

        // https://tailwindcss.com/docs/scroll-margin
        ["scroll", rest, ..] if rest.starts_with('m') => Ok("scroll-margin"),

        // https://tailwindcss.com/docs/scroll-padding
        ["scroll", rest, ..] if rest.starts_with('p') => Ok("scroll-padding"),

        // https://tailwindcss.com/docs/scroll-snap-align
        ["snap", "start"] | ["snap", "end"] | ["snap", "center"] | ["snap", "align", "none"] => {
            Ok("scroll-snap-align")
        }

        // https://tailwindcss.com/docs/scroll-snap-stop#forcing-snap-position-stops
        ["snap", "normal"] | ["snap", "always"] => Ok("scroll-snap-stop"),

        // https://tailwindcss.com/docs/scroll-snap-type
        ["snap", "none"] | ["snap", "x"] | ["snap", "y"] | ["snap", "both"] | ["snap", "mandatory"] | ["snap", "proximity"] => {
            Ok("scroll-snap-type")
        }

        // https://tailwindcss.com/docs/touch-action
        ["touch", "auto"] | ["touch", "none"] 
        | ["touch", "pan", "x"] | ["touch", "pan", "left"] | ["touch", "pan", "right"]
        | ["touch", "pan", "y"] | ["touch", "pan", "up"] | ["touch", "pan", "down"]
        | ["touch", "pinch", "zoom"] | ["touch", "manipulation"] => {
            Ok("touch-action")
        }

        // https://tailwindcss.com/docs/user-select
        ["select", "none"] | ["select", "text"] | ["select", "all"] | ["select", "auto"] => Ok("user-select"),

        // https://tailwindcss.com/docs/will-change
        ["will", "change", ..] => Ok("will-change"),

        // https://tailwindcss.com/docs/fill
        ["fill", ..] => Ok("fill"),

        // https://tailwindcss.com/docs/stroke-width
        ["stroke", rest] if rest.parse::<usize>().is_ok() => Ok("stroke-width"),
        ["stroke"] if is_valid_length(arbitrary) => Ok("stroke-width"),

        // https://tailwindcss.com/docs/stroke
        ["stroke", ..] => Ok("stroke"),

        // https://tailwindcss.com/docs/screen-readers
        ["sr", "only"] | ["not", "sr", "only"] => Ok("screen-readers"),

        // https://tailwindcss.com/docs/forced-color-adjust
        ["forced", "color", "adjust", "auto" | "none"] => Ok("forced-color-adjust"),

        _ => Err("Invalid tailwind class"),
    }
}

fn valid_blend(mode: &[&str]) -> bool {
    matches!(mode, ["normal"] | ["multiply"] | ["screen"] | ["overlay"] | ["darken"] | ["lighten"] | ["color", "dodge"] | ["color", "burn"] | ["hard", "light"] | ["soft", "light"] | ["difference"] | ["exclusion"] | ["hue"] | ["saturation"] | ["color"] | ["luminosity"] | ["lighter"])
}

fn valid_trbl(
    mode: &[&str],
    arbitrary: &str,
    success: &'static str,
    error: &'static str,
) -> Result<&'static str> {
    if mode.len() == 1 && valid_top_right_bottom_left(mode[0]) {
        return Ok(success);
    }
    if valid_top_right_bottom_left(arbitrary) {
        return Ok(success);
    }

    Err(error)
}

fn valid_top_right_bottom_left(mode: &str) -> bool {
    parse_fraction(mode).is_some()
        || mode == "auto"
        || mode == "px"
        || mode == "full"
        || parse_single_digit_decimal(mode).is_some()
}

fn valid_break_after(mode: &str) -> bool {
    matches!(
        mode,
        "auto" | "avoid" | "all" | "avoid-page" | "page" | "left" | "right" | "column"
    )
}

fn valid_order(mode: &str) -> bool {
    mode == "first" || mode == "last" || mode == "none" || mode.parse::<isize>().is_ok()
}

fn valid_text_size(mode: &str) -> bool {
    mode == "base"
        || mode.ends_with("xs")
        || mode.ends_with("sm")
        || mode.ends_with("md")
        || mode.ends_with("lg")
        || mode.ends_with("xl")
}

// parses 1.5 but not 1.55
fn parse_single_digit_decimal(input: &str) -> Option<()> {
    if input.len() == 3 {
        let (l, r) = input.split_once('.')?;
        let _ = l.parse::<usize>().ok()?;
        let _ = r.parse::<usize>().ok()?;
        Some(())
    } else {
        None
    }
}

fn parse_fraction_or_usize(input: &str) -> bool {
    parse_fraction(input)
        .map(|_| ())
        .or_else(|| input.parse::<usize>().ok().map(|_| ()))
        .is_some()
}

fn parse_fraction(input: &str) -> Option<(usize, usize)> {
    let (a, b) = input.split_once('/')?;
    let a = a.parse::<usize>().ok()?;
    let b = b.parse::<usize>().ok()?;
    Some((a, b))
}

fn is_t_shirt_size(input: &str) -> bool {
        input.ends_with("xs")
        || input.ends_with("sm")
        || input.ends_with("md")
        || input.ends_with("lg")
        || input.ends_with("xl")
}


// Define LENGTH_REGEX using lazy_static
lazy_static::lazy_static! {
    static ref LENGTH_REGEX: Regex = Regex::new(r"\d+(%|px|r?em|[sdl]?v([hwib]|min|max)|pt|pc|in|cm|mm|cap|ch|ex|r?lh|cq(w|h|i|b|min|max))|\b(calc|min|max|clamp)\(.+\)|^0$").unwrap();
    static ref COLOR_REGEX: Regex = Regex::new(r"^(rgba?|hsla?|hwb|(ok)?(lab|lch))\(.+\)$").unwrap();
}

fn is_valid_length(input: &str) -> bool {
    LENGTH_REGEX.is_match(input)
    && !is_valid_color(input)
}

fn is_valid_color(input: &str) -> bool {
    COLOR_REGEX.is_match(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test] 
    fn test_len() {
        assert!(is_valid_length("10px"));
        assert!(is_valid_length("10%"));
        assert!(is_valid_length("100rem"));

        assert!(!is_valid_length("10"), "needs unit");
        assert!(!is_valid_length("hsl(350_80%_0%)"), "no color");
    }

    #[test]
    fn parse_stroke() {
        let result = parse(&["stroke"], "10px");
        assert_eq!(result, Ok("stroke-width"));

        let result = parse(&["stroke"], "hsl(350_80%_0%)");
        assert_eq!(result, Ok("stroke"));
    }

    #[test]
    fn parse_margin() {
        let result = parse(&["my", "2"], "");
        assert_eq!(result, Ok("margin-y"));

        let result = parse(&["m", "2"], "");
        assert_eq!(result, Ok("margin"));

        let result = parse(&["m"], "2px");
        assert_eq!(result, Ok("margin"));

        let result = parse(&["my"], "10rem");
        assert_eq!(result, Ok("margin-y"));
    }
}