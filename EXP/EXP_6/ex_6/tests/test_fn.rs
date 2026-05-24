use ex_6::topo_sort_result;

#[test]
fn topo_sort_result_is_valid() {
    let result = topo_sort_result();

    // 结果中应包含 6 个顶点，并满足几条关键有向边的先后关系。
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
