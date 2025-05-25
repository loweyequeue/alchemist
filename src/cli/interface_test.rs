use super::*;

const S: &str = "😀🥹🤣😇";

#[test]
#[should_panic(
    expected = "byte index 1 is not a char boundary; it is inside '😀' (bytes 0..4) of `😀🥹🤣😇`"
)]
fn test_grapheme_length_panic() {
    // Test slicing with (at least .chars()) graphemes is needed to prevent panics.
    let _crash = &S[0..1];
}

#[test]
fn test_graphemes_in_range_safe() {
    // Test length.
    assert_eq!(grapheme_length(S), 4);
    assert_eq!(S.len(), 16);

    // Test happy-path cases.
    assert_eq!(graphemes_in_range_safe(S, None, None), "😀🥹🤣😇");
    assert_eq!(graphemes_in_range_safe(S, Some(0), Some(5)), "😀🥹🤣😇");
    assert_eq!(graphemes_in_range_safe(S, Some(1), Some(4)), "🥹🤣😇");
    assert_eq!(graphemes_in_range_safe(S, Some(1), Some(3)), "🥹🤣");
    assert_eq!(graphemes_in_range_safe(S, Some(1), Some(2)), "🥹");
    assert_eq!(graphemes_in_range_safe(S, Some(1), Some(1)), "");
    assert_eq!(graphemes_in_range_safe(S, Some(0), Some(0)), "");
    assert_eq!(graphemes_in_range_safe(S, None, Some(2)), "😀🥹");
    assert_eq!(graphemes_in_range_safe(S, Some(2), None), "🤣😇");

    // Test safety (out of bounds, wrong indices).
    assert_eq!(graphemes_in_range_safe(S, Some(10), None), "");
    assert_eq!(graphemes_in_range_safe(S, Some(3), Some(2)), "");
    assert_eq!(graphemes_in_range_safe(S, Some(10), Some(5)), "");
    assert_eq!(graphemes_in_range_safe(S, Some(0), Some(10)), "😀🥹🤣😇");
    assert_eq!(graphemes_in_range_safe(S, Some(10), Some(20)), "");
}

#[test]
fn test_grapheme_length_colored() {
    assert_eq!(grapheme_length_colored(S), 4, "grapheme length");
    assert_eq!("a".red().to_string().len(), 11, "colored string length");
    assert_eq!(grapheme_length_colored("a".red()), 1, "ONE");
    assert_eq!(grapheme_length_colored("a".green()), 1);
    assert_eq!(grapheme_length_colored("ab".blue()), 2);
}
