use solver::*;
use std::io::Read;

fn alg_string(m: Option<Vec<Move>>) -> String {
    m.map(|a| a.iter().map(|m| format!("{} ", m)).collect())
        .unwrap_or(String::new())
}

fn ida(f: &SuperFloppy, max_depth: usize, table: &PruningTable) -> Vec<Move> {
    let start = std::time::Instant::now();
    for i in 0..max_depth {
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
            file.read_to_end(&mut buffer);
            bincode::deserialize(&buffer).unwrap()
        },
        _ => save_pruning_table(),
    }
}

fn main() {
    let table = get_pruning_table();
    println!("Table loaded: {}", table.len());
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer);
    let alg = parse(&buffer);
    let mut f = SuperFloppy::solved();
    f.do_moves(alg);
    println!("{f:?} solved {}", f.is_solved());
    println!("Solution: {}", alg_string(Some(ida(&f, 20, &table))));
}
