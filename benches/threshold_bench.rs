use criterion::{criterion_group, criterion_main, Criterion};

fn multiset_threshold(c: &mut Criterion) {
    let (multiset, threshold) = gen::multiset();
    c.bench_function("threshold", move |b| {
        b.iter(|| multiset.threshold(threshold))
    });
}

criterion_group!(benches, multiset_threshold);
criterion_main!(benches);

mod gen {
    use rand::prelude::*;
    use threshold::multiset::MultiSet;

    const SEED: u64 = 1002191092;
    const THRESHOLD: u64 = 5;
    const ADD_COUNT: u32 = 10;
    const ELEM_COUNT: u32 = 100;
    const ELEM_SIZE: u32 = 2000;

    pub fn multiset() -> (MultiSet<String, u64>, u64) {
        let mut rng = StdRng::seed_from_u64(SEED);
        let mut multiset = MultiSet::new();
        let elems = elements(&mut rng);

        for _ in 0..ADD_COUNT {
            let elem_count = rng.gen_range(1, ELEM_COUNT) as usize;
            let set: Vec<(String, u64)> = elems
                .choose_multiple(&mut rng, elem_count)
                .cloned()
                .map(|x| (x, 1))
                .collect();
            multiset.add(set);
        }

        (multiset, THRESHOLD)
    }

    fn elements(rng: &mut StdRng) -> Vec<String> {
        (0..ELEM_COUNT)
            .map(|_| format!("{:?}", element(rng)))
            .collect()
    }

    fn element(rng: &mut StdRng) -> Vec<u32> {
        let mut nums: Vec<u32> = (0..ELEM_SIZE).collect();
        nums.shuffle(rng);
        nums
    }
}
