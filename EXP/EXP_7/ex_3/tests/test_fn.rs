use ex_3::{IndexNode, block_search, sample_data, sample_index};

#[test]
fn index_table_is_built_by_three_blocks() {
    let index = sample_index();

    assert_eq!(
        index,
        vec![
            IndexNode {
                start: 0,
                max_key: 22
            },
            IndexNode {
                start: 6,
                max_key: 48
            },
            IndexNode {
                start: 12,
                max_key: 86
            },
        ]
    );
}

#[test]
fn block_search_finds_existing_data() {
    let data = sample_data();
    let index = sample_index();

    assert_eq!(block_search(&data, &index, 6, 8), 3);
    assert_eq!(block_search(&data, &index, 6, 24), 10);
    assert_eq!(block_search(&data, &index, 6, 53), 17);
}

#[test]
fn block_search_returns_minus_one_when_missing() {
    let data = sample_data();
    let index = sample_index();

    assert_eq!(block_search(&data, &index, 6, 21), -1);
    assert_eq!(block_search(&data, &index, 6, 100), -1);
}
