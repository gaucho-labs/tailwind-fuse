(* ::Package:: *)

pointer = "
/* Keyword values */
font-style: normal;
font-style: italic;
font-style: oblique;

font-style: oblique 10deg;

/* Global values */
font-style: inherit;
font-style: initial;
font-style: revert;
font-style: unset;
";

all = Sort@StringCases[
    pointer,
    RegularExpression[":\\s*([a-zA-Z0-9-]+);"] :> "$1"
];
"[\"" <> StringRiffle[all, "\",\""] <> "\"].contains(&mode)" // CopyToClipboard
