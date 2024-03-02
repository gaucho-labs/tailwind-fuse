#[derive(Clone, Debug)]
pub struct TailwindSnapStop {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindSnapStop => "scroll-snap-stop", ["normal", "always"]);
