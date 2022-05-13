use criterion::{
    black_box, criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion,
    PlotConfiguration,
};
use freedesktop_icons::lookup;
use gtk4::{IconLookupFlags, IconTheme, TextDirection};

pub fn bench_lookups(c: &mut Criterion) {
    let mut group = c.benchmark_group("ComparisonsLookups");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);

    let args = [
        "user-home",      // (Best case) An icon that can be found in the current theme
        "firefox",        // An icon that can be found in the hicolor default theme
        "archlinux-logo", // An icon that resides in /usr/share/pixmaps
        "not-found",      // (Worst case) An icon that does not exist
    ];

    for arg in args {
        group.bench_with_input(BenchmarkId::new("freedesktop-icons", arg), arg, |b, arg| {
            b.iter(|| {
                lookup(black_box(arg))
                    .with_theme(black_box("Adwaita"))
                    .find()
            });
        });

        group.bench_with_input(
            BenchmarkId::new("freedesktop-icons-cache", arg),
            arg,
            |b, arg| {
                b.iter(|| {
                    lookup(black_box(arg))
                        .with_scale(black_box(1))
                        .with_size(black_box(24))
                        .with_theme(black_box("Adwaita"))
                        .with_cache()
                        .find()
                });
            },
        );

        group.bench_with_input(BenchmarkId::new("linicon", arg), arg, |b, arg| {
            b.iter(|| {
                linicon::lookup_icon(black_box(arg))
                    .from_theme(black_box("Adwaita"))
                    .with_scale(black_box(1))
                    .with_size(black_box(24))
                    .next()
            });
        });

        group.bench_with_input(BenchmarkId::new("gtk", arg), arg, |b, arg| {
            gtk4::init().unwrap();
            let theme = IconTheme::new();
            b.iter(|| {
                theme
                    .lookup_icon(
                        black_box(arg),
                        black_box(&[]),
                        black_box(24),
                        black_box(1),
                        black_box(TextDirection::None),
                        black_box(IconLookupFlags::empty()),
                    )
                    .icon_name()
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_lookups);
criterion_main!(benches);
