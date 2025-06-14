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

// A. Traditional parameter struct
fn bench_params_badge(c: &mut Criterion) {
    c.bench_function("params_badge_svg", |b| {
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
                logo: Some("rust"),
                logo_color: Some("#FFF"),
            };
            let _svg = render_badge_svg(&params);
        });
    });
}

// B. Builder pattern
fn bench_builder_badge(c: &mut Criterion) {
    c.bench_function("builder_badge_svg", |b| {
        b.iter(|| {
            let binding = random_string();
            let _svg = shields::builder::Badge::style(random_style())
                .label(&binding)
                .message(&binding)
                .label_color("#555")
                .message_color("#4c1")
                .logo("rust")
                .logo_color("#FFF")
                .build();
        });
    });
}

criterion_group!(benches, bench_params_badge, bench_builder_badge);
criterion_main!(benches);
