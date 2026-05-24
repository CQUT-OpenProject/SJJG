use ex_1::build_sample_graph;

pub fn matrix_to_list_text() -> String {
    // 进阶练习三：先建立图 10-7 的邻接矩阵。
    let graph = build_sample_graph();

    // 调用转换函数，把邻接矩阵改成邻接表后再输出。
    graph.to_list_graph().output()
}
