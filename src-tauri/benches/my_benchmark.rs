use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cherris::{engine::Engine, *};

fn count_moves_for_depth(depth: u8) -> usize {
    let mut game = Game::init();
    if depth == 0 {
        return 1;
    }
    let mut count = 0;

    let moves = game.get_all_moves_for_color(game.white_turn);
    for mv in moves {
        game.play_move_ob(mv);
        count += count_moves_for_depth(depth - 1);
        game.undo_move();
    }
    count
}

fn bench_best_move_gen(depth: u8) {
    let mut engine = Engine::init();

    let _best_move = engine.get_best_move(depth);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Move gen depth 2", |b| b.iter(|| count_moves_for_depth(black_box(2))));
    c.bench_function("Move gen depth 3", |b| b.iter(|| count_moves_for_depth(black_box(3))));
    c.bench_function("Best move depth 3", |b| b.iter(|| bench_best_move_gen(black_box(3))));
    c.bench_function("Best move depth 2", |b| b.iter(|| bench_best_move_gen(black_box(2))));

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);