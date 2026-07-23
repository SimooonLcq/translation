use std::collections::HashSet;
use helicase::*;
use helicase::input::*;
use clap::Parser;
use serde::Serialize;
use rmp_serde::Serializer;
use std::fs;

const CONFIG: Config = ParserOptions::default().config();

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    ///Kmer length
    #[arg(short)]
    k: usize,

    ///Input Fasta filename
    #[arg(short = 'f', long = "file")]
    filein: String,

    ///Output MessagePack filename
    #[arg(short = 'o', long = "output")]
    fileout: String,
}

#[derive(Serialize)]
struct HSet{
    x: HashSet<Vec<u8>>,
}

fn main() {
    let parser = Args::parse();

    let mut reader = FastaParser::<CONFIG, _>::from_file(&parser.filein).expect("Error during fasta reading");

    let mut kmers:HashSet<Vec<u8>> = HashSet::new();
    while let Some(_event) = reader.next(){
        let seq = reader.get_dna_string_owned();
        for i in 0..seq.len()-parser.k+1{
            kmers.insert((&seq[i..i+parser.k]).to_vec());
        }
    }

    let hset = HSet{x:kmers};
    let mut serialized = Vec::new();
    hset.serialize(&mut Serializer::new(&mut serialized)).unwrap();

    fs::write(parser.fileout, serialized).expect("Error during writing");
}
