use super::dice::Roll;

pub fn pmf_convolve<'a>(a: &'a ProbabilityMassFunction, b: &'a ProbabilityMassFunction) -> ProbabilityMassFunction {
    let shorter: &ProbabilityMassFunction;
    let longer: &ProbabilityMassFunction;
    if a.len() <= b.len() {
        shorter = a;
        longer = b;
    } else {
        shorter = b;
        longer = a;
    }

    let mut result = ProbabilityMassFunction::with_capacity(longer.len() + shorter.len() - 1);
    result.offset = shorter.offset + longer.offset;
    for result_ind in 0..(longer.len() + shorter.len() - 1) {
        let mut convolved_proba = 0.0f64;
        for (shorter_ind, shorter_x) in shorter.iter().enumerate() {
            if shorter_ind > result_ind {
                // Prevent convolution from going past the left side.
                break;
            } else {
                let longer_ind = result_ind - shorter_ind;
                if longer_ind >= longer.len() {
                    // Prevent convolution from going past the right side.
                    continue;
                } else {
                    // Valid convolution.
                    convolved_proba += longer.probabilities[longer_ind] * shorter_x.1;
                }
            }
        }

        result.push(convolved_proba);
    }

    return result;
}

#[cfg(test)]
mod convolution_tests {
    use super::*;

    #[test]
    fn coin_flip() {
        let mut coin = ProbabilityMassFunction::with_capacity(2);
        coin.push(0.5);
        coin.push(0.5);

        let mut expected_pmf = ProbabilityMassFunction::with_capacity(3);
        expected_pmf.offset = 2;
        expected_pmf.push(0.25);
        expected_pmf.push(0.50);
        expected_pmf.push(0.25);

        let convolved_pmf = pmf_convolve(&coin, &coin);

        for (expected, actual) in expected_pmf.iter().zip(convolved_pmf.iter()) {
            assert_eq!(expected.0, actual.0);
            assert_eq!(expected.1, actual.1);
        }
    }

    #[test]
    fn two_d_four() {
        let mut d4 = ProbabilityMassFunction::with_capacity(4);
        for _ in 0..4 {
            d4.push(0.25);
        }

        let mut expected_pmf = ProbabilityMassFunction::with_capacity(7);
        expected_pmf.offset = 2;
        {
            let expected_probabilities = [1.0/16.0, 1.0/8.0, 3.0/16.0, 1.0/4.0, 3.0/16.0, 1.0/8.0, 1.0/16.0];
            for i in 0..6 {
                expected_pmf.push(expected_probabilities[i]);
            }
        }

        let convolved_pmf = pmf_convolve(&d4, &d4);

        for (expected, actual) in expected_pmf.iter().zip(convolved_pmf.iter()) {
            assert_eq!(expected.0, actual.0);
            assert_eq!(expected.1, actual.1);
        }
    }

    #[test]
    fn d2_d4() {
        let mut d2 = ProbabilityMassFunction::with_capacity(2);
        for _ in 0..2 {
            d2.push(0.5);
        }

        let mut d4 = ProbabilityMassFunction::with_capacity(4);
        for _ in 0..4 {
            d4.push(0.25);
        }

        let mut expected_pmf = ProbabilityMassFunction::with_capacity(5);
        expected_pmf.offset = 2;
        {
            let expected_probabilities = [1.0/8.0, 1.0/4.0, 1.0/4.0, 1.0/4.0, 1.0/8.0];
            for proba in expected_probabilities.iter() {
                expected_pmf.push(*proba);
            }
        }

        let convolved_pmf = pmf_convolve(&d2, &d4);
        for (expected, actual) in expected_pmf.iter().zip(convolved_pmf.iter()) {
            assert_eq!(expected.0, actual.0);
            assert_eq!(expected.1, actual.1);
        }
    }
}

/// Probability mass function over a range of integers.
pub struct ProbabilityMassFunction {
    offset: i64,
    probabilities: Vec<f64>,
}

impl ProbabilityMassFunction {
    pub fn from_d(num_sides: u32) -> Self {
        let mut pmf = ProbabilityMassFunction::with_capacity(num_sides as usize);
        for _ in 0..num_sides {
            pmf.push(1.0/(num_sides as f64));
        }
        return pmf;
    }

    /// Get a new `ProbabilityMassFunction` with capacity for a given number of distinct values.
    fn with_capacity(capacity: usize) -> ProbabilityMassFunction {
        ProbabilityMassFunction {
            offset: 1,
            probabilities: Vec::<f64>::with_capacity(capacity),
        }
    }

    /// Set the probability of the next value.
    ///
    /// Sets the probability of value `offset + self.len()`.
    fn push(&mut self, probability: f64) {
        self.probabilities.push(probability);
    }

    /// Get an iterator over the values and probabilities of the PMF.
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (i64, f64)> + 'a {
        ProbabilityMassFunctionIterator::new(&self)
    }

    /// Number of distinct values in the domain of the PMF.
    pub fn len(&self) -> usize {
        self.probabilities.len()
    }
}

/// Get the PMF for a Roll.
impl From<&Roll> for ProbabilityMassFunction {
    fn from(roll: &Roll) -> ProbabilityMassFunction {
        let mut roll_iter = roll.iter();
        let mut pmf = ProbabilityMassFunction::from_d(roll_iter.next().expect("Cannot create a PMF from a roll with no dice."));
        pmf.offset = roll.get_modifier() as i64;

        for d in roll_iter {
            let d_pmf = ProbabilityMassFunction::from_d(d);
            pmf = pmf_convolve(&pmf, &d_pmf);
        }

        return pmf;
    }
}

/// Iterator over the values and probabilities of a PMF.
struct ProbabilityMassFunctionIterator<'a> {
    pmf: &'a ProbabilityMassFunction,
    ptr: usize,
}

impl<'a> ProbabilityMassFunctionIterator<'a> {
    /// Get a new `ProbabilityMassFunctionIterator`.
    pub fn new(pmf: &'a ProbabilityMassFunction) -> Self {
        ProbabilityMassFunctionIterator {
            pmf: pmf,
            ptr: 0
        }
    }
}

/// Iterate over a ProbabilityMassFunction
///
/// Each iteration step returns a (value, probability) tuple.
impl <'a> Iterator for ProbabilityMassFunctionIterator<'a> {
    type Item = (i64, f64);

    /// Get the next (value, probability) pair in the PMF.
    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr < self.pmf.len() {
            let ptr = self.ptr;
            self.ptr += 1;
            return Some((self.pmf.offset + (ptr as i64), self.pmf.probabilities[ptr]));
        } else {
            return None;
        }
    }
}
