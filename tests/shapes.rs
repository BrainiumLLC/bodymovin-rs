use bodymovin::Bodymovin;
use std::path::Path;

#[test]
fn test_shapes() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/shapes.json");
    let loaded = Bodymovin::load(path).unwrap();
    insta::assert_debug_snapshot!(loaded);
}
