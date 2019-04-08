use byte_trie::ByteTrie;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut bt = ByteTrie::new();
    let oids: Vec<Vec<u8>> = (0..5)
        .map(|_| (0..20).map(|_| rng.gen_range(0, 255)).collect())
        .collect();

    let none: Option<()> = None;

    oids.into_iter().for_each(|v| bt.insert(v, none));
    println!("{:#?}", bt);
}
