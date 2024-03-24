#[test]
fn assert_example_problem() {
	assert_eq!(infer_mRNA("MA") % 1_000_000, 12)
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