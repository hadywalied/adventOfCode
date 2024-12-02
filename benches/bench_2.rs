use advent2024::day2::{part1, part2};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const INPUT: &str = include_str!("../input/2024/day2.txt");

pub fn day2(c: &mut Criterion) {
    c.bench_function("day2 part1", |b| b.iter(|| part1(black_box(INPUT))));
    c.bench_function("day2 part2", |b| b.iter(|| part2(black_box(INPUT))));
}

criterion_group!(benches, day2);
criterion_main!(benches);