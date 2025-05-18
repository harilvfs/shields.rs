use crate::{
    BadgeParams, BadgeStyle, default_label_color, default_message_color, render_badge_svg,
};

// 共通数据结构体
pub struct CommonBadgeData {
    label: String,
    message: String,
}

impl CommonBadgeData {
    fn new() -> Self {
        Self {
            label: String::new(),
            message: String::new(),
        }
    }

    // 将实际的 set 逻辑移到这里
    fn set_label_internal(&mut self, label: &str) {
        self.label = label.to_string();
    }

    fn set_message_internal(&mut self, message: &str) {
        self.message = message.to_string();
    }
}

// 定义一个包含共通方法的 Trait
pub trait BadgeBuilder {
    fn set_label(&mut self, label: &str) -> &mut Self;
    fn set_message(&mut self, message: &str) -> &mut Self;
}

pub struct SocialBadgeBuilder {
    common_data: CommonBadgeData,
}

impl SocialBadgeBuilder {
    fn new() -> Self {
        SocialBadgeBuilder {
            common_data: CommonBadgeData::new(),
        }
    }

    pub fn build(&self) -> String {
        // 这里可以根据 label, message 等生成 badge 的 URL
        return render_badge_svg(&BadgeParams {
            style: BadgeStyle::Social,
            label: Some(self.common_data.label.as_str()),
            message: self.common_data.message.as_str(),
            label_color: None,
            message_color: &"",
            link: None,
            extra_link: None,
        });
    }
}
impl BadgeBuilder for SocialBadgeBuilder {
    fn set_label(&mut self, label: &str) -> &mut Self {
        self.common_data.set_label_internal(label);
        self
    }

    fn set_message(&mut self, message: &str) -> &mut Self {
        self.common_data.set_message_internal(message);
        self
    }
}

pub struct ColorBadgeBuilder {
    common_data: CommonBadgeData,
    style: BadgeStyle,
    label_color: String,
    message_color: String,
}

impl ColorBadgeBuilder {
    pub fn new(style: BadgeStyle) -> Self {
        ColorBadgeBuilder {
            style,
            common_data: CommonBadgeData::new(),
            label_color: String::from(default_label_color()),
            message_color: String::from(default_message_color()),
        }
    }
    pub fn set_label_color(&mut self, color: &str) -> &mut Self {
        self.label_color = color.to_string();
        self
    }
    pub fn set_message_color(&mut self, color: &str) -> &mut Self {
        self.message_color = color.to_string();
        self
    }

    pub fn build(&self) -> String {
        // 这里可以根据 style, label, message 等生成 badge 的 URL
        return render_badge_svg(&BadgeParams {
            style: self.style,
            label: Some(self.common_data.label.as_str()),
            message: self.common_data.message.as_str(),
            label_color: Some(self.label_color.as_str()),
            message_color: self.message_color.as_str(),
            link: None,
            extra_link: None,
        });
    }
}

impl BadgeBuilder for ColorBadgeBuilder {
    fn set_label(&mut self, label: &str) -> &mut Self {
        self.common_data.set_label_internal(label);
        self
    }
    fn set_message(&mut self, message: &str) -> &mut Self {
        self.common_data.set_message_internal(message);
        self
    }
}

pub struct Builder {}

impl Builder {
    pub fn social(self) -> SocialBadgeBuilder {
        SocialBadgeBuilder::new()
    }

    pub fn flat(self) -> ColorBadgeBuilder {
        ColorBadgeBuilder::new(BadgeStyle::Base(crate::BaseBadgeStyle::Flat))
    }

    pub fn flat_square(self) -> ColorBadgeBuilder {
        ColorBadgeBuilder::new(BadgeStyle::Base(crate::BaseBadgeStyle::FlatSquare))
    }

    pub fn plastic(self) -> ColorBadgeBuilder {
        ColorBadgeBuilder::new(BadgeStyle::Base(crate::BaseBadgeStyle::Plastic))
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test_builder() {
        let builder = Builder {};
        let badge = builder
            .flat()
            .set_label("test")
            .set_message("test")
            .set_label_color("#000000")
            .set_message_color("#FFFFFF")
            .build();
        assert_eq!(
            badge,
            r##"<svg xmlns="http://www.w3.org/2000/svg" width="62" height="20" role="img" aria-label="test: test"><title>test: test</title><linearGradient id="s" x2="0" y2="100%"><stop offset="0" stop-color="#bbb" stop-opacity=".1"/><stop offset="1" stop-opacity=".1"/></linearGradient><clipPath id="r"><rect width="62" height="20" rx="3" fill="#fff"/></clipPath><g clip-path="url(#r)"><rect width="31" height="20" fill="#000000"/><rect x="31" width="31" height="20" fill="#ffffff"/><rect width="62" height="20" fill="url(#s)"/></g><g fill="#fff" text-anchor="middle" font-family="Verdana,Geneva,DejaVu Sans,sans-serif" text-rendering="geometricPrecision" font-size="110"><text aria-hidden="true" x="165" y="150" fill="#010101" fill-opacity=".3" transform="scale(.1)" textLength="210">test</text><text x="165" y="140" transform="scale(.1)" fill="#fff" textLength="210">test</text><text aria-hidden="true" x="455" y="150" fill="#ccc" fill-opacity=".3" transform="scale(.1)" textLength="210">test</text><text x="455" y="140" transform="scale(.1)" fill="#333" textLength="210">test</text></g></svg>"##
        );
    }
}
