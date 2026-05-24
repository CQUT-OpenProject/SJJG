use ex_2::{build_edge_sample_graph, build_sample_graph};

#[test]
fn sample_graph_has_some_required_lines() {
    let graph = build_sample_graph();

    assert!(graph.has_line(1, 2));
    assert!(graph.has_line(1, 4));
    assert!(graph.has_line(6, 5));
    assert!(!graph.has_line(2, 1));
}

#[test]
fn insert_line_has_been_finished() {
    let mut graph = build_sample_graph();

    assert!(!graph.has_line(2, 5));
    assert!(graph.insert_line(2, 5));
    assert!(graph.has_line(2, 5));
}

#[test]
fn delete_line_works() {
    let mut graph = build_sample_graph();

    assert!(graph.has_line(1, 2));
    assert!(graph.delete_line(1, 2));
    assert!(!graph.has_line(1, 2));
    assert!(!graph.delete_line(1, 2));
}

#[test]
fn bfs_visits_all_vexs() {
    let graph = build_sample_graph();

    assert_eq!(graph.bfs(1), vec![1, 4, 3, 2, 5, 6]);
}

#[test]
fn topo_sort_visits_all_vexs() {
    let graph = build_sample_graph();
    let result = graph.topo_sort();

    assert_eq!(result.len(), 6);
    assert!(
        result.iter().position(|x| *x == 1).unwrap() < result.iter().position(|x| *x == 2).unwrap()
    );
    assert!(
        result.iter().position(|x| *x == 3).unwrap() < result.iter().position(|x| *x == 5).unwrap()
    );
    assert!(
        result.iter().position(|x| *x == 6).unwrap() < result.iter().position(|x| *x == 4).unwrap()
    );
}

#[test]
fn kruskal_gets_min_tree() {
    let graph = build_edge_sample_graph();
    let result = graph.kruskal();
    let mut sum = 0;

    for e in &result {
        sum += e.weight;
    }

    assert_eq!(result.len(), 7);
    assert_eq!(sum, 25);
}
