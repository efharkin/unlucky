fn roll_distribution(num_dice: u32, num_sides: u32) -> Vec<f64> {
    return vec![];
}

/// Compute the q-analog of the factorial
///
/// [k]_q! = q^0 * (q^0 + q^1) * ... * (q^0 + ... + q^{k-1})
///
/// Equal to the standard factorial for q=1.
///
/// # Reference
///
/// [Wolfram MathWorld](https://mathworld.wolfram.com/q-Factorial.html)
///
pub fn q_factorial(k: u64, q: u64) -> u64 {
    return recursive_q_factorial(k, q, 1, 1, 1, 1);
}

fn recursive_q_factorial(
    k: u64,
    q: u64,
    i: u64,
    mut running_exp: u64,
    mut running_product: u64,
    mut running_sum: u64,
) -> u64 {
    if i < k {
        running_exp *= q; // Compute q^i
        running_sum += running_exp; // Compute 1 + q + q^2 + ... + q^i
        running_product *= running_sum; // Compute (1 + q) * (1 + q + q^2) * ...
        return recursive_q_factorial(k, q, i + 1, running_exp, running_product, running_sum);
    } else {
        // Stop if i = k-1
        return running_product;
    }
}

#[cfg(test)]
mod factorial_tests {
    use super::*;

    static Q_TEST_VALUES: [u64; 4] = [1, 2, 8, 50];

    #[test]
    fn k_equal_1() {
        for q in Q_TEST_VALUES.iter() {
            let result = q_factorial(1, *q);
            assert_eq!(
                result, 1,
                "Expected q-factorial = 1 for k=1, q={}, got {} instead",
                *q, result
            );
        }
    }

    #[test]
    fn k_equal_2() {
        for q in Q_TEST_VALUES.iter() {
            let result = q_factorial(2, *q);
            assert_eq!(
                result,
                1 + *q,
                "Expected q-factorial = {} for k=2, q={}, got {} instead",
                1 + *q,
                *q,
                result
            );
        }
    }

    #[test]
    fn k_equal_4() {
        for q in Q_TEST_VALUES.iter() {
            let expected_result = (1 + *q) * (1 + *q + q.pow(2)) * (1 + *q + q.pow(2) + q.pow(3));
            let result = q_factorial(4, *q);
            assert_eq!(
                result, expected_result,
                "Expected q-factorial = {} for k=4, q={}, got {} instead",
                expected_result, *q, result
            );
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
