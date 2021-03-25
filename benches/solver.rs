use criterion::*;

use connect_four::board::FieldType;
use connect_four::logic::{ABSolver, BOARD_HEIGHT, BOARD_WIDTH, GameBoard};

const p: Option<FieldType> = Some(FieldType::Player);
const c: Option<FieldType> = Some(FieldType::Opponent);
const n: Option<FieldType> = None::<FieldType>;
const fields_easy01: [[Option<FieldType>; BOARD_HEIGHT]; BOARD_WIDTH]
= [
[n,n,n,n,n,n,],
[c,c,c,n,n,n,],
[p,n,n,n,n,n,],
[p,n,n,n,n,n,],
[c,n,n,n,n,n,],
[n,n,n,n,n,n,],
[n,n,n,n,n,n,],
];
const fields_easy02: [[Option<FieldType>; BOARD_HEIGHT]; BOARD_WIDTH]
= [
[n,n,n,n,n,n,],
[c,p,p,n,n,n,],
[p,c,n,n,n,n,],
[p,c,n,n,n,n,],
[c,p,n,n,n,n,],
[n,n,n,n,n,n,],
[n,n,n,n,n,n,],
];
const fields_endgame01: [[Option<FieldType>; BOARD_HEIGHT]; BOARD_WIDTH]
= [
[c,p,p,c,n,n,],
[c,p,p,p,n,n,],
[p,c,c,p,n,n,],
[p,c,p,n,n,n,],
[c,p,c,n,n,n,],
[c,p,c,n,n,n,],
[p,c,p,c,p,n,],
];
const fields_beginning01: [[Option<FieldType>; BOARD_HEIGHT]; BOARD_WIDTH]
= [
    [n,n,n,n,n,n,],
    [n,n,n,n,n,n,],
    [n,n,n,n,n,n,],
    [p,n,n,n,n,n,],
    [n,n,n,n,n,n,],
    [n,n,n,n,n,n,],
    [n,n,n,n,n,n,],
];

pub fn bench_solve_d7(crit: &mut Criterion) {


    let board_easy01 = GameBoard::new(fields_easy01);
    let board_easy02 = GameBoard::new(fields_easy02);
    let board_endgame01 = GameBoard::new(fields_endgame01);
    crit.bench_function("easy01_d7", |b| b.iter(|| ABSolver::solve(&black_box(board_easy01), 7, FieldType::Opponent)));
    crit.bench_function("easy02_d7", |b| b.iter(|| ABSolver::solve(&black_box(board_easy02), 7, FieldType::Opponent)));
    crit.bench_function("endgame01_d7", |b| b.iter(|| ABSolver::solve(&black_box(board_endgame01), 7, FieldType::Opponent)));

    // let best_move = ABSolver::solve(&board, 7, FieldType::Computer);
    // println!("Score: {}, Move: {}", best_move.score, best_move.move_row);
}

pub fn bench_solve_d9(crit: &mut Criterion) {

    let board_easy01 = GameBoard::new(fields_easy01);
    let board_easy02 = GameBoard::new(fields_easy02);
    let board_endgame01 = GameBoard::new(fields_endgame01);
    crit.bench_function("easy01_d9", |b| b.iter(|| ABSolver::solve(&black_box(board_easy01), 9, FieldType::Opponent)));
    crit.bench_function("easy02_d9", |b| b.iter(|| ABSolver::solve(&black_box(board_easy02), 9, FieldType::Opponent)));
    crit.bench_function("endgame01_d9", |b| b.iter(|| ABSolver::solve(&black_box(board_endgame01), 9, FieldType::Opponent)));

    // let best_move = ABSolver::solve(&board, 7, FieldType::Computer);
    // println!("Score: {}, Move: {}", best_move.score, best_move.move_row);
}
pub fn bench_solve_d13(crit: &mut Criterion) {

    let board_easy01 = GameBoard::new(fields_easy01);
    let board_easy02 = GameBoard::new(fields_easy02);
    let board_endgame01 = GameBoard::new(fields_endgame01);
    let board_beginning01 = GameBoard::new(fields_beginning01);
    // crit.bench_function("easy01_d9", |b| b.iter(|| ABSolver::solve(&black_box(board_easy01), 9, FieldType::Computer)));
    // crit.bench_function("easy02_d9", |b| b.iter(|| ABSolver::solve(&black_box(board_easy02), 9, FieldType::Computer)));
    // crit.bench_function("endgame01_d9", |b| b.iter(|| ABSolver::solve(&black_box(board_endgame01), 9, FieldType::Computer)));
    crit.bench_function("beginning01_d13", |b| b.iter(|| ABSolver::solve(&black_box(board_beginning01), 13, FieldType::Opponent)));

    // let best_move = ABSolver::solve(&board, 7, FieldType::Computer);
    // println!("Score: {}, Move: {}", best_move.score, best_move.move_row);
}

