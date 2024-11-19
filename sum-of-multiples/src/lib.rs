use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // todo!("Sum the multiples of all of {factors:?} which are less than {limit}")
    let mut set: HashSet<u32> = HashSet::new();
    factors.iter().for_each(|x| {
        for i in 0..limit {
            if x * i < limit {
                set.insert(x * i);
            } else {
                break;
            }
        }
    });
    set.iter().sum()
}
