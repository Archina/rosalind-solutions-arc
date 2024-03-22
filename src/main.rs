fn main() {
    // Solve Problem 09 for 2,2,2
    println!("{:.4}", probability_for_dominant_trait(2, 2, 2));

    // Solve Problem 10 for "GATATATGCATATACTT" in "ATAT"
    println!("{:?}", find_needles("GATATATGCATATACTT", "ATAT"));

	// Solve Problem 11
	println!("{:.3}", protein_mass("SKADYEK"));

	// Solve Problem
	println!("{}", infer_mRNA("MA") % 1_000_000);
}

// Problem 09 IPRB - mendelian
fn probability_for_dominant_trait(k: usize, m: usize, n: usize) -> f64 {
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

// Problem 10 SUBS - motif search in DNA
fn find_needles(haystack: &str, needle: &str) -> Vec<usize> {
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

// Problem PRIM - calculate the protein mass
fn protein_mass(protein: &str) -> f64 {
	protein.chars().fold(0.0, |current, c| current + amino_acid_monoisotopic_mass(&c))
}

fn amino_acid_monoisotopic_mass(amino_acid: &char) -> f64{
	match amino_acid {
		// monoisotopic mass of amino acids
		'A' | 'a' => 71.03711,
		'C' | 'c' => 103.00919,
		'D' | 'd' => 115.02694,
		'E' | 'e' => 129.04259,
		'F' | 'f' => 147.06841,
		'G' | 'g' => 57.02146,
		'H' | 'h' => 137.05891,
		'I' | 'i' => 113.08406,
		'K' | 'k' => 128.09496,
		'L' | 'l' => 113.08406,
		'M' | 'm' => 131.04049,
		'N' | 'n' => 114.04293,
		'P' | 'p' => 97.05276,
		'Q' | 'q' => 128.05858,
		'R' | 'r' => 156.10111,
		'S' | 's' => 87.03203,
		'T' | 't' => 101.04768,
		'V' | 'v' => 99.06841,
		'W' | 'w' => 186.07931,
		'Y' | 'y' => 163.06333,
		_ => panic!("This is not a valid amino acid"),
	}
}

// Problem MRNA - infer mRNA from amino acid sequence
fn infer_mRNA(amino_acid_sequence: &str) -> usize{
	// We always assume a stop, so we can assume at least 3 possible mRNA to start with.
	amino_acid_sequence.replace("Stop", ".").chars().fold(3, |current, c| current * mRNAs(&c))
}

fn mRNAs(from_aa: &char) -> usize {
	match from_aa {
		'A' | 'a' => 4, // GCU, GCC, GCA, GCG
		'C' | 'c' => 2, // UGU, UGC
		'D' | 'd' => 2, // GAU, GAC
		'E' | 'e' => 2, // GAA, GAG
		'F' | 'f' => 2, // UUU, UUC
		'G' | 'g' => 4, // GGU, GGC, GGA, GGG
		'H' | 'h' => 2, // CAU, CAC
		'I' | 'i' => 3, // AUU, AUC, AUA
		'K' | 'k' => 2, // AAA, AAG
		'L' | 'l' => 6, // CUU, CUC, UUA, CUA, UUG, CUG
		'M' | 'm' => 1, // AUG
		'N' | 'n' => 2, // AAU, AAC
		'P' | 'p' => 4, // CCU, CCC, CCA, CCG
		'Q' | 'q' => 2, // CAA, CAG
		'R' | 'r' => 6, // CGU, CGC, CGA, AGA, CGG, AGG
		'S' | 's' => 6, // UCU, UCC, UCA, UCG, AGU, AGC
		'.'       => 3, // UAA, UAG, UGA - maybe this should panic instead...
		'T' | 't' => 4, // ACU, ACC, ACA, ACG
		'V' | 'v' => 4, // GUU, GUC, GUA, GUG
		'W' | 'w' => 1, // UGG
		'Y' | 'y' => 2, // UAU, UAC
		_ => panic!("This is not a valid amino acid")
	}
}