pub fn bench_solve_mtdf_d13(crit: &mut Criterion) {
    let board_easy01 = GameBoard::new(fields_easy01);
    let board_easy02 = GameBoard::new(fields_easy02);
    let board_endgame01 = GameBoard::new(fields_endgame01);
    let board_beginning01 = GameBoard::new(fields_beginning01);
    // crit.bench_function("easy01_d9", |b| b.iter(|| ABSolver::solve(&black_box(board_easy01), 9, FieldType::Computer)));
    // crit.bench_function("easy02_d9", |b| b.iter(|| ABSolver::solve(&black_box(board_easy02), 9, FieldType::Computer)));
    // crit.bench_function("endgame01_d9", |b| b.iter(|| ABSolver::solve(&black_box(board_endgame01), 9, FieldType::Computer)));
    crit.bench_function("beginning01_d13_mtdf", |b| b.iter(|| ABSolver::solve_mtdf(&black_box(board_beginning01), 13, FieldType::Opponent)));
    // crit.bench_function("beginning01_d13", |b| b.iter(|| ABSolver::solve(&black_box(board_beginning01), 13, FieldType::Computer)));
}

pub fn bench_solve_mtdf_d13_guessed(crit: &mut Criterion) {
    let board_easy01 = GameBoard::new(fields_easy01);
    let board_easy02 = GameBoard::new(fields_easy02);
    let board_endgame01 = GameBoard::new(fields_endgame01);
    let board_beginning01 = GameBoard::new(fields_beginning01);
    // crit.bench_function("easy01_d9", |b| b.iter(|| ABSolver::solve(&black_box(board_easy01), 9, FieldType::Computer)));
    // crit.bench_function("easy02_d9", |b| b.iter(|| ABSolver::solve(&black_box(board_easy02), 9, FieldType::Computer)));
    // crit.bench_function("endgame01_d9", |b| b.iter(|| ABSolver::solve(&black_box(board_endgame01), 9, FieldType::Computer)));
    crit.bench_function("beginning01_d13_mtdf_guessed", |b| b.iter(|| ABSolver::solve_mtdf_guessed(&black_box(board_beginning01), 13, FieldType::Opponent, 10)));
    // crit.bench_function("beginning01_d13", |b| b.iter(|| ABSolver::solve(&black_box(board_beginning01), 13, FieldType::Computer)));
}
pub fn bench_solve_mtdf_d15(crit: &mut Criterion) {
    // let board_easy01 = GameBoard::new(fields_easy01);
    // let board_easy02 = GameBoard::new(fields_easy02);
    // let board_endgame01 = GameBoard::new(fields_endgame01);
    let board_beginning01 = GameBoard::new(fields_beginning01);
    // crit.bench_function("easy01_d9", |b| b.iter(|| ABSolver::solve(&black_box(board_easy01), 9, FieldType::Computer)));
    // crit.bench_function("easy02_d9", |b| b.iter(|| ABSolver::solve(&black_box(board_easy02), 9, FieldType::Computer)));
    // crit.bench_function("endgame01_d9", |b| b.iter(|| ABSolver::solve(&black_box(board_endgame01), 9, FieldType::Computer)));
    crit.bench_function("beginning01_d15_mtdf", |b| b.iter(|| ABSolver::solve(&black_box(board_beginning01), 15, FieldType::Opponent)));
    crit.bench_function("beginning01_d15", |b| b.iter(|| ABSolver::solve(&black_box(board_beginning01), 15, FieldType::Opponent)));
}
criterion_group!(benches_d7, bench_solve_d7);
criterion_group!(benches_d9, bench_solve_d9);
criterion_group!(benches_d13, bench_solve_d13);
criterion_group!(benches_mtdf_13, bench_solve_mtdf_d13);
criterion_group!(benches_mtdf_13_guessed, bench_solve_mtdf_d13_guessed);
criterion_group!(
    name = benches_mtdf_15;
    config = Criterion::default().sample_size(10);
    targets = bench_solve_mtdf_d15
);
criterion_main!(benches_mtdf_13_guessed);