use criterion::{Criterion, criterion_group, criterion_main};
use rand::{Rng, distr::Alphanumeric};
use shields::{BadgeParams, BadgeStyle, render_badge_svg};

fn random_string() -> String {
    let len = rand::rng().random_range(8..=12);
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

fn bench_flat_badge(c: &mut Criterion) {
    c.bench_function("render_flat_badge_svg", |b| {
        b.iter(|| {
            let binding = random_string();
            let params = BadgeParams {
                style: BadgeStyle::Flat,
                label: Some(&binding),
                message: &random_string(),
                label_color: Some("#555"),
                message_color: "#4c1",
                link: None,
                extra_link: None,
                logo: None,
                logo_color: None,
            };
            let _svg = render_badge_svg(&params);
        });
    });
}

criterion_group!(benches, bench_flat_badge);
criterion_main!(benches);
