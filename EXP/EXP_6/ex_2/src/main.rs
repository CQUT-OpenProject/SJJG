use ex_2::{build_edge_sample_graph, build_sample_graph};

fn main() {
    let mut graph = build_sample_graph();

    println!("图 10-8 的邻接表:");
    print!("{}", graph.output());

    graph.insert_line(2, 5);
    println!("加入边 2 -> 5 后:");
    print!("{}", graph.output());

    graph.delete_line(2, 5);
    println!("删除边 2 -> 5 后:");
    print!("{}", graph.output());

    println!("从 1 开始广度优先搜索:");
    println!("{:?}", graph.bfs(1));

    println!("图 10-8 的拓扑排序:");
    println!("{:?}", graph.topo_sort());

    let edge_graph = build_edge_sample_graph();
    println!("Kruskal 最小生成树:");
    for e in edge_graph.kruskal() {
        println!("{} - {} : {}", e.from, e.to, e.weight);
    }
}
