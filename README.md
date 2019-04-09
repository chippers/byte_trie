# byte_trie

* crate: https://crates.io/crates/byte_trie
* docs: https://docs.rs/byte_trie

A compressed trie structure specifically for a list of bytes.  Made while
wanting to create a serialized trie of git Oid hashes, which are 20 byte
long arrays.  Played with some weird node size stuff to (hopefully) optimize
the size of a edge node since git oids get unique very fast.

The performance goal is being able to insert and serialize every commit in
the linux git repository (825k+ commits), which it does.

Improvements to be made but it works for now.

## Features
* Insertion
* Serialization as hex (feature `serde`)

## Todo
* Deletion (and re-compression)
* Documentation
* Testing


### Serialization Example

The trie serializes the bytes as hex strings in the same shape it stores the
nodes.  Compressed, empty nodes are skipped when serializing so that there
should be no keys serialized that is an empty string.  Here is a small json
snippet of a few commits from the Typescript repository from a serialized file
of 25k commits.

```json
{
  "00": {
    "01": {
      "1a52af387cabeaddb261ca426529f1cdbe5a": "Refactor root files addition/update for non inferred project",
      "b8cb37e1988a4809aff8e5c8f55dd0f98ee6": "Remove target-following code when erasing signatures"
    },
    "0206fd8fdb5118d14371b0f5f033c311653ca5": "Update servicesVersion",
    "0637156aba40b51f7410c53783d01e27a73b6d": "Merge pull request #10374 from Microsoft/readonly-array-type-argument-assignability",
    "0f121d348913ca13ba1354f21adaf10eabc3c4": "Improve conditional type constraint checking",
    "10a38660b1eb03c188c9c1758177b4501760b7": "Merge pull request #28343 from Microsoft/lib/update-nov-2018",
    "130f1a68011ae59fb61e2f70897d48ec100b47": "Handle when using -p and config file not found",
    "16fd72f749f25561a4d70ad36978d48b3505c2": "Add test",
    "1b7b5bbe872d83ccb38ddb38ec70f0a2f1327a": "Merge pull request #2 from Microsoft/master"
  }
}
```