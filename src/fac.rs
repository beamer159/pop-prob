use num_bigint::{BigUint, ToBigUint};
use num_traits::One;
use std::collections::HashMap;

pub struct Fac {
    map: HashMap<u32, BigUint>,
}

impl Fac {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert(0, BigUint::one());

        Self { map }
    }

    pub fn fac(&mut self, n: u32) -> &BigUint {
        if !self.map.contains_key(&n) {
            let mut fac = n.to_biguint().unwrap();
            let mut m = n - 1;
            while !self.map.contains_key(&m) {
                fac *= m;
                m -= 1;
            }
            fac *= self.map.get(&m).unwrap();
            self.map.insert(n, fac);
        }
        self.map.get(&n).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn run_single(expected: &BigUint, input: u32) {
        let mut fac = Fac::new();
        assert_eq!(expected, fac.fac(input));
    }

    #[test]
    fn fac_0() {
        run_single(&BigUint::from(1_u32), 0);
    }

    #[test]
    fn fac_1() {
        run_single(&BigUint::from(1_u32), 1);
    }

    #[test]
    fn fac_2() {
        run_single(&BigUint::from(2_u32), 2);
    }

    #[test]
    fn fac_4() {
        run_single(&BigUint::from(24_u32), 4);
    }

    #[test]
    fn fac_8() {
        run_single(&BigUint::from(40320_u32), 8);
    }

    #[test]
    fn fac_16() {
        run_single(&BigUint::from(20922789888000_u64), 16);
    }

    #[test]
    fn fac_32() {
        run_single(
            &BigUint::from(263130836933693530167218012160000000_u128),
            32,
        );
    }

    #[test]
    fn fac_64() {
        run_single(&BigUint::from_str("126886932185884164103433389335161480802865516174545192198801894375214704230400000000000000").unwrap(), 64);
    }

    #[test]
    fn fac_all_at_once() {
        let mut fac = Fac::new();
        assert_eq!(&BigUint::from(1_u32), fac.fac(0));
        assert_eq!(&BigUint::from(1_u32), fac.fac(1));
        assert_eq!(&BigUint::from(2_u32), fac.fac(2));
        assert_eq!(&BigUint::from(24_u32), fac.fac(4));
        assert_eq!(&BigUint::from(40320_u32), fac.fac(8));
        assert_eq!(&BigUint::from(20922789888000_u64), fac.fac(16));
        assert_eq!(
            &BigUint::from(263130836933693530167218012160000000_u128),
            fac.fac(32)
        );
        assert_eq!(&BigUint::from_str("126886932185884164103433389335161480802865516174545192198801894375214704230400000000000000").unwrap(), fac.fac(64));
    }
}
