# byte_trie

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
