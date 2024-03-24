#[test]
fn assert_example_problem() {
    assert_eq!(find_needles("GATATATGCATATACTT", "ATAT"), vec![2, 4, 10])
}

#[test]
fn assert_actual_problem() {
	let haystack = include_str!("_subs_hay");
	let needle = include_str!("_subs_needle");
	
	assert_eq!(find_needles(haystack, needle), vec![40, 55, 100, 107, 134, 189, 248, 277, 380, 414, 447, 465, 605, 612, 637, 702, 719, 764, 771, 787, 802, 827, 834, 845, 860, 895, 940])
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
