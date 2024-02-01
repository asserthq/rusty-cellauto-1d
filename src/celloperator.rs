use crate::cellauto::CellAutomata1D;

pub struct CellOperator {
    automata: CellAutomata1D
}

impl CellOperator {
    pub fn new(rule: u8) -> Self {
        CellOperator {
            automata: CellAutomata1D::with_rule(rule)
        }
    }

    pub fn eval(&mut self, lhs: u8, rhs: u8) -> u8 {
        fn bit_char(num: u8, pos: u8) -> char {
            match num & (1 << (7 - pos)) {
                0 => '0',
                _ => '1'
            }
        }

        let mut union_str = String::with_capacity(16);
        for i in 0..8 {
            union_str.push(bit_char(lhs, i));
            union_str.push(bit_char(rhs, i));
        }

        self.automata.set_state(union_str.as_str());
        self.automata.next();

        let result = self.automata.get_state();
        let mut result_str = String::with_capacity(8);

        for i in 0..16 {
            if i % 2 == 1 {
                match result[i] {
                    true => result_str.push('1'),
                    false => result_str.push('0')
                }
            }
        }

        let result = u8::from_str_radix(result_str.as_str(), 2).unwrap();
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::celloperator::*;

    #[test]
    fn cell_operator_works() {
        let mut and_op = CellOperator::new(0xc0);
        assert_eq!(and_op.eval(0x3c, 0x0f), 0x0c);
        let mut not_op = CellOperator::new(0x0f);
        assert_eq!(not_op.eval(0x3c, 0xff), 0xc3);
    }
}