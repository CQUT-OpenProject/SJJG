use ex_3::dfs_search;

#[test]
fn dfs_from_matrix_visits_all_vexs() {
    // 图 10-7 是连通图，从 0 出发应能访问全部 8 个顶点。
    assert_eq!(dfs_search(0), vec![0, 1, 2, 3, 4, 5, 6, 7]);
}
