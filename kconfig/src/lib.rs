#![feature(option_result_contains)]
#[deny(unused_imports)]

pub mod parser;
pub mod structure;
pub mod errors;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
