pub struct Rule {
    number: u8
}

impl Rule {
    pub fn new(num: u8) -> Self {
        Rule {
            number: num
        }
    }

    pub fn result_of(&self, triad: [bool; 3]) -> bool {
        let mut rule_bit_ind: u8 = 0;
        for (i, cell) in triad.iter().rev().enumerate() {
            rule_bit_ind += (*cell as u8) * 2u8.pow(i as u32);
        }
        self.bit_at(rule_bit_ind)
    }
    
    fn bit_at(&self, ind: u8) -> bool {
        self.number & (1 << ind) != 0
    }
}

#[cfg(test)]
mod tests {
    use crate::rule::*;

    #[test]
    fn creating_rule_from_number() {
        let rule = Rule::new(0b10000101);
        assert!(rule.bit_at(0));
        assert!(rule.bit_at(7));
        assert!(rule.bit_at(2));
        for i in [1,3,4,5,6] {
            assert!(!rule.bit_at(i));
        }
    }
}