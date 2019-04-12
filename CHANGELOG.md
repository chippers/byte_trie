## 0.3.0 (April 11, 2019)

It clicked in my head that with my implementation of `Node` that I could
generically implement any key whose maximum size is `u8`.  So I made some
traits.

* New traits [`BytesTrie`] and [`BytesKey`]
* Existing [`ByteTrie`] moved to [`tries`] module
* 2 new trie implementation, [`NibbleTrie`] and [`BitTrie`]
* 3 new key implementations, [`ByteKey`], [`NibbleKey`] and [`BitKey`]
* Internal `Node` is now a public [`AdaptiveNode`]
* prelude added: [`use byte_trie::prelude::*`]

[`BytesTrie`]: https://docs.rs/byte_trie/0.3.0/byte_trie/trait.BytesTrie.html
[`BytesKey`]: https://docs.rs/byte_trie/0.3.0/byte_trie/trait.BytesKey.html
[`ByteTrie`]: https://docs.rs/byte_trie/0.3.0/byte_trie/tries/struct.ByteTrie.html
[`tries`]: https://docs.rs/byte_trie/0.3.0/byte_trie/tries/index.html
[`NibbleTrie`]: https://docs.rs/byte_trie/0.3.0/byte_trie/tries/struct.NibbleTrie.html
[`BitTrie`]: https://docs.rs/byte_trie/0.3.0/byte_trie/tries/struct.BitTrie.html
[`ByteKey`]: https://docs.rs/byte_trie/0.3.0/byte_trie/keys/struct.ByteKey.html
[`NibbleKey`]: https://docs.rs/byte_trie/0.3.0/byte_trie/keys/struct.NibbleKey.html
[`BitKey`]: https://docs.rs/byte_trie/0.3.0/byte_trie/keys/struct.BitKey.html
[`AdaptiveNode`]: https://docs.rs/byte_trie/0.3.0/byte_trie/struct.AdaptiveNode.html
[`use byte_trie::prelude::*`]: https://docs.rs/byte_trie/0.3.0/byte_trie/prelude/index.html

## 0.2.0 (April 9, 2019)

New minor just to be safe:
* Change internal `Node` child to be an `Option`
* Changes to how new nodes are inserted into the trie

### 0.1.1 (April 8, 2019)

* Remove accidental debugging

## 0.1.0 (April 8, 2019)

Initial release.  Includes:
* Insert only byte key Trie
* `serde` feature to serialize keys as hex
