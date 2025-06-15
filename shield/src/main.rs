use shields::{BadgeParams, BadgeStyle, render_badge_svg};

fn main() {
    let params = BadgeParams {
        style: BadgeStyle::Flat,
        label: Some("Built With"),
        message: Some("Ratatui"),
        label_color: Some("black"),
        message_color: Some("black"),
        link: Some("https://ratatui.rs/"),
        extra_link: None,
        logo: Some("rust"),
        logo_color: None,
    };

    let svg = render_badge_svg(&params);
    assert!(svg.contains("Ratatui"));
    println!("{}", svg);
}
