use aoc::{
    Solution,
    implementation::{
        day_1::DayOneSolution, day_2::DayTwoSolution, day_3::DayThreeSolution,
        day_4::DayFourSolution, day_5::DayFiveSolution, day_6::DaySixSolution,
        day_7::DaySevenSolution, day_8::DayEightSolution, day_9::DayNineSolution,
        day_10::DayTenSolution, day_11::DayElevenSolution,
    },
};

use criterion::{Criterion, criterion_group, criterion_main};

fn benchmark_aoc_day_one(c: &mut Criterion) {
    let day_one = DayOneSolution::new();
    let mut group = c.benchmark_group("1");

    group.bench_function("Solution one", |b| b.iter(|| day_one.part_one()));
    group.bench_function("Solution two", |b| b.iter(|| day_one.part_two()));
    group.finish();
}

fn benchmark_aoc_day_two(c: &mut Criterion) {
    let day_two = DayTwoSolution::new();
    let mut group = c.benchmark_group("2");

    group.bench_function("Solution one", |b| b.iter(|| day_two.part_one()));
    group.bench_function("Solution two", |b| b.iter(|| day_two.part_two()));
    group.finish();
}

fn benchmark_aoc_day_three(c: &mut Criterion) {
    let day_three = DayThreeSolution::new();
    let mut group = c.benchmark_group("3");

    group.bench_function("Solution one", |b| b.iter(|| day_three.part_one()));
    group.bench_function("Solution two", |b| b.iter(|| day_three.part_two()));
    group.finish();
}

fn benchmark_aoc_day_four(c: &mut Criterion) {
    let day_four = DayFourSolution::new();
    let mut group = c.benchmark_group("4");

    group.bench_function("Solution one", |b| b.iter(|| day_four.part_one()));
    group.bench_function("Solution two", |b| b.iter(|| day_four.part_two()));
    group.finish();
}

fn benchmark_aoc_day_five(c: &mut Criterion) {
    let day_five = DayFiveSolution::new();
    let mut group = c.benchmark_group("5");

    group.bench_function("Solution one", |b| b.iter(|| day_five.part_one()));
    group.bench_function("Solution two", |b| b.iter(|| day_five.part_two()));
    group.finish();
}

fn benchmark_aoc_day_six(c: &mut Criterion) {
    let day_six = DaySixSolution::new();
    let mut group = c.benchmark_group("6");

    group.bench_function("Solution one", |b| b.iter(|| day_six.part_one()));
    group.bench_function("Solution two", |b| b.iter(|| day_six.part_two()));
    group.finish();
}

fn benchmark_aoc_day_seven(c: &mut Criterion) {
    let day_seven = DaySevenSolution::new();
    let mut group = c.benchmark_group("7");

    group.bench_function("Solution one", |b| b.iter(|| day_seven.part_one()));
    group.bench_function("Solution two", |b| b.iter(|| day_seven.part_two()));
    group.finish();
}

fn benchmark_aoc_day_eight(c: &mut Criterion) {
    let day_eight = DayEightSolution::new();
    let mut group = c.benchmark_group("8");

    group.bench_function("Solution one", |b| b.iter(|| day_eight.part_one()));
    group.bench_function("Solution two", |b| b.iter(|| day_eight.part_two()));
    group.finish();
}

fn benchmark_aoc_day_nine(c: &mut Criterion) {
    let day_nine = DayNineSolution::new();
    let mut group = c.benchmark_group("9");

    group.bench_function("Solution one", |b| b.iter(|| day_nine.part_one()));
    group.bench_function("Solution two", |b| b.iter(|| day_nine.part_two()));
    group.finish();
}

fn benchmark_aoc_day_ten(c: &mut Criterion) {
    let day_ten = DayTenSolution::new();
    let mut group = c.benchmark_group("10");

    group.bench_function("Solution one", |b| b.iter(|| day_ten.part_one()));
    group.bench_function("Solution two", |b| b.iter(|| day_ten.part_two()));
    group.finish();
}

fn benchmark_aoc_day_eleven(c: &mut Criterion) {
    let day_eleven = DayElevenSolution::new();
    let mut group = c.benchmark_group("11");

    group.bench_function("Solution one", |b| b.iter(|| day_eleven.part_one()));
    group.bench_function("Solution two", |b| b.iter(|| day_eleven.part_two()));
    group.finish();
}

criterion_group!(
    benches,
    benchmark_aoc_day_one,
    benchmark_aoc_day_two,
    benchmark_aoc_day_three,
    benchmark_aoc_day_four,
    benchmark_aoc_day_five,
    benchmark_aoc_day_six,
    benchmark_aoc_day_seven,
    benchmark_aoc_day_eight,
    benchmark_aoc_day_nine,
    benchmark_aoc_day_ten,
    benchmark_aoc_day_eleven,
);
criterion_main!(benches);
