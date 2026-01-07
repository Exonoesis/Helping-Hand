use criterion::{criterion_group, criterion_main, Criterion};
use helping_hand::map::{movement::collision::CollisionCollection, GridCords3D};
use std::hint::black_box;

fn create_collision_collection_from_dimensions(width: usize, height: usize) -> CollisionCollection {
    let mut collision_collection = CollisionCollection::new();

    for x in 1..=width {
        for y in 1..=height {
            let grid_coordinate = GridCords3D::new(x, y, 0);

            collision_collection.add(&grid_coordinate);
        }
    }

    collision_collection
}

fn benchmark_collisions_on_small_map(c: &mut Criterion) {
    let collision_checker = create_collision_collection_from_dimensions(5, 5);

    // Equivalent to an assert statement, but with performance numbers.
    c.bench_function("Look for Tile 5, 5 in Small sized map", |benchmarker| {
        // This runs the function 100 times to generate performance numbers
        // averaged based on all of these cases.
        benchmarker.iter(|| collision_checker.has(black_box(&GridCords3D::new(5, 5, 0))))
    });
}

fn benchmark_collisions_on_medium_map(c: &mut Criterion) {
    let collision_checker = create_collision_collection_from_dimensions(20, 20);

    // Equivalent to an assert statement, but with performance numbers.
    c.bench_function("Look for Tile 20, 20 in Medium sized map", |benchmarker| {
        // This runs the function 100 times to generate performance numbers
        // averaged based on all of these cases.
        //
        // NOTE: We're calling black_box because we don't want any optimizations to be
        // done from Rust. Without the optimizations, that makes the run as "fair" as possible.
        benchmarker.iter(|| collision_checker.has(black_box(&GridCords3D::new(20, 20, 0))))
    });
}

fn benchmark_collisions_on_large_map(c: &mut Criterion) {
    let collision_checker = create_collision_collection_from_dimensions(50, 50);

    // Equivalent to an assert statement, but with performance numbers.
    c.bench_function("Look for Tile 50, 50 in Large sized map", |benchmarker| {
        // This runs the function 100 times to generate performance numbers
        // averaged based on all of these cases.
        //
        // NOTE: We're calling black_box because we don't want any optimizations to be
        // done from Rust. Without the optimizations, that makes the run as "fair" as possible.
        benchmarker.iter(|| collision_checker.has(black_box(&GridCords3D::new(50, 50, 0))))
    });
}

fn benchmark_collisions_on_jumbo_map(c: &mut Criterion) {
    let collision_checker = create_collision_collection_from_dimensions(1280, 1280);

    // Equivalent to an assert statement, but with performance numbers.
    c.bench_function(
        "Look for Tile 1280, 1280 in Jumbo sized map",
        |benchmarker| {
            // This runs the function 100 times to generate performance numbers
            // averaged based on all of these cases.
            //
            // NOTE: We're calling black_box because we don't want any optimizations to be
            // done from Rust. Without the optimizations, that makes the run as "fair" as possible.
            benchmarker.iter(|| collision_checker.has(black_box(&GridCords3D::new(1280, 1280, 0))))
        },
    );
}

criterion_group!(
    benches,
    benchmark_collisions_on_small_map,
    benchmark_collisions_on_medium_map,
    benchmark_collisions_on_large_map,
    benchmark_collisions_on_jumbo_map,
);
criterion_main!(benches);
