use crate::pop_prob_error::PopProbError;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::collections::HashMap;

pub struct Snsk {
    map: HashMap<(u32, u32), BigUint>,
}

impl Snsk {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert((0, 0), BigUint::one());
        map.insert((1, 0), BigUint::zero());
        Self { map }
    }

    pub fn calc(&mut self, n: u32, k: u32) -> Result<&BigUint, PopProbError> {
        if k > n {
            Err(PopProbError::SnskKGreaterThanN((k, n)))
        } else {
            self.store(n, k);
            Ok(self.get(n, k).unwrap())
        }
    }

    fn store(&mut self, n: u32, k: u32) {
        if let None = self.get(n, k) {
            self.store(n - 1, k - 1);
            self.store(n - 1, k);
            let nk_value = self.get(n - 1, k - 1).unwrap() + k * self.get(n - 1, k).unwrap();
            self.map.insert((n, k), nk_value);
        }
    }

    fn get(&self, n: u32, k: u32) -> Option<&BigUint> {
        match k {
            k if k == 1 || k == n => Some(&self.map[&(0, 0)]),
            0 => Some(&self.map[&(1, 0)]),
            _ => self.map.get(&(n, k)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_single(expected: u32, n: u32, k: u32) {
        let mut snsk = Snsk::new();
        assert_eq!(&BigUint::from(expected), snsk.calc(n, k).unwrap());
    }

    #[test]
    fn snsk_0_0() {
        run_single(1, 0, 0);
    }

    #[test]
    fn snsk_1_0() {
        run_single(0, 1, 0);
    }

    #[test]
    fn snsk_1_1() {
        run_single(1, 1, 1);
    }

    #[test]
    fn snsk_2_1() {
        run_single(1, 2, 1);
    }

    #[test]
    fn snsk_2_2() {
        run_single(1, 2, 2);
    }

    #[test]
    fn snsk_3_1() {
        run_single(1, 3, 1);
    }

    #[test]
    fn snsk_3_2() {
        run_single(3, 3, 2);
    }

    #[test]
    fn snsk_3_3() {
        run_single(1, 3, 3);
    }

    #[test]
    fn snsk_4_1() {
        run_single(1, 4, 1);
    }

    #[test]
    fn snsk_4_2() {
        run_single(7, 4, 2);
    }

    #[test]
    fn snsk_4_3() {
        run_single(6, 4, 3);
    }

    #[test]
    fn snsk_4_4() {
        run_single(1, 4, 4);
    }

    #[test]
    fn snsk_5_2() {
        run_single(15, 5, 2);
    }

    #[test]
    fn snsk_5_3() {
        run_single(25, 5, 3);
    }

    #[test]
    fn snsk_5_4() {
        run_single(10, 5, 4);
    }

    #[test]
    fn snsk_6_2() {
        run_single(31, 6, 2);
    }

    #[test]
    fn snsk_6_3() {
        run_single(90, 6, 3);
    }

    #[test]
    fn snsk_6_4() {
        run_single(65, 6, 4);
    }

    #[test]
    fn snsk_6_5() {
        run_single(15, 6, 5);
    }

    #[test]
    fn snsk_7_2() {
        run_single(63, 7, 2);
    }

    #[test]
    fn snsk_7_3() {
        run_single(301, 7, 3);
    }

    #[test]
    fn snsk_7_4() {
        run_single(350, 7, 4);
    }

    #[test]
    fn snsk_7_5() {
        run_single(140, 7, 5);
    }

    #[test]
    fn snsk_7_6() {
        run_single(21, 7, 6);
    }

    #[test]
    fn snsk_9_4() {
        run_single(7770, 9, 4);
    }

    #[test]
    fn snsk_10_5() {
        run_single(42525, 10, 5);
    }

    #[test]
    fn snsk_all_at_once() {
        let mut snsk = Snsk::new();
        assert_eq!(&BigUint::from(1_u32), snsk.calc(0, 0).unwrap());
        assert_eq!(&BigUint::from(0_u32), snsk.calc(1, 0).unwrap());
        assert_eq!(&BigUint::from(1_u32), snsk.calc(1, 1).unwrap());
        assert_eq!(&BigUint::from(1_u32), snsk.calc(2, 1).unwrap());
        assert_eq!(&BigUint::from(1_u32), snsk.calc(2, 2).unwrap());
        assert_eq!(&BigUint::from(1_u32), snsk.calc(3, 1).unwrap());
        assert_eq!(&BigUint::from(3_u32), snsk.calc(3, 2).unwrap());
        assert_eq!(&BigUint::from(1_u32), snsk.calc(3, 3).unwrap());
        assert_eq!(&BigUint::from(1_u32), snsk.calc(4, 1).unwrap());
        assert_eq!(&BigUint::from(7_u32), snsk.calc(4, 2).unwrap());
        assert_eq!(&BigUint::from(6_u32), snsk.calc(4, 3).unwrap());
        assert_eq!(&BigUint::from(1_u32), snsk.calc(4, 4).unwrap());
        assert_eq!(&BigUint::from(15_u32), snsk.calc(5, 2).unwrap());
        assert_eq!(&BigUint::from(25_u32), snsk.calc(5, 3).unwrap());
        assert_eq!(&BigUint::from(10_u32), snsk.calc(5, 4).unwrap());
        assert_eq!(&BigUint::from(31_u32), snsk.calc(6, 2).unwrap());
        assert_eq!(&BigUint::from(90_u32), snsk.calc(6, 3).unwrap());
        assert_eq!(&BigUint::from(65_u32), snsk.calc(6, 4).unwrap());
        assert_eq!(&BigUint::from(15_u32), snsk.calc(6, 5).unwrap());
        assert_eq!(&BigUint::from(63_u32), snsk.calc(7, 2).unwrap());
        assert_eq!(&BigUint::from(301_u32), snsk.calc(7, 3).unwrap());
        assert_eq!(&BigUint::from(350_u32), snsk.calc(7, 4).unwrap());
        assert_eq!(&BigUint::from(140_u32), snsk.calc(7, 5).unwrap());
        assert_eq!(&BigUint::from(21_u32), snsk.calc(7, 6).unwrap());
        assert_eq!(&BigUint::from(7770_u32), snsk.calc(9, 4).unwrap());
        assert_eq!(&BigUint::from(42525_u32), snsk.calc(10, 5).unwrap());
    }

    #[test]
    fn snsk_k_larger_than_n() {
        let mut snsk = Snsk::new();
        assert!(snsk.calc(1, 2).is_err())
    }
}
