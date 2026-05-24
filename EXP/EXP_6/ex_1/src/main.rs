use ex_1::{build_sample_graph, build_weighted_sample_graph};

fn main() {
    let mut graph = build_sample_graph();

    println!("图 10-7 的邻接矩阵:");
    print!("{}", graph.output());

    graph.insert_line(2, 4);
    println!("加入边 2-4 后:");
    print!("{}", graph.output());

    graph.delete_line(2, 4);
    println!("删除边 2-4 后:");
    print!("{}", graph.output());

    println!("从 0 开始深度优先搜索:");
    println!("{:?}", graph.dfs(0));

    println!("邻接矩阵转换成邻接表:");
    print!("{}", graph.to_list_graph().output());

    let weighted_graph = build_weighted_sample_graph();
    println!("PRIM 最小生成树:");
    for (from, to, weight) in weighted_graph.prim(0) {
        println!("{} - {} : {}", from, to, weight);
    }
}
