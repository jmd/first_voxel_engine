use criterion::{criterion_group, criterion_main, Criterion};
use voxel_lib::voxel_tools::{ chunks::Chunks, mesh_builder::build_chunk_mesh };
use cgmath::Vector3;

const WINDOW_SIZE: i32 = 4;

fn bench_build_chunk_data(chunks: &mut Chunks) {
    for x in 0..WINDOW_SIZE {
        for y in 0..WINDOW_SIZE {
            for z in 0..WINDOW_SIZE {
                chunks.build_chunk_data(Vector3::new(x, y, z));
            }
        }
    }
}

fn bench_build_chunk_mesh(chunks: &mut Chunks) {
    for x in 0..WINDOW_SIZE {
        for y in 0..WINDOW_SIZE {
            for z in 0..WINDOW_SIZE {
                build_chunk_mesh(chunks, &Vector3::new(x, y, z));
            }
        }
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut chunks = Chunks::new();
    c.bench_function("build_chunk_data", |b| b.iter(|| bench_build_chunk_data(&mut chunks)));
    c.bench_function("build_chunk_mesh", |b| b.iter(|| bench_build_chunk_mesh(&mut chunks)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);