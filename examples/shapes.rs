use bodymovin::Bodymovin;
use simple_logger::SimpleLogger;
use std::path::Path;

fn main() {
    SimpleLogger::new().init().unwrap();
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/shapes.json");
    let loaded = Bodymovin::load(path).unwrap();
    println!("{:#?}", loaded);
}
