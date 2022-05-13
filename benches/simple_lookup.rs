use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use freedesktop_icons::lookup;
use gtk4::{IconLookupFlags, IconTheme, TextDirection};

pub fn simple_bench(c: &mut Criterion) {
    c.bench_function("firefox lookup", |b| {
        b.iter(|| lookup("firefox").with_theme("Papirus").with_cache().find())
    });
}

pub fn bench_lookups(c: &mut Criterion) {
    let mut group = c.benchmark_group("ComparisonsLookups");

    let args = [
        "user-home", // (Best case) An icon that can be found in the current theme
        "video-single-display-symbolic", // An icon that can be found in the parent theme
        "firefox",   // An icon that can be found in the hicolor default theme
        "archlinux-logo", // An icon that resides in /usr/share/pixmap
        "not-found", // (Worst case) An icon that does not exist
    ];

    for arg in args {
        group.bench_with_input(
            BenchmarkId::new("freedesktop-icons lookup", arg),
            arg,
            |b, arg| {
                b.iter(|| lookup(arg).with_theme("Arc").find());
            },
        );

        group.bench_with_input(BenchmarkId::new("lookup with cache", arg), arg, |b, arg| {
            b.iter(|| lookup(arg).with_theme("Arc").with_cache().find());
        });

        group.bench_with_input(BenchmarkId::new("lookup linicon", arg), arg, |b, arg| {
            b.iter(|| linicon::lookup_icon(arg).from_theme("Arc").next());
        });

        group.bench_with_input(BenchmarkId::new("lookup gtk", arg), arg, |b, arg| {
            gtk4::init().unwrap();
            let theme = IconTheme::new();
            b.iter(|| {
                theme.lookup_icon(
                    arg,
                    &[],
                    24,
                    1,
                    TextDirection::None,
                    IconLookupFlags::empty(),
                )
            });
        });
    }

    group.finish();
}

criterion_group!(benches, simple_bench, bench_lookups);
criterion_main!(benches);
