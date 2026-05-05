#include <iostream>
#include <fstream>
#include <vector>
#include <string>
#include <argparse/argparse.hpp>
#include "absl/container/flat_hash_set.h"


int main(int argc, char *argv[]){
    std::cout << "Starting\n";
    argparse::ArgumentParser program("swisstables");
    std::string input;
    program.add_argument("-f").help("Fasta input filename").required().store_into(input);
    int k;
    program.add_argument("-k").help("kmer size").required().store_into(k);
    program.parse_args(argc, argv);

    std::ifstream fasta(input);
    bool header;
    std::vector<std::string> kmers_list;
    std::cout << "ok1\n";
    absl::flat_hash_set<std::string> kmers;
    std::cout << "ok2\n";

    std::string line;
    while (getline(fasta, line)){
        std::cout << line << '\n';
        header = false;
        if (line.substr(0,1) == ">"){
            header = true;
        }

        if (!header){
            if (!line.empty() && line[line.length()-1] == '\n') {
                line.erase(line.length()-1);
            }
            for (int i=0; i<line.length()-k+1; i++){
                kmers.insert(line.substr(i, k));
                kmers_list.push_back(line.substr(i, k));
            }
        }
    }

    for (std::string seq : kmers_list){
        auto it = kmers.find(seq);
        bool seq_occurs = false;
        for (auto thing : *it){
            seq_occurs = true;
        }
        if (seq_occurs){
            std::cout << seq << " : True\n";
        }
        else{
            std::cout << seq << " : False\n";
        }
    }
    
    fasta.close();
    return 0;
}