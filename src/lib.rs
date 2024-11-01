use serde::{Deserialize, Serialize};

// 0 == solved
// 1 == clockwise
// 2 == double
// 3 == anti-clockwise
#[derive(Clone, Copy, Debug, PartialEq)]
struct Edge(u8);

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub enum Corner {
    BL,
    BR,
    FR,
    FL,
}

#[derive(Debug, PartialEq)]
enum CornerOrder {
    BL = 0,
    UB = 1,
    BR = 2,
    UR = 3,
    FR = 4,
    UF = 5,
    FL = 6,
    UL = 7,
    DB = 8,
    DR = 9,
    DF = 10,
    DL = 11,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Moves {
    U,
    R,
    D,
    L,
}

pub fn parse(alg: &str) -> Vec<Move> {
    let mut moves = vec![];
    for part in alg.trim().split_whitespace() {
        let mv = match part.chars().next().unwrap() {
            'U' => Moves::U,
            'R' => Moves::R,
            'D' => Moves::D,
            'L' => Moves::L,
            _ => unreachable!(),
        };

        let order = match part.chars().nth(1) {
            Some('2') => 2,
            Some('\'') => 3,
            _ => 1,
        };

        moves.push(Move { mv, order });
    }

    moves
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Move {
    pub mv: Moves,
    pub order: usize,
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{:?}{}",
            self.mv,
            if self.order == 2 {
                "2"
            } else if self.order == 3 {
                "'"
            } else {
                ""
            }
        )
    }
}

// Edge order: B R F L
// Corner order: BL UB BR UR FR UF FL UL DB DR DF DL
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SuperFloppy {
    edges: [Edge; 4],
    corners: [Option<Corner>; 12],
}

impl SuperFloppy {
    pub fn solved() -> Self {
        Self {
            edges: [Edge(0), Edge(0), Edge(0), Edge(0)],
            corners: [
                Some(Corner::BL),
                None,
                Some(Corner::BR),
                None,
                Some(Corner::FR),
                None,
                Some(Corner::FL),
                None,
                None,
                None,
                None,
                None,
            ],
        }
    }

    pub fn is_solved(&self) -> bool {
        self == &Self::solved()
    }

    pub fn random_state() -> Self {
        use rand::prelude::*;
        let mut rng = rand::thread_rng();
        let mut corners = [
            Some(Corner::BL),
            None,
            Some(Corner::BR),
            None,
            Some(Corner::FR),
            None,
            Some(Corner::FL),
            None,
            None,
            None,
            None,
            None,
        ];
        corners.shuffle(&mut rng);
        Self {
            edges: [
                Edge(rng.gen_range(0..=3)),
                Edge(rng.gen_range(0..=3)),
                Edge(rng.gen_range(0..=3)),
                Edge(rng.gen_range(0..=3)),
            ],
            corners,
        }
    }

    pub fn do_moves(&mut self, m: Vec<Move>) {
        for mv in m {
            self.do_move(mv);
        }
    }

    pub fn do_move(&mut self, m: Move) {
        match m.mv {
            Moves::U => {
                for _ in 0..m.order {
                    self.do_U()
                }
            }
            Moves::R => {
                for _ in 0..m.order {
                    self.do_R()
                }
            }
            Moves::D => {
                for _ in 0..m.order {
                    self.do_D()
                }
            }
            Moves::L => {
                for _ in 0..m.order {
                    self.do_L()
                }
            }
        }
    }

    #[allow(non_snake_case)]
    fn do_U(&mut self) {
        self.edges[0] = Edge((self.edges[0].0 + 1) % 4);
        let prev_corners = self.corners;

        self.corners[CornerOrder::BL as usize] = prev_corners[CornerOrder::UB as usize];
        self.corners[CornerOrder::UB as usize] = prev_corners[CornerOrder::BR as usize];
        self.corners[CornerOrder::BR as usize] = prev_corners[CornerOrder::DB as usize];
        self.corners[CornerOrder::DB as usize] = prev_corners[CornerOrder::BL as usize];
    }

    #[allow(non_snake_case)]
    fn do_R(&mut self) {
        self.edges[1] = Edge((self.edges[1].0 + 1) % 4);
        let prev_corners = self.corners;

        self.corners[CornerOrder::BR as usize] = prev_corners[CornerOrder::UR as usize];
        self.corners[CornerOrder::UR as usize] = prev_corners[CornerOrder::FR as usize];
        self.corners[CornerOrder::FR as usize] = prev_corners[CornerOrder::DR as usize];
        self.corners[CornerOrder::DR as usize] = prev_corners[CornerOrder::BR as usize];
    }

