use criterion::{criterion_group, criterion_main, Criterion};
use rust_embed_utils::{get_files, get_files_old, PathMatcher};

fn criterion_benchmark(c: &mut Criterion) {
  let incs = vec![];
  let exs = vec!["node_modules/*"];
  let matcher = PathMatcher::new(&incs, &exs);
  // TODO: we need a real path here
  // better be an larger one
  let base = String::from("path/to/folder");
  let mut group = c.benchmark_group("walk_dir");
  group.sample_size(30);
  group.bench_function("walk_dir", |b| b.iter(|| get_files(base.clone(), matcher.clone()).count()));
  group.bench_function("walk_dir_old", |b| b.iter(|| get_files_old(base.clone(), matcher.clone()).count()));
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
