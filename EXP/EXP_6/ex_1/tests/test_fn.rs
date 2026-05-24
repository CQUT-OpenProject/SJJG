use ex_1::{build_sample_graph, build_weighted_sample_graph};

#[test]
fn sample_graph_has_required_lines() {
    let graph = build_sample_graph();

    assert!(graph.has_line(0, 1));
    assert!(graph.has_line(1, 0));
    assert!(graph.has_line(5, 7));
    assert!(!graph.has_line(0, 7));
}

#[test]
fn insert_and_delete_line_work() {
    let mut graph = build_sample_graph();

    assert!(!graph.has_line(2, 4));
    assert!(graph.insert_line(2, 4));
    assert!(graph.has_line(2, 4));
    assert!(graph.has_line(4, 2));

    assert!(graph.delete_line(2, 4));
    assert!(!graph.has_line(2, 4));
}

#[test]
fn dfs_visits_all_vexs() {
    let graph = build_sample_graph();

    assert_eq!(graph.dfs(0), vec![0, 1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn matrix_can_be_changed_to_list_graph() {
    let graph = build_sample_graph();
    let list = graph.to_list_graph();
    let text = list.output();

    assert!(text.contains("0: -> 1 -> 5"));
    assert!(text.contains("7: -> 5 -> 6"));
}

#[test]
fn prim_gets_min_tree() {
    let graph = build_weighted_sample_graph();
    let result = graph.prim(0);
    let mut sum = 0;

    for item in &result {
        sum += item.2;
    }

    assert_eq!(result.len(), 7);
    assert_eq!(sum, 25);
}
