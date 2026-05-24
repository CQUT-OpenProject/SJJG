use ex_2::build_sample_graph;

pub fn topo_sort_result() -> Vec<i32> {
    // 扩展练习一：对图 10-8 的邻接表进行拓扑排序。
    let graph = build_sample_graph();

    // 返回一种合法的拓扑序列。
    graph.topo_sort()
}
