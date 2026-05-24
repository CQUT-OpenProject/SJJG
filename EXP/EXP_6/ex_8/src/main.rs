use ex_8::{kruskal_result, total_weight};

fn main() {
    let result = kruskal_result();

    println!("附加题二：Kruskal 最小生成树");
    for e in &result {
        println!("{} - {} : {}", e.from, e.to, e.weight);
    }
    println!("总权值: {}", total_weight(&result));
}
