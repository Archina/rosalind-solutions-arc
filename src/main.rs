fn main() {
    let haystack = "GATATATGCATATACTT";
    let needle = "ATAT";

    println!("{:?}", find_needles(&haystack, &needle));
}

fn find_needles(
    haystack: &str,
    needle: &str,
) -> Vec<usize> {
    let mut found_needle_index = vec![];
    let needle_length = needle.len();
    let haystack_length = haystack.len();

    for index in 0..haystack_length - needle_length {
        if haystack.chars().skip(index).take(needle_length).eq(needle.chars()) {
            found_needle_index.push(index+1);
        }
    }

    found_needle_index
}
