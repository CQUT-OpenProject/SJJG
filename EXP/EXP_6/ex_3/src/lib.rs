use ex_1::build_sample_graph;

pub fn dfs_search(start: i32) -> Vec<i32> {
    // 进阶练习一：复用基础练习一建立的邻接矩阵。
    let graph = build_sample_graph();

    // 从给定顶点开始深度优先搜索，返回访问到的顶点序列。
    graph.dfs(start)
}
