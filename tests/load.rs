use bodymovin::Bodymovin;
use std::path::Path;

fn test_file(file: &str) {
    let _ = env_logger::builder().is_test(true).try_init();
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data")
        .join(file);
    let loaded = Bodymovin::load(path).unwrap();
    insta::assert_debug_snapshot!(loaded);
}

#[test]
fn test_shapes() {
    test_file("shapes.json")
}

#[test]
fn test_4thofjuly2018() {
    test_file("4thofjuly2018.json")
}

#[test]
fn test_lottie_adrock() {
    test_file("lottie_adrock.json")
}

#[test]
fn test_lottie_bodymovin() {
    test_file("lottie_bodymovin.json")
}

#[test]
fn test_lottie_gatin() {
    test_file("lottie_gatin.json")
}

#[test]
fn test_lottie_happy2016() {
    test_file("lottie_happy2016.json")
}

#[test]
fn test_underline() {
    test_file("underline.json")
}
