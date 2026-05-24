use ex_8::{kruskal_result, total_weight};

#[test]
fn kruskal_result_gets_min_tree() {
    let result = kruskal_result();

    // 8 个顶点的最小生成树应有 7 条边。
    assert_eq!(result.len(), 7);
    assert_eq!(total_weight(&result), 25);
}
