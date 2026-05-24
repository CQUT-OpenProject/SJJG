use ex_3::dfs_search;

fn main() {
    // 从 0 号顶点开始，也可以改成其它顶点观察搜索序列变化。
    let result = dfs_search(0);

    println!("进阶练习一：邻接矩阵深度优先搜索序列");
    println!("{:?}", result);
}
