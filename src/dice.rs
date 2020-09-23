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
