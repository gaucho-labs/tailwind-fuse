use tw_fuse::tw_merge_with_options;

pub fn tw_merge(class: &str) -> String {
    tw_merge_with_options(
        Default::default(),
        collision_id_fn,
        get_collisions,
        &[class],
    )
}

pub fn collision_id_fn(elements: &[&str], _: Option<&str>) -> Option<&'static str> {
    match elements {
        ["bg", "red", _] => Some("america"),
        ["bg", "white"] => Some("america"),
        ["bg", "blue", _] => Some("america"),
        ["flex"] => Some("florida"),
        _ => None,
    }
}

pub fn get_collisions(collision_id: &str) -> Option<Vec<&'static str>> {
    match collision_id {
        "florida" => Some(vec!["america"]),
        _ => None,
    }
}

#[test]
fn test_collisions() {
    let class = "bg-red-500 bg-black";
    let result = tw_merge(class);
    assert_eq!(class, result);

    let result = tw_merge("bg-white bg-red-500");
    assert_eq!("bg-red-500", result);

    let result = tw_merge("bg-white flex");
    assert_eq!("flex", result);
}