    #[allow(non_snake_case)]
    fn do_D(&mut self) {
        self.edges[2] = Edge((self.edges[2].0 + 1) % 4);
        let prev_corners = self.corners;

        self.corners[CornerOrder::FR as usize] = prev_corners[CornerOrder::UF as usize];
        self.corners[CornerOrder::UF as usize] = prev_corners[CornerOrder::FL as usize];
        self.corners[CornerOrder::FL as usize] = prev_corners[CornerOrder::DF as usize];
        self.corners[CornerOrder::DF as usize] = prev_corners[CornerOrder::FR as usize];
    }

    #[allow(non_snake_case)]
    fn do_L(&mut self) {
        self.edges[3] = Edge((self.edges[3].0 + 1) % 4);
        let prev_corners = self.corners;

        self.corners[CornerOrder::BL as usize] = prev_corners[CornerOrder::DL as usize];
        self.corners[CornerOrder::UL as usize] = prev_corners[CornerOrder::BL as usize];
        self.corners[CornerOrder::FL as usize] = prev_corners[CornerOrder::UL as usize];
        self.corners[CornerOrder::DL as usize] = prev_corners[CornerOrder::FL as usize];
    }
}

fn same_axis(mv: Moves, mv2: Moves) -> bool {
    match (mv, mv2) {
        (Moves::U, Moves::D) => true,
        (Moves::D, Moves::U) => true,
        (Moves::R, Moves::L) => true,
        (Moves::L, Moves::R) => true,
        _ => false,
    }
}

fn face_check(m: Moves, solution: &mut Vec<Move>) -> bool {
    if Some(m) == solution.last().map(|a| a.mv) {
        return true;
    }

    if solution.len() > 1 {
        if same_axis(m, solution.last().map(|a| a.mv).unwrap())
            && m == solution[solution.len() - 2].mv
        {
            return true;
        }
    }

    match m {
        Moves::U => if solution.last().map(|a|a.mv) == Some(Moves::D) { return true; },
        Moves::R => if solution.last().map(|a|a.mv) == Some(Moves::L) { return true; },
        _ => {},
    }

    false
}

pub fn alg_string(m: Option<Vec<Move>>) -> String {
    m.map(|a| a.iter().map(|m| format!("{} ", m)).collect())
        .unwrap_or(String::new())
}

fn search_inner(
    f: &SuperFloppy,
    current_depth: usize,
    solution: &mut Vec<Move>,
    pruning_table: &PruningTable,
) -> bool {
    if current_depth == 0 {
        return f.is_solved();
    }

    if pruning_table.contains_key(&f.corners) {
        if current_depth < pruning_table[&f.corners] {
            return false;
        }
    }

    for m in [Moves::U, Moves::R, Moves::D, Moves::L] {
        if face_check(m, solution) {
            continue;
        }
        for o in 1..=3 {
            let mov = Move { mv: m, order: o };
            let mut nf = f.clone();
            nf.do_move(mov);
            solution.push(mov);
            if search_inner(&nf, current_depth - 1, solution, pruning_table) {
                return true;
            }
            solution.pop();
        }
    }

    false
}

pub fn search(
    f: &SuperFloppy,
    max_depth: usize,
    pruning_table: &PruningTable,
) -> Option<Vec<Move>> {
    let mut soln = Vec::with_capacity(max_depth);
    if search_inner(f, max_depth, &mut soln, pruning_table) {
        return Some(soln);
    }
    None
}

pub type PruningTable = std::collections::HashMap<[Option<Corner>; 12], usize>;
pub fn gen_pruning_table(max_depth: usize) -> PruningTable {
    let mut table = PruningTable::new();
    let mut soln = vec![];
    gen_pruning_table_inner(&SuperFloppy::solved(), 0, max_depth, &mut table, &mut soln);
    table
}

pub fn gen_pruning_table_inner(
    f: &SuperFloppy,
    current_depth: usize,
    max_depth: usize,
    table: &mut PruningTable,
    soln: &mut Vec<Move>,
) {
    if current_depth == max_depth {
        return;
    }

    if table.contains_key(&f.corners) && table[&f.corners] > current_depth {
        table.insert(f.corners, current_depth);
    }

    if !table.contains_key(&f.corners) {
        table.insert(f.corners, current_depth);
    }

    for m in [Moves::U, Moves::R, Moves::D, Moves::L] {
        if face_check(m, soln) {
            continue;
        }
        for o in 1..=3 {
            let mov = Move { mv: m, order: o };
            let mut nf = f.clone();
            nf.do_move(mov);
            soln.push(mov);
            gen_pruning_table_inner(&nf, current_depth + 1, max_depth, table, soln);
            soln.pop();
        }
    }
}
