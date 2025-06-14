use crate::{
    BadgeParams, BadgeStyle, default_label_color, default_message_color, render_badge_svg,
};

pub struct BadgeBuilder<'a> {
    style: BadgeStyle,
    label: Option<&'a str>,
    message: Option<&'a str>,
    label_color: Option<&'a str>,
    message_color: Option<&'a str>,
    logo: Option<&'a str>,
    logo_color: Option<&'a str>,
    link: Option<&'a str>,
    extra_link: Option<&'a str>,
}

impl<'a> BadgeBuilder<'a> {
    /// Create a new builder
    fn new(style: BadgeStyle) -> Self {
        Self {
            style,
            label: None,
            message: None,
            label_color: None,
            message_color: None,
            logo: None,
            logo_color: None,
            link: None,
            extra_link: None,
        }
    }

    /// Set the label text. Note: method signature now takes &mut self and &'a str
    pub fn label(&mut self, label: &'a str) -> &mut Self {
        self.label = Some(label);
        self
    }

    /// Set the message text
    pub fn message(&mut self, message: &'a str) -> &mut Self {
        self.message = Some(message);
        self
    }

    /// Set the label color
    pub fn label_color(&mut self, color: &'a str) -> &mut Self {
        self.label_color = Some(color);
        self
    }

    /// Set the message color
    pub fn message_color(&mut self, color: &'a str) -> &mut Self {
        self.message_color = Some(color);
        self
    }

    /// Set the logo
    pub fn logo(&mut self, logo: &'a str) -> &mut Self {
        self.logo = Some(logo);
        self
    }

    /// Set the logo color
    pub fn logo_color(&mut self, color: &'a str) -> &mut Self {
        self.logo_color = Some(color);
        self
    }

    /// Set the link
    pub fn link(&mut self, link: &'a str) -> &mut Self {
        self.link = Some(link);
        self
    }

    /// Set the extra link (second link)
    pub fn extra_link(&mut self, link: &'a str) -> &mut Self {
        self.extra_link = Some(link);
        self
    }

    /// Build the SVG string. Note: takes &self, does not consume the builder.
    pub fn build(&self) -> String {
        let (label_color, message_color) = if self.style == BadgeStyle::Social {
            (None, Some(""))
        } else {
            (
                Some(self.label_color.unwrap_or(default_label_color())),
                Some(self.message_color.unwrap_or(default_message_color())),
            )
        };

        render_badge_svg(&BadgeParams {
            style: self.style,
            label: self.label,
            message: self.message,
            label_color,
            message_color,
            logo: self.logo,
            logo_color: self.logo_color,
            link: self.link,
            extra_link: self.extra_link,
        })
    }
}

/// Badge builder entry point
// This struct is now just a namespace, does not hold any data
pub struct Badge;

impl Badge {
    pub fn style(style: BadgeStyle) -> BadgeBuilder<'static> {
        BadgeBuilder::new(style)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimized_flat_badge() {
        let mut builder = Badge::style(BadgeStyle::Flat);
        let badge = builder
            .label("test1")
            .message("test2")
            .label_color("#000000")
            .message_color("#ffffff")
            .build();

        assert!(badge.contains("test1"));
        assert!(badge.contains("test2"));
    }

    #[test]
    fn test_optimized_chaining() {
        let badge = Badge::style(BadgeStyle::Plastic)
            .label("version")
            .message("1.0.0")
            .logo("github")
            .build();

        assert!(badge.contains("version"));
        assert!(badge.contains("1.0.0"));
    }

    #[test]
    fn test_alternative_chaining() {
        let badge = {
            let mut b = Badge::style(BadgeStyle::Flat);
            b.label("hello").message("world");
            b.build()
        };
        assert!(badge.contains("hello"));
        assert!(badge.contains("world"));
    }
    #[test]
    fn test_configuring_step_by_step() {
        let mut badge = Badge::style(BadgeStyle::Flat);
        badge.label("no chaining");
        badge.message("test");
        badge.label_color("#000");
        badge.message_color("#fff");
        let resp = badge.build();
        assert!(resp.contains("no chaining"));
        assert!(resp.contains("test"));
    }
}
