use std::process;

use exp_6::{find_top_k, generate_sample_data, read_edges};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("用法: {} <数据文件路径> [top K 数量]", args[0]);
        eprintln!("  数据文件格式: time1 time2 from_id to_id");
        eprintln!("  默认 K = 10");
        eprintln!("  附加选项: --generate 先生成测试数据再分析");
        process::exit(1);
    }

    let input_path = &args[1];

    let generate = args.iter().any(|a| a == "--generate");

    let k: usize = args
        .iter()
        .skip(2)
        .find(|a| a.parse::<usize>().is_ok())
        .and_then(|a| a.parse().ok())
        .unwrap_or(10);

    if generate {
        generate_sample_data(input_path).unwrap_or_else(|err| {
            eprintln!("生成数据失败: {}", err);
            process::exit(1);
        });
        println!("测试数据已生成: {}\n", input_path);
    }

    let graph = read_edges(input_path).unwrap_or_else(|err| {
        eprintln!("无法读取数据文件 {}: {}", input_path, err);
        process::exit(1);
    });

    let in_degrees = graph.in_degrees();

    println!("=== 社交网络分析 ===");
    println!("数据文件: {}", input_path);
    println!("顶点数: {}", graph.vertex_count());
    println!("边数: {}", graph.edge_count());

    let max_deg = in_degrees.iter().max().copied().unwrap_or(0);
    let min_deg = in_degrees.iter().min().copied().unwrap_or(0);
    println!("入度范围: {} ~ {}", min_deg, max_deg);

    let top_k = find_top_k(&graph, k);

    println!();
    println!("=== 入度排名前 {} 的大 V 结点 ===", k);
    println!(
        "{:<8} {:<10}",
        "结点ID", "入度"
    );
    println!("{}", "-".repeat(20));

    for r in &top_k {
        println!("{:<8} {:<10}", r.vertex, r.in_degree);
    }

    if top_k.len() < k {
        println!();
        println!("（注意：网络仅有 {} 个顶点，不足 {} 个）", graph.vertex_count(), k);
    }
}
