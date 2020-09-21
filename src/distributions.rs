fn roll_distribution(num_dice: u32, num_sides: u32) -> Vec<f64> {
    return vec![];
}

#[derive(Clone)]
pub struct ProbabilityMassFunction {
    pub values: Vec<u64>,
    pub probabilities: Vec<f64>,
}

impl ProbabilityMassFunction {
    pub fn with_capacity(capacity: usize) -> ProbabilityMassFunction {
        ProbabilityMassFunction {
            values: Vec::<u64>::with_capacity(capacity),
            probabilities: Vec::<f64>::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, value: u64, probability: f64) {
        self.values.push(value);
        self.probabilities.push(probability);
    }

    pub fn iter<'a>(&'a self) -> impl Iterator + 'a {
        ProbabilityMassFunctionIterator::new(&self)
    }

    pub fn len(&self) -> usize {
        assert_eq!(self.values.len(), self.probabilities.len());
        self.values.len()
    }
}

struct ProbabilityMassFunctionIterator<'a> {
    pmf: &'a ProbabilityMassFunction,
    ptr: usize,
}

impl<'a> ProbabilityMassFunctionIterator<'a> {
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
    type Item = (u64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr < self.pmf.len() {
            let ptr = self.ptr;
            self.ptr += 1;
            return Some((self.pmf.values[ptr], self.pmf.probabilities[ptr]));
        } else {
            return None;
        }
    }
}



#[cfg(test)]
mod distribution_tests {
    use super::*;

    #[test]
    fn single_roll_prob_dist_length_matches_num_sides() {
        for num_sides in [2, 8, 10, 20, 50].iter() {
            let result = roll_distribution(1, *num_sides);
            assert_eq!(
                result.len() as u32,
                *num_sides,
                "Result length {} does not match 1d{}",
                result.len(),
                *num_sides
            );
        }
    }

    #[test]
    fn single_roll_prob_dist_is_uniform() {
        for num_sides in [2, 8, 10, 20, 50].iter() {
            let result = roll_distribution(1, *num_sides);

            if result.len() == 0 {
                assert!(false, "Got distribution of length zero.")
            } else {
                let expected_prob = 1. / (*num_sides as f64);
                for prob in result.iter() {
                    assert_eq!(
                        *prob, expected_prob,
                        "Expected probability {} for 1d{}, got {} instead",
                        expected_prob, num_sides, prob
                    );
                }
            }
        }
    }
}
