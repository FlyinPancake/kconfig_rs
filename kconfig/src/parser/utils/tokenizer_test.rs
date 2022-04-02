use crate::parser::utils::tokenizer::LineKConfigTokenizerIterator;

#[test]
fn happy_path_with_string_works() {
    let mut iter = LineKConfigTokenizerIterator::from_line("menu \"Hello World!\"");

    assert_eq!(iter.next(), Some("menu"));
    assert_eq!(iter.next(), Some("\"Hello World!\""));
    assert_eq!(iter.next(), None);
}

#[test]
fn happy_path_two() {
    let mut iter = LineKConfigTokenizerIterator::from_line(
        "kcon_fig_stug   \t\t\t     \"Hello \t  World!\" kekwow   ",
    );

    assert_eq!(iter.next(), Some("kcon_fig_stug"));
    assert_eq!(iter.next(), Some("\"Hello \t  World!\""));
    assert_eq!(iter.next(), Some("kekwow"));
    assert_eq!(iter.next(), None);
}
