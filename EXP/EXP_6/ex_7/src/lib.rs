use ex_1::build_weighted_sample_graph;

pub fn prim_result(start: i32) -> Vec<(i32, i32, i32)> {
    // 附加题一：在带权邻接矩阵上执行 Prim 算法。
    let graph = build_weighted_sample_graph();

    // 返回的每个三元组表示：起点、终点、边权。
    graph.prim(start)
}

pub fn total_weight(edges: &[(i32, i32, i32)]) -> i32 {
    let mut sum = 0;

    // 把最小生成树中的所有边权相加。
    for item in edges {
        sum += item.2;
    }

    sum
}
