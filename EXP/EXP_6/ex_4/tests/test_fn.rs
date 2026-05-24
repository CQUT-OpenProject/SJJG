use ex_4::bfs_search;

#[test]
fn bfs_from_list_visits_all_vexs() {
    // 邻接表使用头插法，该序列与当前边链表顺序对应。
    assert_eq!(bfs_search(1), vec![1, 4, 3, 2, 5, 6]);
}
