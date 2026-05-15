use std::hint::black_box;
use rand::Rng;
use rand::RngExt;
use helloworld::find_largest_num1;
use helloworld::find_largest_num2;

//todo: we could genercize this
fn gen_rng_vec<T>(len: usize, mut fn_rng: impl FnMut()->T) -> Vec<T> 
        where T: Ord {
    let mut v = Vec::<T>::with_capacity(len);
    for _ in 0..len {
        v.push(fn_rng());
    }
    v
}

fn find_largest(v: &Vec<i32>, fn_find: fn(&Vec<i32>) -> Option<i32>) {
    let largest = black_box(fn_find(black_box(&v)));
    if let Some(largest) = largest {
        println!("Largest: {largest}");
    }
}

const VECSIZE_SMALL: usize = 2 >> 13; //8192
const VECSIZE_MEDIUM: usize = 2 >> 18; //262144
const VECSIZE_LARGE: usize = 2 >> 24; //~16 million

pub fn bench_small_1(c: &mut Criterion) {
    let mut rng = rand::rng();
    let fn_rng = || rng.random::<i32>();
    let v: Vec<i32> = gen_rng_vec(VECSIZE_SMALL, fn_rng);
    c.bench_function("small-1", |b| b.iter(|| find_largest(&v, find_largest_num1)));
}

pub fn bench_medium_1(c: &mut Criterion) {
    let mut rng = rand::rng();
    let fn_rng = || rng.random::<i32>();
    let v: Vec<i32> = gen_rng_vec(VECSIZE_MEDIUM, fn_rng);
    c.bench_function("medium-1", |b| b.iter(|| find_largest(&v, find_largest_num1)));
}

pub fn bench_large_1(c: &mut Criterion) {
    let mut rng = rand::rng();
    let fn_rng = || rng.random::<i32>();
    let v: Vec<i32> = gen_rng_vec(VECSIZE_LARGE, fn_rng);
    c.bench_function("large-1", |b| b.iter(|| find_largest(&v, find_largest_num1)));
}

pub fn bench_small_2(c: &mut Criterion) {
    let mut rng = rand::rng();
    let fn_rng = || rng.random::<i32>();
    let v: Vec<i32> = gen_rng_vec(VECSIZE_SMALL, fn_rng);
    c.bench_function("small-2", |b| b.iter(|| find_largest(&v, find_largest_num2)));
}

pub fn bench_medium_2(c: &mut Criterion) {
    let mut rng = rand::rng();
    let fn_rng = || rng.random::<i32>();
    let v: Vec<i32> = gen_rng_vec(VECSIZE_MEDIUM, fn_rng);
    c.bench_function("medium-2", |b| b.iter(|| find_largest(&v, find_largest_num2)));
}

pub fn bench_large_2(c: &mut Criterion) {
    let mut rng = rand::rng();
    let fn_rng = || rng.random::<i32>();
    let v: Vec<i32> = gen_rng_vec(VECSIZE_LARGE, fn_rng);
    c.bench_function("large-2", |b| b.iter(|| find_largest(&v, find_largest_num2)));
}

use criterion::{criterion_group, criterion_main, Criterion};
criterion_group!(benchmarks, bench_small_1, bench_medium_1, bench_large_1);
criterion_group!(benchmarks2, bench_small_2, bench_medium_2, bench_large_2);
criterion_main!(benchmarks, benchmarks2);
