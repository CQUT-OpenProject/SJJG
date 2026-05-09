use string_ex_3::{CommonResult, max_common_substring};

#[test]
fn finds_longest_common_substring() {
    let result = max_common_substring("student", "deskstudy");

    assert_eq!(
        result,
        CommonResult {
            max_len: 4,
            pos1: 0,
            pos2: 4,
            text: "stud".to_string()
        }
    );
}

#[test]
fn no_common_substring_has_zero_length() {
    let result = max_common_substring("abc", "XYZ");

    assert_eq!(result.max_len, 0);
    assert_eq!(result.pos1, 0);
    assert_eq!(result.pos2, 0);
    assert_eq!(result.text, "");
}
