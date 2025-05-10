use super::*;

#[test]
fn test_graphemes_in_range_safe() {
    use std::panic::{self, PanicHookInfo};

    let s = "😀🥹🤣😇";

    // Test length.
    assert_eq!(grapheme_length(s), 4);
    assert_eq!(s.len(), 16);

    // Test slicing with (at least .chars()) graphemes is needed to prevent panics.

    // Save the original panic hook
    let original_hook = panic::take_hook();

    // Set a no-op hook to suppress output
    panic::set_hook(Box::new(|_info: &PanicHookInfo| {
        // Do nothing
    }));
    let result = std::panic::catch_unwind(|| {
        let _crash = &s[0..1];
    });

    // Restore the original hook
    panic::set_hook(original_hook);

    assert!(result.is_err());

    // Test happy-path cases.
    assert_eq!(graphemes_in_range_safe(s, None, None), "😀🥹🤣😇");
    assert_eq!(graphemes_in_range_safe(s, Some(0), Some(5)), "😀🥹🤣😇");
    assert_eq!(graphemes_in_range_safe(s, Some(1), Some(4)), "🥹🤣😇");
    assert_eq!(graphemes_in_range_safe(s, Some(1), Some(3)), "🥹🤣");
    assert_eq!(graphemes_in_range_safe(s, Some(1), Some(2)), "🥹");
    assert_eq!(graphemes_in_range_safe(s, Some(1), Some(1)), "");
    assert_eq!(graphemes_in_range_safe(s, Some(0), Some(0)), "");
    assert_eq!(graphemes_in_range_safe(s, None, Some(2)), "😀🥹");
    assert_eq!(graphemes_in_range_safe(s, Some(2), None), "🤣😇");

    // Test safety (out of bounds, wrong indices).
    assert_eq!(graphemes_in_range_safe(s, Some(10), None), "");
    assert_eq!(graphemes_in_range_safe(s, Some(3), Some(2)), "");
    assert_eq!(graphemes_in_range_safe(s, Some(10), Some(5)), "");
    assert_eq!(graphemes_in_range_safe(s, Some(0), Some(10)), "😀🥹🤣😇");
    assert_eq!(graphemes_in_range_safe(s, Some(10), Some(20)), "");
}
