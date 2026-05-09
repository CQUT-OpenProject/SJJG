use array_ex_2::odd_before_even;

fn all_odds_before_evens(data: &[i32]) -> bool {
    let mut seen_even = false;

    for item in data {
        if item % 2 == 0 {
            seen_even = true;
        } else if seen_even {
            return false;
        }
    }

    true
}

#[test]
fn moves_odd_numbers_before_even_numbers() {
    let mut data = vec![2, 9, 4, 7, 6, 3, 8, 1];

    odd_before_even(&mut data);

    assert!(all_odds_before_evens(&data));
    assert_eq!(data.len(), 8);
}

#[test]
fn handles_short_and_single_side_lists() {
    let mut empty = Vec::<i32>::new();
    let mut one = vec![2];
    let mut odds = vec![1, 3, 5];
    let mut evens = vec![2, 4, 6];

    odd_before_even(&mut empty);
    odd_before_even(&mut one);
    odd_before_even(&mut odds);
    odd_before_even(&mut evens);

    assert_eq!(empty, Vec::<i32>::new());
    assert_eq!(one, vec![2]);
    assert_eq!(odds, vec![1, 3, 5]);
    assert_eq!(evens, vec![2, 4, 6]);
}
