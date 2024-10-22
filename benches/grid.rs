use connect::*;
use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use rand::{rngs::SmallRng, Rng, SeedableRng};

const SEED: u64 = 123456789;

fn add_random_moves_to_grid<G: Grid>(rng: &mut SmallRng, mut g: G, total_moves: usize) -> G {
    let original = g.clone();
    if total_moves == 0 {
        return g;
    }
    for _ in 0..total_moves {
        match g.drop(rng.gen_range(0..G::WIDTH)) {
            Ok(Status::OnGoing) => {}
            Err(_) => {}
            // If we reached a terminal state, reset and try again recursively.
            // Technically if total_moves is high enough this could cause an infinite recursive loop,
            // as it isn't possible to make more than 42 moves without reaching a win/draw
            Ok(Status::Win(_)) => {
                return add_random_moves_to_grid(rng, original.clone(), total_moves)
            }
            Ok(Status::Draw) => {
                return add_random_moves_to_grid(rng, original.clone(), total_moves)
            }
        }
    }
    g
}

pub fn bench_grid_drop(c: &mut Criterion) {
    let mut group = c.benchmark_group("Grid::drop");
    for total_moves in (0..30).step_by(2) {
        {
            let rng = &mut SmallRng::seed_from_u64(SEED);
            group.bench_with_input(
                BenchmarkId::new("GenericGrid", total_moves),
                &total_moves,
                |b, total_moves| {
                    b.iter_batched(
                        || {
                            (
                                add_random_moves_to_grid(
                                    rng,
                                    GenericGrid::<7, 6, 4>::new(),
                                    *total_moves as usize,
                                ),
                                rng.gen_range(0..7),
                            )
                        },
                        |(mut g, i)| black_box(g.drop(i)),
                        BatchSize::SmallInput,
                    )
                },
            );
        }
        {
            let rng = &mut SmallRng::seed_from_u64(SEED);
            group.bench_with_input(
                BenchmarkId::new("BitboardGrid", total_moves),
                &total_moves,
                |b, total_moves| {
                    b.iter_batched(
                        || {
                            (
                                add_random_moves_to_grid(
                                    rng,
                                    BitboardGrid::new(),
                                    *total_moves as usize,
                                ),
                                rng.gen_range(0..7),
                            )
                        },
                        |(mut g, i)| black_box(g.drop(i)),
                        BatchSize::SmallInput,
                    )
                },
            );
        }
    }
    group.finish();
}

pub fn bench_grid_valid_moves(c: &mut Criterion) {
    let mut group = c.benchmark_group("Grid::valid_moves");
    for total_moves in (0..30).step_by(2) {
        group.bench_with_input(
            BenchmarkId::new("GenericGrid", total_moves),
            &total_moves,
            |b, total_moves| {
                b.iter_batched(
                    || {
                        add_random_moves_to_grid(
                            &mut SmallRng::seed_from_u64(SEED),
                            GenericGrid::<7, 6, 4>::new(),
                            *total_moves as usize,
                        )
                    },
                    |g| black_box(g.valid_moves()),
                    BatchSize::SmallInput,
                );
            },
        );
        group.bench_with_input(
            BenchmarkId::new("BitboardGrid", total_moves),
            &total_moves,
            |b, total_moves| {
                b.iter_batched(
                    || {
                        add_random_moves_to_grid(
                            &mut SmallRng::seed_from_u64(SEED),
                            BitboardGrid::new(),
                            *total_moves as usize,
                        )
                    },
                    |g| black_box(g.valid_moves()),
                    BatchSize::SmallInput,
                );
            },
        );
    }
    group.finish();
}

criterion_group!(benches, bench_grid_drop, bench_grid_valid_moves);
criterion_main!(benches);
