//mod _iprb; // solved
//mod _subs; // solved
//mod _prim; // solved
//mod _mrna; // solved

pub mod orfs;

use noodles_fasta as fasta;
use std::fs::File;
use std::io::BufReader;

fn main() {
    // orfs::generate_code_mapping();
    orfs::find_all();
    
    let mut reader = File::open("reference.fa")
        .map(BufReader::new)
        .map(fasta::Reader::new)
        .unwrap();

    let results: Vec<fasta::Record> = reader
        .records()
        .into_iter()
        .filter_map(|r| r.ok())
        .collect();

    for record in &results {
        let record = record.sequence().slice(..);
        println!("{record:?}");
    }

    for out in generate_overlap_graph(&results, 3) {
        println!("{:?}", out);
    }
}

fn generate_overlap_graph(sequences: &Vec<fasta::Record>, k: usize) -> Vec<(usize, usize)> {
    let mut graph = vec![];
    for i in 0..sequences.len() {
        for j in 0..sequences.len() {
            if i == j {
                continue;
            }
            let i_from = 0;
            let i_to = k;

            let length_j = sequences[j].sequence().len();
            let j_from = length_j - k - 1;
            let j_to = length_j;

            let long_enough =
                sequences[i].sequence().len() > k && sequences[j].sequence().len() > k;

            if long_enough
                && sequences[i].sequence().as_ref()[i_from..i_to]
                    == sequences[j].sequence().as_ref()[j_from..j_to]
            {
                graph.push((i, j));
            }
        }
    }
    return graph;
}

// Problem - enumerate lexicographically
