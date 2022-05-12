use criterion::{black_box, criterion_group, criterion_main, Criterion};
use freedesktop_icons::lookup;

pub fn simple_lookup(c: &mut Criterion) {
    c.bench_function("lookup firefox", |b| {
        b.iter(|| lookup("firefox").find_one())
    });
}

pub fn simple_lookup_linicon(c: &mut Criterion) {
    c.bench_function("lookup firefox linicon", |b| {
        b.iter(|| linicon::lookup_icon("firefox").next().unwrap().unwrap())
    });
}

criterion_group!(benches, simple_lookup, simple_lookup_linicon);
criterion_main!(benches);
