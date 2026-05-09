use string_ex_4::delete_substring;

#[test]
fn deletes_all_substrings() {
    assert_eq!(delete_substring("abcxxabcxxabc", "abc"), "xxxx");
}

#[test]
fn handles_overlapping_after_each_delete() {
    assert_eq!(delete_substring("aaaa", "aa"), "");
}

#[test]
fn empty_pattern_keeps_original_string() {
    assert_eq!(delete_substring("abc", ""), "abc");
}
