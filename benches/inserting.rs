use byte_trie::ByteTrie;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::distributions::Alphanumeric;
use rand::prelude::*;

fn random_string(rng: &mut ThreadRng, length: usize) -> String {
    std::iter::repeat(())
        .map(|_| rng.sample(Alphanumeric))
        .take(length)
        .collect()
}

fn inserting(fake_commit_oids: &[(Vec<u8>, String)]) -> ByteTrie<&String> {
    let mut trie = ByteTrie::new();
    fake_commit_oids
        .into_iter()
        .for_each(|(oid, summary)| trie.insert(oid.clone(), summary));
    trie
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = thread_rng();
    let oids: Vec<(Vec<u8>, String)> = (0..1_000)
        .map(|_| {
            let bytes = (0..20).map(|_| rng.gen_range(0, 255)).collect();
            let summary = random_string(&mut rng, 60);
            (bytes, summary)
        })
        .collect();

    c.bench_function("inserting 1,000", move |b| {
        b.iter(|| inserting(black_box(&oids)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
