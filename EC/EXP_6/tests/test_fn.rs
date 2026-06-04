use exp_6::{find_top_k, generate_sample_data, read_edges, DiGraph};

#[test]
fn build_simple_graph() {
    let mut g = DiGraph::new(5);
    g.add_edge(0, 1);
    g.add_edge(0, 2);
    g.add_edge(1, 2);
    g.add_edge(2, 3);
    g.add_edge(3, 0);

    assert_eq!(g.vertex_count(), 5);
    assert_eq!(g.edge_count(), 5);
}

#[test]
fn in_degree_calculation() {
    let mut g = DiGraph::new(4);
    g.add_edge(0, 1);
    g.add_edge(2, 1);
    g.add_edge(3, 1);
    g.add_edge(1, 2);

    let degrees = g.in_degrees();
    assert_eq!(degrees[0], 0);
    assert_eq!(degrees[1], 3);
    assert_eq!(degrees[2], 1);
    assert_eq!(degrees[3], 0);
}

#[test]
fn out_degree_calculation() {
    let mut g = DiGraph::new(4);
    g.add_edge(0, 1);
    g.add_edge(0, 2);
    g.add_edge(0, 3);
    g.add_edge(2, 1);

    let deg = g.out_degrees();
    assert_eq!(deg[0], 3);
    assert_eq!(deg[1], 0);
    assert_eq!(deg[2], 1);
    assert_eq!(deg[3], 0);
}

#[test]
fn find_top_k_basic() {
    let mut g = DiGraph::new(5);
    g.add_edge(0, 4);
    g.add_edge(1, 4);
    g.add_edge(2, 4);
    g.add_edge(3, 4);
    g.add_edge(0, 3);
    g.add_edge(1, 3);

    let top = find_top_k(&g, 3);
    assert_eq!(top.len(), 3);
    assert_eq!(top[0].vertex, 4);
    assert_eq!(top[0].in_degree, 4);
    assert_eq!(top[1].vertex, 3);
    assert_eq!(top[1].in_degree, 2);
}

#[test]
fn find_top_k_more_than_vertices() {
    let mut g = DiGraph::new(3);
    g.add_edge(0, 1);
    g.add_edge(0, 2);

    let top = find_top_k(&g, 10);
    assert_eq!(top.len(), 3);
}

#[test]
fn empty_graph() {
    let g = DiGraph::new(5);
    assert_eq!(g.edge_count(), 0);
    let deg = g.in_degrees();
    assert!(deg.iter().all(|&d| d == 0));
}

#[test]
fn generate_and_read_roundtrip() {
    let tmp = "/tmp/test_social_network.txt";
    generate_sample_data(tmp).unwrap();

    let graph = read_edges(tmp).unwrap();
    assert!(graph.vertex_count() > 0);
    assert!(graph.edge_count() > 0);

    let top = find_top_k(&graph, 10);
    assert!(!top.is_empty());

    for r in &top {
        assert!(r.in_degree > 0);
    }

    std::fs::remove_file(tmp).ok();
}

#[test]
fn all_degrees_sum_equal_edges() {
    let mut g = DiGraph::new(4);
    g.add_edge(0, 1);
    g.add_edge(0, 2);
    g.add_edge(1, 3);
    g.add_edge(2, 3);

    let in_sum: usize = g.in_degrees().iter().sum();
    let out_sum: usize = g.out_degrees().iter().sum();
    assert_eq!(in_sum, out_sum);
    assert_eq!(in_sum, g.edge_count());
}
