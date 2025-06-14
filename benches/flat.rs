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

fn random_style() -> BadgeStyle {
    let styles = [
        BadgeStyle::Flat,
        BadgeStyle::Plastic,
        BadgeStyle::ForTheBadge,
        BadgeStyle::Social,
    ];
    let index = rand::rng().random_range(0..styles.len());
    styles[index]
}

fn bench_flat_badge(c: &mut Criterion) {
    c.bench_function("render_flat_badge_svg", |b| {
        b.iter(|| {
            let binding = random_string();
            let params = BadgeParams {
                style: random_style(),
                label: Some(binding.as_str()),
                message: Some(binding.as_str()),
                label_color: Some("#555"),
                message_color: Some("#4c1"),
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
