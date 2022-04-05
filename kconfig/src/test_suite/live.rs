use std::path::Path;
use std::time::Instant;
use crate::parser::kconfig_parser::KconfigParser;

#[test]
pub fn live() {
    let time = Instant::now();
    let kconfig = KconfigParser::new()
        .set_kconfig_path(Path::new("/home/barnabas/Projects/kconfig_rs/linux-3.6.8/Kconfig"))
        .unwrap()
        .set_variable("SRCARCH", "arm")
        .allow_sourcing(true)
        .parse()
        .take_result().unwrap();

    // LMAO
    let el = time.elapsed().as_millis();
    let size = format!("{:?}", kconfig).len();
    println!("Took {} ms, debug printed: {}", el, size)
}