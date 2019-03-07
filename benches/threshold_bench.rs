use criterion::{criterion_group, criterion_main, Criterion};

fn tset_threshold_union(c: &mut Criterion) {
    let (tset, threshold) = gen::tset();
    c.bench_function("threshold union", move |b| {
        b.iter(|| tset.threshold_union(threshold))
    });
}

criterion_group!(benches, tset_threshold_union);
criterion_main!(benches);

mod gen {
    use rand::prelude::*;
    use threshold::tset::TSet;

    const SEED: u64 = 1002191092;
    const THRESHOLD: u64 = 5;
    const SET_COUNT: u32 = 10;
    const ELEM_COUNT: u32 = 100;
    const ELEM_SIZE: u32 = 2000;

    pub fn tset() -> (TSet<String>, u64) {
        let mut rng = StdRng::seed_from_u64(SEED);
        let mut tset = TSet::new();
        let elems = elements(&mut rng);

        for _ in 0..SET_COUNT {
            let elem_count = rng.gen_range(1, ELEM_COUNT) as usize;
            let set: Vec<String> = elems
                .choose_multiple(&mut rng, elem_count)
                .cloned()
                .collect();
            tset.add(set);
        }

        (tset, THRESHOLD)
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
