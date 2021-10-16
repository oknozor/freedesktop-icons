use criterion::{criterion_group, criterion_main, Criterion};
use freedesktop_icon::theme::IconTheme;

fn icon_lookup(c: &mut Criterion) {
    c.bench_function("lookup firefox icon", |b| {
        b.iter(|| {
            freedesktop_icon::lookup("firefox")
                .theme("hicolor")
                .size(24)
                .scale(1)
                .execute()
        })
    });
}

fn theme_deserialize(c: &mut Criterion) {
    c.bench_function("deserialize hicolor", |b| {
        b.iter(|| IconTheme::from_path("/usr/share/icons/hicolor/index.theme"))
    });
}

criterion_group!(benches, icon_lookup, theme_deserialize);
criterion_main!(benches);
