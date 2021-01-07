use crate::fac::Fac;
use crate::pop_prob_error::PopProbError;
use crate::snsk::Snsk;
use num_bigint::ToBigUint;
use num_traits::cast::ToPrimitive;

mod fac;
mod pop_prob_error;
mod snsk;

pub struct Calculator {
    fac: Fac,
    snsk: Snsk,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            fac: Fac::new(),
            snsk: Snsk::new(),
        }
    }

    /// Calculates the most likely population size if `sample` elements are selected randomly
    /// with replacement from the population and `unique` of these elements are unique.
    pub fn pop(&mut self, sample: u32, unique: u32) -> Result<u32, PopProbError> {
        let mut prev_prev = None;
        let mut prev: Option<(u32, f64)> = None;
        for pop in (1_u32..).map(|x| unique + 2_u32.pow(x - 1) - 1) {
            let v = (pop, self.prob_unique(pop, sample, unique)?);
            if let Some(prev) = prev {
                if v.1 < prev.1 {
                    return if let Some(prev_prev) = prev_prev {
                        self.find_peak(sample, unique, prev_prev, prev, v)
                    } else {
                        Ok(prev.0)
                    };
                }
            }
            prev_prev = prev;
            prev = Some(v);
        }
        Err(PopProbError::Generic)
    }

    fn find_peak(
        &mut self,
        sample: u32,
        unique: u32,
        lbound: (u32, f64),
        pivot: (u32, f64),
        ubound: (u32, f64),
    ) -> Result<u32, PopProbError> {
        if ubound.0 == pivot.0 + 1 {
            return Ok(pivot.0);
        }
        let (lbound, pivot, ubound) = if ubound.0 - pivot.0 > pivot.0 - lbound.0 {
            let x = (pivot.0 + ubound.0) / 2;
            let v = (x, self.prob_unique(x, sample, unique)?);
            if v.1 < pivot.1 {
                (lbound, pivot, v)
            } else {
                (pivot, v, ubound)
            }
        } else {
            let x = (lbound.0 + pivot.0) / 2;
            let v = (x, self.prob_unique(x, sample, unique)?);
            if pivot.1 < v.1 {
                (lbound, v, pivot)
            } else {
                (v, pivot, ubound)
            }
        };
        self.find_peak(sample, unique, lbound, pivot, ubound)
    }

    /// Calculates the probability that a population contains exactly `size` unique elements if
    /// `sample` elements are selected randomly with replacement from the population and
    /// `unique` of these elements are unique.
    pub fn prob_pop(&mut self, sample: u32, unique: u32, size: u32) -> Result<f64, PopProbError> {
        let numerator = self.prob_unique(size, sample, unique)?;
        let mut denominator = numerator;
        let mut latest_prob = numerator;
        let mut low_size = size - 1;
        let mut low_prob = self.prob_unique(low_size, sample, unique)?;
        let mut high_size = size + 1;
        let mut high_prob = self.prob_unique(high_size, sample, unique)?;
        while latest_prob * 10000.0 > denominator {
            if low_prob > high_prob {
                latest_prob = low_prob;
                denominator += low_prob;
                low_size -= 1;
                low_prob = if low_size < unique {
                    0.0
                } else {
                    self.prob_unique(low_size, sample, unique)?
                };
            } else {
                latest_prob = high_prob;
                denominator += high_prob;
                high_size += 1;
                high_prob = self.prob_unique(high_size, sample, unique)?;
            };
        }
        Ok(numerator / denominator)
    }

    /// Calculates the probability that `unique` unique elements will be selected after
    /// `sample` elements are selected randomly with replacement
    /// from a population containing exactly `size` unique elements
    pub fn prob_unique(
        &mut self,
        size: u32,
        sample: u32,
        unique: u32,
    ) -> Result<f64, PopProbError> {
        if unique > size {
            return Err(PopProbError::UniqueGreaterThanSize((unique, size)));
        }
        if unique > sample {
            return Err(PopProbError::UniqueGreaterThanSample((unique, sample)));
        }
        if unique == 0 {
            return Err(PopProbError::UniqueZero);
        }

        let numerator = {
            let snsk = self.snsk.calc(sample, unique)?;
            let fac = self.fac.fac(size).clone();
            snsk * fac
        };

        let denominator = {
            let pow = size.to_biguint().unwrap().pow(sample as u32);
            let fac = self.fac.fac(size - unique).clone();
            pow * fac
        };

        Ok((numerator * u32::max_value() / denominator)
            .to_f64()
            .unwrap()
            / u32::max_value() as f64)
    }
}
