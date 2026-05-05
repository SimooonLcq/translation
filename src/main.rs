use std::collections::HashSet;
use bio::io::fasta::Reader;
use clap::Parser;
use serde::Serialize;
use serde_json;
use std::fs;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    ///Kmer length
    #[arg(short)]
    k: usize,

    ///Input Fasta filename
    #[arg(short = 'f', long = "file")]
    filein: String,

    ///Output Json filename
    #[arg(short = 'o', long = "output")]
    fileout: String,
}

#[derive(Serialize)]
struct HSet{
    x: HashSet<Vec<u8>>,
}

fn main() {
    let parser = Args::parse();

    let result0 = Reader::from_file(parser.filein);
    let reader = result0.expect("Error during file opening");

    let mut kmers:HashSet<Vec<u8>> = HashSet::new();
    for result in reader.records(){
        let seq = result.expect("Error during fasta record parsing").seq().to_vec();
        for i in 0..seq.len()-parser.k+1{
            kmers.insert((&seq[i..i+parser.k]).to_vec());
        }
    }

    let hset = HSet{x:kmers};
    let serialized = serde_json::to_string(&hset).unwrap();

    fs::write(parser.fileout, serialized).expect("Error during writing");
}
