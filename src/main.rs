use cell_automata_1d::CellAutomata1D;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 4 {
        panic!("You must give 3 arguments to program");
    }

    let rule_number: u8 = args[2].parse().expect("You must enter number 0..255 as argument #2");
    let gens_count: usize = args[3].parse().expect("You must enter positive number of gens as argument #3");

    let mut automata = CellAutomata1D::new(
        &args[1], 
        rule_number
    );

    for _ in 0..gens_count {
        println!("|{}|", automata.string_state());
        automata.next();
    }
}
