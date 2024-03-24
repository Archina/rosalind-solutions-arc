#[test]
fn assert_example_problem() {
    assert_eq!(protein_mass("SKADYEK"), 821.3919199999999)
}

// Problem PRIM - calculate the protein mass
pub fn protein_mass(protein: &str) -> f64 {
    protein
        .chars()
        .fold(0.0, |current, c| current + amino_acid_monoisotopic_mass(&c))
}

fn amino_acid_monoisotopic_mass(amino_acid: &char) -> f64 {
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
