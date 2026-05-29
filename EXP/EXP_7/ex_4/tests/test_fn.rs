use ex_4::{bin_insert_pos, insert_ordered, sample_data};

#[test]
fn bin_insert_pos_finds_position() {
    let data = sample_data();

    assert_eq!(bin_insert_pos(&data, 1), 0);
    assert_eq!(bin_insert_pos(&data, 20), 5);
    assert_eq!(bin_insert_pos(&data, 50), 10);
}

#[test]
fn insert_ordered_keeps_table_ordered() {
    let mut data = sample_data();
    let pos = insert_ordered(&mut data, 20);

    assert_eq!(pos, 5);
    assert_eq!(data, vec![3, 8, 12, 15, 18, 20, 21, 25, 30, 36, 42]);
}

#[test]
fn insert_ordered_handles_head_and_tail() {
    let mut data = sample_data();

    assert_eq!(insert_ordered(&mut data, 1), 0);
    assert_eq!(insert_ordered(&mut data, 50), 11);
    assert_eq!(data, vec![1, 3, 8, 12, 15, 18, 21, 25, 30, 36, 42, 50]);
}
