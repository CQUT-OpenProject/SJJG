use ex_4::bfs_search;

fn main() {
    // 图 10-8 不是强连通图，函数内部会继续访问未遍历过的顶点。
    let result = bfs_search(1);

    println!("进阶练习二：邻接表广度优先搜索序列");
    println!("{:?}", result);
}
