// Open reading frames

use itertools::Itertools;

enum Codon {
    Stop,
    Codon(char),
}

struct Track {
    active: bool,
    sequence: Vec<char>,
    additional_starts: Vec<usize>,
}

impl Default for Track {
    fn default() -> Self {
        Self {
            active: false,
            sequence: vec![],
            additional_starts: vec![],
        }
    }
}

impl Track {
    fn add(&mut self, char: char) {
        if self.active || char == 'M' {
            self.active = true;
            if char == 'M' {
                self.additional_starts.push(self.sequence.len());
            }
            self.sequence.push(char);
        }
    }

    fn finish(&mut self, drain_into: &mut Vec<Vec<char>>) {
        if self.active {
            for index in &self.additional_starts {
                drain_into.push(
                    self.sequence[*index..self.sequence.len()]
                        .iter()
                        .copied()
                        .collect::<Vec<char>>(),
                );
            }
            self.additional_starts = vec![];
            self.active = false;
        }
    }
}

pub fn find_all() {
    let input: Vec<_> = "AGCCATGTAGCTAACTCAGGTTACATGGGGATGACCCCGCGACTTGGATTAGAGTCTCTTTTGGAATAAGCCTGAATGATCCGAGTAGCATCTCAG".chars().into_iter().map(|c| if c == 'T' {'U'} else {c}).collect();
    let mut inverse = input
        .iter()
        .map(|c| match c {
            'A' => 'U',
            'G' => 'C',
            'C' => 'G',
            _ => 'A',
        })
        .collect::<Vec<_>>();

    println!(
        "{}\n{}",
        input.clone().iter().collect::<String>(),
        inverse.clone().iter().collect::<String>()
    );

    let mut finished = vec![];

    let (mut a, mut b, mut c) = (Track::default(), Track::default(), Track::default());
    let (mut d, mut e, mut f) = (Track::default(), Track::default(), Track::default());

    inverse.reverse();

    for i in 0..(input.len() - 2) {
        if i % 3 == 0 {
            // println!("{:?}", &input[i..i + 3]);
            let matched = codon_mapping(&input[i..i + 3]);
            match matched {
                Codon::Stop => a.finish(&mut finished),
                Codon::Codon(c) => a.add(c),
            }
        }
        if i % 3 == 1 {
            let matched = codon_mapping(&input[i..i + 3]);
            match matched {
                Codon::Stop => b.finish(&mut finished),
                Codon::Codon(c) => b.add(c),
            }
        }
        if i % 3 == 2 {
            let matched = codon_mapping(&input[i..i + 3]);
            match matched {
                Codon::Stop => c.finish(&mut finished),
                Codon::Codon(char) => c.add(char),
            }
        }
        if i % 3 == 0 {
            // println!("{:?}", &inverse[i..i + 3]);
            let matched = codon_mapping(&inverse[i..i + 3]);
            match matched {
                Codon::Stop => d.finish(&mut finished),
                Codon::Codon(c) => d.add(c),
            }
        }
        if i % 3 == 1 {
            let matched = codon_mapping(&inverse[i..i + 3]);
            match matched {
                Codon::Stop => e.finish(&mut finished),
                Codon::Codon(c) => e.add(c),
            }
        }
        if i % 3 == 2 {
            let matched = codon_mapping(&inverse[i..i + 3]);
            match matched {
                Codon::Stop => f.finish(&mut finished),
                Codon::Codon(char) => f.add(char),
            }
        }
    }
    println!("Found sequences:");
    for find in finished
        .into_iter()
        .map(|e| e.iter().collect::<String>())
        .unique(){
        println!("{find}");    
    }
}

