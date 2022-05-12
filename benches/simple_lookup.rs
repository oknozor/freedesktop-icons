use criterion::{criterion_group, criterion_main, Criterion};
use freedesktop_icons::lookup;
use gtk4::{IconLookupFlags, IconTheme, TextDirection};

pub fn simple_lookup(c: &mut Criterion) {
    c.bench_function("lookup firefox", |b| b.iter(|| lookup("firefox").find()));
}

pub fn simple_lookup_linicon(c: &mut Criterion) {
    c.bench_function("lookup firefox linicon", |b| {
        b.iter(|| linicon::lookup_icon("firefox").next().unwrap().unwrap())
    });
}

pub fn simple_lookup_gtk(c: &mut Criterion) {
    c.bench_function("lookup firefox gkt", |b| {
        gtk4::init().unwrap();
        let theme = IconTheme::new();
        b.iter(|| {
            theme.lookup_icon(
                "firefox",
                &[],
                24,
                1,
                TextDirection::None,
                IconLookupFlags::empty(),
            )
        })
    });
}

criterion_group!(
    benches,
    simple_lookup,
    simple_lookup_linicon,
    simple_lookup_gtk
);
criterion_main!(benches);
