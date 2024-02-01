use crate::rule::*;

pub struct CellAutomata1D {
    state: Vec<bool>,
    rule: Rule
}

impl CellAutomata1D {
    pub fn new(init_state: &str, rule_num: u8) -> Self {
        CellAutomata1D { 
            state: Self::str_state_to_vec(init_state), 
            rule: Rule::new(rule_num) 
        }
    }

    pub fn with_rule(rule_num: u8) -> Self {
        CellAutomata1D { 
            state: Vec::new(), 
            rule: Rule::new(rule_num) 
        }
    }

    pub fn set_state(&mut self, state: &str) {
        self.state = Self::str_state_to_vec(state);
    }

    pub fn set_rule(&mut self, number: u8) {
        self.rule = Rule::new(number);
    }

    pub fn next(&mut self) {
        let mut next_state: Vec<bool> = Vec::with_capacity(self.state.len());
        next_state.resize(self.state.len(), false);
        for i in 0..self.state.len() {
            let triad = self.triad_for(i);
            next_state[i] = self.rule.result_of(triad);
        }
        self.state = next_state;
    }

    pub fn string_state(&self) -> String {
        let mut state_str = String::with_capacity(self.state.len());
        for cell in self.state.iter() {
            state_str.push(if *cell {'#'} else {' '});
        }
        state_str
    }

    pub fn get_state(&self) -> Vec<bool> {
        self.state.clone()
    }

    fn triad_for(&self, ind: usize) -> [bool; 3] {
        let ind = ind as isize;
        let mut triad = [false; 3];
        let mut bit_ind = 0;
        for i in ind-1..=ind+1 {
            triad[bit_ind] = self.cycle_at(i);
            bit_ind += 1;
        }
        triad
    }

    fn cycle_at(&self, ind: isize) -> bool {
        if ind >= self.state.len() as isize {
            self.state[0]
        } else if ind < 0 {
            *self.state.last().unwrap()
        } else {
            self.state[ind as usize]
        }
    }

    fn str_state_to_vec(str: &str) -> Vec<bool> {
        let mut vec: Vec<bool> = Vec::with_capacity(str.len());
        for ch in str.chars() {
            vec.push(if ch != '0' {true} else {false});
        }
        vec
    }
}

#[cfg(test)]
mod tests {
    use crate::cellauto::*;

    #[test]
    fn automata_work() {
        let mut automata = CellAutomata1D::new("00001001", 2);
        automata.next();
        assert_eq!(automata.string_state(), "   #  # ");
        automata.next();
        assert_eq!(automata.string_state(), "  #  #  ");
    }

    #[test]
    fn setting_state_works() {
        let mut automata = CellAutomata1D::new("00001001", 2);
        automata.next();
        assert_eq!(automata.string_state(), "   #  # ");
        automata.set_state("10000100");
        automata.next();
        assert_eq!(automata.string_state(), "    #  #");

        let mut automata = CellAutomata1D::with_rule(18);
        automata.set_state("00010000");
        automata.next();
        assert_eq!(automata.string_state(), "  # #   ");
    }

    #[test]
    fn printing_state() {
        let automata = CellAutomata1D::new("11100101", 0b00000100);
        assert_eq!(automata.string_state(), "###  # #");
    }

    #[test]
    fn triad_creating() {
        let automata = CellAutomata1D::new("10000101", 0xff);
        assert_eq!(automata.triad_for(0), [true, true, false]);
        assert_eq!(automata.triad_for(7), [false, true, true]);
        assert_eq!(automata.triad_for(4), [false, false, true]);
    }

    #[test]
    fn cycle_state_indexing() {
        let automata = CellAutomata1D::new("00000101", 0xff);
        assert_eq!(automata.cycle_at(-1), true);
        assert_eq!(automata.cycle_at(8), false);
        assert_eq!(automata.cycle_at(5), true);
        assert_eq!(automata.cycle_at(4), false);
    }

    #[test]
    fn reading_state_from_str() {
        let readed = CellAutomata1D::str_state_to_vec("11100101");
        assert_eq!(readed, vec![true, true, true, false, false, true, false, true]);
    }
}