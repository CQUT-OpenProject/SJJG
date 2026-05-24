use ex_2::build_sample_graph;

pub fn bfs_search(start: i32) -> Vec<i32> {
    // 进阶练习二：复用基础练习二建立的邻接表。
    let graph = build_sample_graph();

    // 使用广度优先搜索，按队列先进先出的顺序访问顶点。
    graph.bfs(start)
}
