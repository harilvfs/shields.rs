use criterion::{Criterion, criterion_group, criterion_main};
use rand::{Rng, distributions::Alphanumeric};
use shields::{BadgeParams, BadgeStyle, render_badge_svg};

fn random_string() -> String {
    let len = rand::thread_rng().gen_range(8..=12);
    rand::thread_rng()
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
                style: BadgeStyle::flat(),
                label: Some(&binding),
                message: &random_string(),
                label_color: Some("#555"),
                message_color: "#4c1",
                link: None,
                extra_link: None,
            };
            let _svg = render_badge_svg(&params);
        });
    });
}

criterion_group!(benches, bench_flat_badge);
criterion_main!(benches);
