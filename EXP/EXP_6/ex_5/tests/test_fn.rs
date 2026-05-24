use ex_5::matrix_to_list_text;

#[test]
fn matrix_has_been_changed_to_list() {
    let text = matrix_to_list_text();

    // 顶点 0 与 1、5 相邻，转换后邻接表中应能看到这两个邻接点。
    assert!(text.contains("0: -> 1 -> 5"));
    assert!(text.contains("7: -> 5 -> 6"));
}
