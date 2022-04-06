use std::time::Instant;
use crate::parser::kconfig_parser::KconfigParser;

#[test]
pub fn basic() {
    let basic_src = include_str!("Kconfig.basic");

    let time = Instant::now();
    let kconfig = KconfigParser::new()
        .allow_sourcing(false)
        .set_kconfig_source(basic_src.to_string())
        .parse()
        .take_result().unwrap();

    // LMAO
    println!("Childrens: {:#?}", kconfig.children.children.len());
    println!("Took {} ms for basic", time.elapsed().as_millis())
}