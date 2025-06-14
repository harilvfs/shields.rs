use crate::{
    BadgeParams, BadgeStyle, default_label_color, default_message_color, render_badge_svg,
};

/// 通用的徽章构建器
pub struct BadgeBuilder {
    style: BadgeStyle,
    label: String,
    message: String,
    label_color: Option<String>,
    message_color: Option<String>,
    logo: Option<String>,
    logo_color: Option<String>,
    link: Option<String>,
    extra_link: Option<String>,
}

impl BadgeBuilder {
    /// 创建新的构建器
    fn new(style: BadgeStyle) -> Self {
        Self {
            style,
            label: String::new(),
            message: String::new(),
            label_color: None,
            message_color: None,
            logo: None,
            logo_color: None,
            link: None,
            extra_link: None,
        }
    }

    /// 设置标签文本
    pub fn label<S: Into<String>>(mut self, label: S) -> Self {
        self.label = label.into();
        self
    }

    /// 设置消息文本
    pub fn message<S: Into<String>>(mut self, message: S) -> Self {
        self.message = message.into();
        self
    }

    /// 设置标签颜色
    pub fn label_color<S: Into<String>>(mut self, color: S) -> Self {
        self.label_color = Some(color.into());
        self
    }

    /// 设置消息颜色
    pub fn message_color<S: Into<String>>(mut self, color: S) -> Self {
        self.message_color = Some(color.into());
        self
    }

    /// 设置 logo
    pub fn logo<S: Into<String>>(mut self, logo: S) -> Self {
        self.logo = Some(logo.into());
        self
    }

    /// 设置 logo 颜色
    pub fn logo_color<S: Into<String>>(mut self, color: S) -> Self {
        self.logo_color = Some(color.into());
        self
    }

    /// 设置链接
    pub fn link<S: Into<String>>(mut self, link: S) -> Self {
        self.link = Some(link.into());
        self
    }

    /// 设置额外链接（第二个链接）
    pub fn extra_link<S: Into<String>>(mut self, link: S) -> Self {
        self.extra_link = Some(link.into());
        self
    }

    /// 构建 SVG 字符串
    pub fn build(self) -> String {
        // Social 风格的特殊处理
        let (label_color, message_color) = if self.style == BadgeStyle::Social {
            (None, Some(""))
        } else {
            (
                Some(self.label_color.as_deref().unwrap_or(default_label_color())),
                Some(
                    self.message_color
                        .as_deref()
                        .unwrap_or(default_message_color()),
                ),
            )
        };

        render_badge_svg(&BadgeParams {
            style: self.style,
            label: if self.label.is_empty() {
                None
            } else {
                Some(&self.label)
            },
            message: if self.message.is_empty() {
                None
            } else {
                Some(&self.message)
            },
            label_color,
            message_color,
            logo: self.logo.as_deref(),
            logo_color: self.logo_color.as_deref(),
            link: self.link.as_deref(),
            extra_link: self.extra_link.as_deref(),
        })
    }
}

/// 徽章构建器入口点
pub struct Badge;

impl Badge {
    /// 创建 Social 风格徽章
    pub fn social() -> BadgeBuilder {
        BadgeBuilder::new(BadgeStyle::Social)
    }

    /// 创建 Flat 风格徽章
    pub fn flat() -> BadgeBuilder {
        BadgeBuilder::new(BadgeStyle::Flat)
    }

    /// 创建 Flat Square 风格徽章
    pub fn flat_square() -> BadgeBuilder {
        BadgeBuilder::new(BadgeStyle::FlatSquare)
    }

    /// 创建 Plastic 风格徽章
    pub fn plastic() -> BadgeBuilder {
        BadgeBuilder::new(BadgeStyle::Plastic)
    }

    /// 创建 For The Badge 风格徽章
    pub fn for_the_badge() -> BadgeBuilder {
        BadgeBuilder::new(BadgeStyle::ForTheBadge)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flat_badge() {
        let badge = Badge::flat()
            .label("test")
            .message("test")
            .label_color("#000000")
            .message_color("#FFFFFF")
            .build();

        assert!(badge.contains(r##"fill="#000000"##));
        assert!(badge.contains(r##"fill="#ffffff"##));
    }

    #[test]
    fn test_social_badge_with_links() {
        let badge = Badge::social()
            .label("test")
            .message("test")
            .link("https://example.com")
            .extra_link("https://example.com/extra")
            .build();

        assert!(badge.contains(r#"href="https://example.com""#));
        assert!(badge.contains(r#"href="https://example.com/extra""#));
    }

    #[test]
    fn test_plastic_badge_with_logo() {
        let badge = Badge::plastic()
            .label("test")
            .message("test")
            .logo("rust")
            .logo_color("red")
            .build();

        assert!(badge.contains(r#"<image"#));
    }

    #[test]
    fn test_for_the_badge_style() {
        let badge = Badge::for_the_badge()
            .label("BUILD")
            .message("PASSING")
            .label_color("#555")
            .message_color("#4c1")
            .build();

        assert!(badge.contains("BUILD"));
        assert!(badge.contains("PASSING"));
    }

    #[test]
    fn test_builder_chaining() {
        let badge = Badge::flat()
            .label("version")
            .message("1.0.0")
            .label_color("#555")
            .message_color("#blue")
            .logo("github")
            .link("https://github.com/user/repo")
            .build();

        assert!(badge.contains("version"));
        assert!(badge.contains("1.0.0"));
    }
}
