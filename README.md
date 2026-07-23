**TRANSLATION**

This Rust script builds a hash table containing the set of k-mers from a Fasta file, and serialises it as MessagePack file.

_Trivia:_ I originally had written this script in C++, but facing huge complications when trying to add libraries, it was decided to translate it altogether in Rust, hence the name.

This script lives in the Matterhorn family, along with [cheesy_query](https://github.com/SimooonLcq/cheesy_query) and [insect](https://github.com/SimooonLcq/insect).

**Parameters:**

-i : Fasta file containing the sequence(s) to extract k-mers from. Can be Multifasta. Sequences must be on a single line.

-k : Size (in nucleotides) of k-mers.

-o : Output MessagePack containing the serialized hash table.