fn codon_mapping(slice: &[char]) -> Codon {
    match slice {
        ['G', 'C', 'U'] | ['G', 'C', 'C'] | ['G', 'C', 'A'] | ['G', 'C', 'G'] => Codon::Codon('A'),
        ['U', 'G', 'U'] | ['U', 'G', 'C'] => Codon::Codon('C'),
        ['G', 'A', 'U'] | ['G', 'A', 'C'] => Codon::Codon('D'),
        ['G', 'A', 'A'] | ['G', 'A', 'G'] => Codon::Codon('E'),
        ['U', 'U', 'U'] | ['U', 'U', 'C'] => Codon::Codon('F'),
        ['G', 'G', 'U'] | ['G', 'G', 'C'] | ['G', 'G', 'A'] | ['G', 'G', 'G'] => Codon::Codon('G'),
        ['C', 'A', 'U'] | ['C', 'A', 'C'] => Codon::Codon('H'),
        ['A', 'U', 'U'] | ['A', 'U', 'C'] | ['A', 'U', 'A'] => Codon::Codon('I'),
        ['A', 'A', 'A'] | ['A', 'A', 'G'] => Codon::Codon('K'),
        ['C', 'U', 'U']
        | ['C', 'U', 'C']
        | ['U', 'U', 'A']
        | ['C', 'U', 'A']
        | ['U', 'U', 'G']
        | ['C', 'U', 'G'] => Codon::Codon('L'),
        ['A', 'U', 'G'] => Codon::Codon('M'),
        ['A', 'A', 'U'] | ['A', 'A', 'C'] => Codon::Codon('N'),
        ['C', 'C', 'U'] | ['C', 'C', 'C'] | ['C', 'C', 'A'] | ['C', 'C', 'G'] => Codon::Codon('P'),
        ['C', 'A', 'A'] | ['C', 'A', 'G'] => Codon::Codon('Q'),
        ['C', 'G', 'U']
        | ['C', 'G', 'C']
        | ['C', 'G', 'A']
        | ['A', 'G', 'A']
        | ['C', 'G', 'G']
        | ['A', 'G', 'G'] => Codon::Codon('R'),
        ['U', 'C', 'U']
        | ['U', 'C', 'C']
        | ['U', 'C', 'A']
        | ['U', 'C', 'G']
        | ['A', 'G', 'U']
        | ['A', 'G', 'C'] => Codon::Codon('S'),
        ['A', 'C', 'U'] | ['A', 'C', 'C'] | ['A', 'C', 'A'] | ['A', 'C', 'G'] => Codon::Codon('T'),
        ['G', 'U', 'U'] | ['G', 'U', 'C'] | ['G', 'U', 'A'] | ['G', 'U', 'G'] => Codon::Codon('V'),
        ['U', 'G', 'G'] => Codon::Codon('W'),
        ['U', 'A', 'U'] | ['U', 'A', 'C'] => Codon::Codon('Y'),
        _ => Codon::Stop,
    }
}

fn codons() -> Vec<(char, String)> {
    vec![
        ('A', "GCU, GCC, GCA, GCG".into()),
        ('C', "UGU, UGC".into()),
        ('D', "GAU, GAC".into()),
        ('E', "GAA, GAG".into()),
        ('F', "UUU, UUC".into()),
        ('G', "GGU, GGC, GGA, GGG".into()),
        ('H', "CAU, CAC".into()),
        ('I', "AUU, AUC, AUA".into()),
        ('K', "AAA, AAG".into()),
        ('L', "CUU, CUC, UUA, CUA, UUG, CUG".into()),
        ('M', "AUG".into()),
        ('N', "AAU, AAC".into()),
        // ('O', "UGU, UGC".into()),
        ('P', "CCU, CCC, CCA, CCG".into()),
        ('Q', "CAA, CAG".into()),
        ('R', "CGU, CGC, CGA, AGA, CGG, AGG".into()),
        ('S', "UCU, UCC, UCA, UCG, AGU, AGC".into()),
        ('T', "ACU, ACC, ACA, ACG".into()),
        ('V', "GUU, GUC, GUA, GUG".into()),
        ('W', "UGG".into()),
        ('Y', "UAU, UAC".into()),
    ]
}

pub fn generate_code_mapping() {
    for (c, alts) in codons() {
        let codons = alts
            .split(", ")
            .map(|x| {
                format!(
                    "[{}]",
                    x.chars()
                        .into_iter()
                        .map(|c| format!("'{c}'"))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            })
            .collect::<Vec<_>>()
            .join(" | ");
        println!("{codons} => Codon::Codon('{c}'),");
    }
}
