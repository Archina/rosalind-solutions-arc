#[ignore]
#[test]
fn assert_example_problem() {
	assert_eq!(infer_mRNA("MA"), 12)
}

#[ignore]
#[test]
fn assert_actual_problem() {
	let aminoacids = include_str!("_mrna_aminos");
	assert_eq!(infer_mRNA(aminoacids), 768768)
}

// Problem MRNA - infer mRNA from amino acid sequence
pub fn infer_mRNA(amino_acid_sequence: &str) -> u128 {
	// We always assume a stop, so we can assume at least 3 possible mRNA to start with.
	amino_acid_sequence.replace("Stop", ".").chars().fold(3, |current, c| (current * mRNAs(&c)) % 1_000_000)
}

fn mRNAs(from_aa: &char) -> u128 {
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