mod solver;

use solver::*;
use std::io::Read;

fn ida(f: &SuperFloppy, max_depth: usize, table: &PruningTable, start_depth: usize) -> Vec<Move> {
    let start = std::time::Instant::now();
    for i in start_depth..max_depth {
        println!("Searching depth {i}...");
        let moves = search(f, i, table);
        if let Some(moves) = moves {
            let end = std::time::Instant::now();
            println!("Searching took {:?}", end - start);
            return moves;
        }
    }
    let end = std::time::Instant::now();
    println!("Searching took {:?}", end - start);

    vec![]
}

fn save_pruning_table() -> PruningTable {
    use std::io::Write;
    println!("Generating table");
    let start = std::time::Instant::now();
    let table = gen_pruning_table(9);
    let end = std::time::Instant::now();
    println!("Generating table took {:?}", end - start);
    let encoded = bincode::serialize(&table).unwrap();
    let mut file = std::fs::File::create("prune.table").unwrap();
    file.write_all(&encoded).unwrap();
    table
}

fn get_pruning_table() -> PruningTable {
    match std::fs::File::open("prune.table") {
        Ok(mut file) => {
            let mut buffer = vec![];
            file.read_to_end(&mut buffer).unwrap();
            bincode::deserialize(&buffer).unwrap()
        }
        _ => save_pruning_table(),
    }
}

fn main() {
    let table = get_pruning_table();
    println!("Table loaded: {}", table.len());
    let (start_depth, f) = if std::env::args().nth(1).as_deref() == Some("--scramble") {
        (9, SuperFloppy::random_state())
    } else {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let alg = parse(&buffer);
        let mut f = SuperFloppy::solved();
        f.do_moves(alg);
        (0, f)
    };
    println!("Solution: {}", alg_string(Some(ida(&f, 20, &table, start_depth))));
}
