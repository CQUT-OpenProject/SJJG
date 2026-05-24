use ex_7::{prim_result, total_weight};

fn main() {
    let result = prim_result(0);

    println!("附加题一：Prim 最小生成树");
    for (from, to, weight) in &result {
        println!("{} - {} : {}", from, to, weight);
    }
    println!("总权值: {}", total_weight(&result));
}
