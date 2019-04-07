use byte_trie::ByteTrie;

fn main() {
    let mut bt = ByteTrie::new();
    bt.insert(vec![1, 2, 3, 4], "Commit Message");
    println!("{:#?}", bt);
}
