use link_ex_5::Polynomial;

fn assert_terms_close(actual: Vec<(i32, f32)>, expected: &[(i32, f32)]) {
    assert_eq!(actual.len(), expected.len());
    for ((actual_exp, actual_coef), (expected_exp, expected_coef)) in actual.iter().zip(expected) {
        assert_eq!(actual_exp, expected_exp);
        assert!((actual_coef - expected_coef).abs() < 1e-6);
    }
}

#[test]
fn polynomial_addition_merges_like_exponents_and_sorts_descending() {
    let pa = Polynomial::from_terms(&[(3, 2.0), (1, -4.0), (0, 7.0)]);
    let pb = Polynomial::from_terms(&[(4, 1.5), (3, -2.0), (1, 1.0)]);

    let sum = pa.add(&pb);

    assert_terms_close(sum.to_vec(), &[(4, 1.5), (1, -3.0), (0, 7.0)]);
}

#[test]
fn polynomial_build_combines_duplicate_terms_and_drops_zero_terms() {
    let p = Polynomial::from_terms(&[(2, 3.0), (5, 1.0), (2, -3.0), (1, 0.0)]);

    assert_terms_close(p.to_vec(), &[(5, 1.0)]);
}

#[test]
fn polynomial_formats_zero_and_nonzero_results() {
    let zero = Polynomial::from_terms(&[(2, 1.0)]).add(&Polynomial::from_terms(&[(2, -1.0)]));
    let p = Polynomial::from_terms(&[(2, -3.5), (0, 2.0)]);

    assert_eq!(zero.format(), "0");
    assert_eq!(p.format(), "-3.5x^2 + 2");
}
