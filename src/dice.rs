pub fn parse_roll_string(roll_string: String) -> Result<Roll, String> {
    unimplemented!();
}

#[cfg(test)]
mod roll_parse_tests {
    use super::*;

    #[test]
    fn single_set_of_dice() {
        let num_dice = [2, 4, 6];
        let num_sides = [10, 4, 8];
        for (n_dice, n_sides) in num_dice.iter().zip(num_sides.iter()) {
            let actual = parse_roll_string(format!("{}d{}", *n_dice, *n_sides)).expect("Roll parsing failed unexpectedly.");
            assert_eq!(actual.num_dice[0], *n_dice);
            assert_eq!(actual.num_sides[0], *n_sides);
        }
    }

    #[test]
    fn multiple_sets_without_spaces() {
        let actual = parse_roll_string("2d6+1d4".to_string()).expect("Roll parsing failed unexpectedly.");
        assert_eq!(actual.num_dice[0], 2);
        assert_eq!(actual.num_dice[1], 1);
        assert_eq!(actual.num_sides[0], 6);
        assert_eq!(actual.num_sides[1], 4);
    }

    #[test]
    fn multiple_sets_with_spaces() {
        let actual = parse_roll_string("2d6 + 1d4".to_string()).expect("Roll parsing failed unexpectedly.");
        assert_eq!(actual.num_dice[0], 2);
        assert_eq!(actual.num_dice[1], 1);
        assert_eq!(actual.num_sides[0], 6);
        assert_eq!(actual.num_sides[1], 4);
    }

    /// Test whether roll20 `/roll` command prefix is ignored
    #[test]
    fn leading_long_command_is_ignored() {
        let actual = parse_roll_string("/roll 1d4".to_string()).expect("Roll parsing failed unexpectedly.");
        assert_eq!(actual.num_dice[0], 1);
        assert_eq!(actual.num_sides[0], 4);
    }

    /// Test whether roll20 `/roll` command prefix is ignored
    #[test]
    fn leading_short_command_is_ignored() {
        let actual = parse_roll_string("/r 1d4".to_string()).expect("Roll parsing failed unexpectedly.");
        assert_eq!(actual.num_dice[0], 1);
        assert_eq!(actual.num_sides[0], 4);
    }
}

/// A set of dice being rolled.
pub struct Roll {
    num_dice: Vec<u32>,
    num_sides: Vec<u32>,
    modifier: i32,
}

static ROLL_DEFAULT_CAPACITY: usize = 8;

impl Roll {
    /// Get a new `Roll`.
    pub fn new() -> Self {
        Roll {
            num_dice: Vec::<u32>::with_capacity(ROLL_DEFAULT_CAPACITY),
            num_sides: Vec::<u32>::with_capacity(ROLL_DEFAULT_CAPACITY),
            modifier: 0,
        }
    }

    /// Add a die to the roll.
    pub fn add_d(&mut self, num_dice: u32, num_sides: u32) -> &mut Self {
        self.num_dice.push(num_dice);
        self.num_sides.push(num_sides);
        self
    }

    /// Add a modifier (constant offset) to the roll.
    pub fn add_modifier(&mut self, modifier: i32) -> &mut Self {
        self.modifier += modifier;
        self
    }

    /// Get an iterator over the dice in the roll.
    ///
    /// Iterator will return the number of sides on each die individually.
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = u32> + 'a {
        RollIterator::new(&self)
    }

    /// Total number of sides on all dice in this roll.
    pub fn total_num_sides(&self) -> u32 {
        let mut total_num_sides = 0;
        for num_sides in self.iter() {
            total_num_sides += num_sides;
        }
        return total_num_sides;
    }

    pub fn get_modifier(&self) -> i32 {
        self.modifier
    }
}

struct RollIterator<'a> {
    roll: &'a Roll,
    group_ptr: usize,
    die_ptr: u32,
}

impl<'a> RollIterator<'a> {
    fn new(roll: &'a Roll) -> Self {
        RollIterator {
            roll: roll,
            group_ptr: 0,
            die_ptr: 0,
        }
    }
}

impl <'a> Iterator for RollIterator<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        assert_eq!(self.roll.num_dice.len(), self.roll.num_sides.len());

        // Advance to the next group if we've run out of dice in this group.
        if self.die_ptr == self.roll.num_dice[self.group_ptr] {
            self.die_ptr = 0;
            self.group_ptr += 1;
        }

        // If we haven't run out of groups, return the num sides on the next die.
        if self.group_ptr < self.roll.num_dice.len() {
            self.die_ptr += 1;
            return Some(self.roll.num_sides[self.group_ptr]);
        } else {
            return None;
        }
    }
}
