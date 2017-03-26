use crypto::rng::Generator;
use rand::Rng;
use std::collections::HashSet;

#[test]
fn test_rng_output() {
    // Test that we get at least a few distinct values from the RNG if we generate
    // many values. This is kind of a silly test, but it should at least protect
    // against mis-implementing Generator so that it returns the same value every
    // time.
    let mut generator = Generator;
    let values: HashSet<u64> = (0..1000).map(|_| generator.next_u64()).collect();
    assert!(values.len() > 1);
}

#[test]
fn test_rng_range() {
    let mut generator = Generator;
    let min: u64 = 123;
    let max: u64 = 9876;
    for _ in 0..100 {
        let n: u64 = generator.gen_range(min, max);
        assert!(min <= n);
        assert!(n < max);
    }
}
