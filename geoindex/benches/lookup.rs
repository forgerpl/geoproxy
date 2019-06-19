use criterion::{black_box, criterion_group, criterion_main, Criterion, ParameterizedBenchmark};
use geo::{polygon, Point, Polygon};
use geoindex::{GeoIndex, IndexDefinition};

fn triangle(scale: f32, tx: f32, ty: f32) -> Polygon<f32> {
    // base bounding rect: 0-6
    const X0: f32 = 3f32;
    const Y0: f32 = 0f32;
    const X1: f32 = 0f32;
    const Y1: f32 = 3f32;
    const X2: f32 = 6f32;
    const Y2: f32 = 3f32;

    polygon![
        (x: (X0 + tx) * scale, y: (Y0 + ty) * scale),
        (x: (X1 + tx) * scale, y: (Y1 + ty) * scale),
        (x: (X2 + tx) * scale, y: (Y2 + ty) * scale),
    ]
}

fn prepare_samples(cols: usize, rows: usize) -> Vec<Point<f32>> {
    // use triangles

    (0..rows)
        .filter(|row| row % 4 == 0)
        .flat_map(|row| {
            (0..cols).filter(|col| col % 4 == 0).map(move |col| {
                let scale = match () {
                    _ if row % 3 == 0 && col % 3 == 0 => 2.0,
                    _ if row % 3 != 0 && col % 3 == 0 => 0.5,
                    _ if row % 3 == 0 && col % 3 != 0 => 1.5,
                    _ => 1.0,
                };

                Point::new(
                    ((col * 6) as f32 + 2.5) * scale,
                    ((row * 6) as f32 + 2.4) * scale,
                )
            })
        })
        .collect()
}

fn prepare_dataset(cols: usize, rows: usize) -> GeoIndex<usize, f32> {
    let defs = (0..rows)
        .flat_map(|row| {
            (0..cols).map(move |col| {
                let scale = match () {
                    _ if row % 3 == 0 && col % 3 == 0 => 2.0,
                    _ if row % 3 != 0 && col % 3 == 0 => 0.5,
                    _ if row % 3 == 0 && col % 3 != 0 => 1.5,
                    _ => 1.0,
                };

                let value_index = 1 + cols * row + col;

                (
                    vec![triangle(scale, (col * 6) as f32, (row * 6) as f32)],
                    value_index,
                )
            })
        })
        .collect::<IndexDefinition<_, _>>();

    GeoIndex::new(defs, 0)
}

fn lookup(needle: &Point<f32>, haystack: &GeoIndex<usize, f32>) -> usize {
    *haystack.lookup_coords(needle)
}

fn bench_lookup(c: &mut Criterion) {
    c.bench(
        "Lookup",
        ParameterizedBenchmark::new(
            "RTree",
            |b, (rows, cols)| {
                let data = prepare_dataset(*rows, *cols);
                let mut s = prepare_samples(*rows, *cols).into_iter().cycle();

                b.iter(move || black_box(lookup(&s.next().unwrap(), &data)))
            },
            vec![(10, 10), (100, 100), (200, 200)],
        ),
    );
}

criterion_group!(benches, bench_lookup);
criterion_main!(benches);
