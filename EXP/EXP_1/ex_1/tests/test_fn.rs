use ex_1::fun;

#[test]
fn sums_mixed_numbers() {
    let data = [1, 2, 3, 4, 5, 6];
    let mut odd = 0;
    let mut even = 0;
    fun(&data, &mut odd, &mut even);
    assert_eq!(odd, 1 + 3 + 5);
    assert_eq!(even, 2 + 4 + 6);
}

#[test]
fn sums_with_negatives() {
    let data = [-3, -2, -1, 0, 1, 2, 3];
    let mut odd = 0;
    let mut even = 0;
    fun(&data, &mut odd, &mut even);
    assert_eq!(odd, -3 + -1 + 1 + 3);
    assert_eq!(even, -2 + 0 + 2);
}

#[test]
// 空数组
fn sums_empty_slice() {
    let data: [i32; 0] = [];
    let mut odd = 123;
    let mut even = 456;
    fun(&data, &mut odd, &mut even);
    assert_eq!(odd, 0);
    assert_eq!(even, 0);
}
