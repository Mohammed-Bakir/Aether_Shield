use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aether_shield::AetherWall;

fn bench_trie_lookup(c: &mut Criterion) {
    let mut firewall = AetherWall::new();
    // Simulate populating it with a few rules
    firewall.insert("abd-bakir.netlify.app");
    firewall.insert("doubleclick.net");
    firewall.insert("googleads.g.doubleclick.net");

    c.bench_function("trie_contains_speed", |b| {
        b.iter(|| firewall.contains(black_box("abd-bakir.netlify.app")))
    });
}

criterion_group!(benches, bench_trie_lookup);
criterion_main!(benches);
