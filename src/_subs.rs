#[test]
fn assert_example_problem() {
    assert_eq!(find_needles("GATATATGCATATACTT", "ATAT"), vec![2, 4, 10])
}

// Problem 10 SUBS - motif search in DNA
pub fn find_needles(haystack: &str, needle: &str) -> Vec<usize> {
    let mut found_needle_index = vec![];
    let needle_length = needle.len();
    let haystack_length = haystack.len();

    for index in 0..haystack_length - needle_length {
        if haystack
            .chars()
            .skip(index)
            .take(needle_length)
            .eq(needle.chars())
        {
            found_needle_index.push(index + 1);
        }
    }

    found_needle_index
}
