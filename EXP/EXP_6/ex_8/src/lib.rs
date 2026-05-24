use ex_2::{Edge, build_edge_sample_graph};

pub fn kruskal_result() -> Vec<Edge> {
    // 附加题二：在边集数组上执行 Kruskal 算法。
    let graph = build_edge_sample_graph();

    // 返回按边权逐步选出的最小生成树边集合。
    graph.kruskal()
}

pub fn total_weight(edges: &[Edge]) -> i32 {
    let mut sum = 0;

    // 把选中的边权累加，便于检查最小生成树总权值。
    for e in edges {
        sum += e.weight;
    }

    sum
}
