use criterion::{black_box, criterion_group, criterion_main, Criterion};
use advent2024::day1::{part1, part2};

const INPUT: &str = include_str!("../input/2024/day1.txt");

pub fn day1(c: &mut Criterion) {
    c.bench_function("day1 part1", |b| b.iter(|| part1(black_box(INPUT))));
    c.bench_function("day1 part2", |b| b.iter(|| part2(black_box(INPUT))));
}

criterion_group!(benches, day1);
criterion_main!(benches);