#![feature(option_result_contains)]

pub mod errors;
#[deny(unused_imports)]
pub mod parser;
pub mod structure;

#[cfg(test)]
pub mod test_utils;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
