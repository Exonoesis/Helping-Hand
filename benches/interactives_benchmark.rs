use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use std::path::PathBuf;

use helping_hand::visuals::map::*;

fn create_interactive_collection_from_dimensions(
    width: usize,
    height: usize,
) -> InteractiveCollection {
    let mut interactive_markers = Vec::new();
    for x in 1..=width {
        for y in 1..=height {
            let dimensions = PxDimensions::new(64, 64);
            let grid_coordinate = XyzCords::new(x, y, 0);
            let interactive_type = InteractiveType::Transition(PathBuf::from(""));
            let interactive_marker =
                InteractiveMarker::new(grid_coordinate, dimensions, interactive_type);

            interactive_markers.push(interactive_marker);
        }
    }

    InteractiveCollection::from_markers(interactive_markers)
}

fn benchmark_interactives_on_small_map(c: &mut Criterion) {
    let interactives = create_interactive_collection_from_dimensions(5, 5);

    // Equivalent to an assert statement, but with performance numbers.
    c.bench_function("Look for Position 5, 5 in Small sized map", |benchmarker| {
        // This runs the function 100 times to generate performance numbers
        // averaged based on all of these cases.
        benchmarker
            .iter(|| interactives.get_marker_from_position(black_box(&XyzCords::new(5, 5, 0))))
    });
}

fn benchmark_interactives_on_medium_map(c: &mut Criterion) {
    let interactives = create_interactive_collection_from_dimensions(20, 20);

    // Equivalent to an assert statement, but with performance numbers.
    c.bench_function(
        "Look for Position 20, 20 in Medium sized map",
        |benchmarker| {
            // This runs the function 100 times to generate performance numbers
            // averaged based on all of these cases.
            benchmarker.iter(|| {
                interactives.get_marker_from_position(black_box(&XyzCords::new(20, 20, 0)))
            })
        },
    );
}

fn benchmark_interactives_on_large_map(c: &mut Criterion) {
    let interactives = create_interactive_collection_from_dimensions(50, 50);

    // Equivalent to an assert statement, but with performance numbers.
    c.bench_function(
        "Look for Position 50, 50 in Large sized map",
        |benchmarker| {
            // This runs the function 100 times to generate performance numbers
            // averaged based on all of these cases.
            benchmarker.iter(|| {
                interactives.get_marker_from_position(black_box(&XyzCords::new(50, 50, 0)))
            })
        },
    );
}

fn benchmark_interactives_on_jumbo_map(c: &mut Criterion) {
    let interactives = create_interactive_collection_from_dimensions(1280, 1280);

    // Equivalent to an assert statement, but with performance numbers.
    c.bench_function(
        "Look for Position 1280, 1280 in Jumbo sized map",
        |benchmarker| {
            // This runs the function 100 times to generate performance numbers
            // averaged based on all of these cases.
            benchmarker.iter(|| {
                interactives.get_marker_from_position(black_box(&XyzCords::new(1280, 1280, 0)))
            })
        },
    );
}

criterion_group!(
    benches,
    benchmark_interactives_on_small_map,
    benchmark_interactives_on_medium_map,
    benchmark_interactives_on_large_map,
    benchmark_interactives_on_jumbo_map,
);
criterion_main!(benches);
