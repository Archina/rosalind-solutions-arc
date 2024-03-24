// Problem 09 IPRB - mendelian
pub fn probability_for_dominant_trait(k: usize, m: usize, n: usize) -> f64 {
    let sum = k + m + n;

    probability(k, sum)
        + probability(m, sum)
            * (probability(k, sum - 1)
                + probability(m - 1, sum - 1) * 0.75
                + probability(n, sum - 1) * 0.5)
        + probability(n, sum) * (probability(k, sum - 1) + probability(m, sum - 1) * 0.5)
}

fn probability(n: usize, out_of: usize) -> f64 {
    n as f64 / out_of as f64
}

#[ignore]
#[test]
fn asset_example_problem() {
    assert_eq!(probability_for_dominant_trait(2, 2, 2), 0.7833333333333332)
}
