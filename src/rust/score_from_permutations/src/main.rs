// https://www.codewars.com/kata/5676ffaa8da527f234000025/rust
// Score From Permutations Of Combinations of an Integer

use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    assert_eq!(sc_perm_comb(348), 3675);
}

fn sc_perm_comb(num: u32) -> u64 {
    let v = num.to_string().chars().collect_vec();
    let mut scores = HashSet::new();
    for k in 1..=v.len() {
        for p in v.iter().permutations(k) {
            scores.insert(p.iter().join("").parse::<u64>().unwrap());
        }
    }
    scores.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::sc_perm_comb;

    #[test]
    fn test_sc_perm_comb() {
        assert_eq!(sc_perm_comb(348), 3675);
        assert_eq!(sc_perm_comb(333), 369);
        assert_eq!(sc_perm_comb(340), 1631);
        assert_eq!(sc_perm_comb(0), 0);
        assert_eq!(sc_perm_comb(6), 6);
    }
}
