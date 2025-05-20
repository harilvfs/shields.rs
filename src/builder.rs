use crate::{
    BadgeParams, BadgeStyle, default_label_color, default_message_color, render_badge_svg,
};

pub struct NoLink;
pub struct OneLink {
    link: String,
}
pub struct TwoLink {
    link: String,
    extra_link: String,
}

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

pub struct SocialBadgeBuilder<State = NoLink> {
    common_data: CommonBadgeData,
    state: State,
}

impl<S> SocialBadgeBuilder<S> {
    pub fn set_label(mut self, label: &str) -> Self {
        self.common_data.set_label_internal(label);
        self
    }
    pub fn set_message(mut self, message: &str) -> Self {
        self.common_data.set_message_internal(message);
        self
    }
}

impl SocialBadgeBuilder<NoLink> {
    pub fn new() -> Self {
        Self {
            common_data: CommonBadgeData::new(),
            state: NoLink,
        }
    }
    // 第一次设置 link，返回 OneLink 状态的新 Builder
    pub fn set_link(&mut self, link: &str) -> SocialBadgeBuilder<OneLink> {
        SocialBadgeBuilder {
            common_data: CommonBadgeData {
                label: self.common_data.label.clone(),
                message: self.common_data.message.clone(),
            },
            state: OneLink {
                link: link.to_string(),
            },
        }
    }
    pub fn build(&self) -> String {
        render_badge_svg(&BadgeParams {
            style: BadgeStyle::Social,
            label: Some(self.common_data.label.as_str()),
            message: self.common_data.message.as_str(),
            label_color: None,
            message_color: &"",
            link: None,
            extra_link: None,
            logo: None,
            logo_color: None,
        })
    }
}

impl SocialBadgeBuilder<OneLink> {
    pub fn set_extra_link(self, link: &str) -> SocialBadgeBuilder<TwoLink> {
        SocialBadgeBuilder {
            common_data: self.common_data,
            state: TwoLink {
                link: self.state.link,
                extra_link: link.to_string(),
            },
        }
    }
    pub fn build(&self) -> String {
        render_badge_svg(&BadgeParams {
            style: BadgeStyle::Social,
            label: Some(self.common_data.label.as_str()),
            message: self.common_data.message.as_str(),
            label_color: None,
            message_color: &"",
            link: Some(&self.state.link),
            extra_link: None,
            logo: None,
            logo_color: None,
        })
    }
}

impl SocialBadgeBuilder<TwoLink> {
    pub fn build(&self) -> String {
        render_badge_svg(&BadgeParams {
            style: BadgeStyle::Social,
            label: Some(self.common_data.label.as_str()),
            message: self.common_data.message.as_str(),
            label_color: None,
            message_color: &"",
            link: Some(&self.state.link),
            extra_link: Some(&self.state.extra_link),
            logo: None,
            logo_color: None,
        })
    }
}

pub struct ColorBadgeBuilder<State = NoLink> {
    common_data: CommonBadgeData,
    style: BadgeStyle,
    label_color: String,
    message_color: String,
    state: State,
}

impl ColorBadgeBuilder<NoLink> {
    pub fn new(style: BadgeStyle) -> Self {
        Self {
            style,
            common_data: CommonBadgeData::new(),
            label_color: String::from(default_label_color()),
            message_color: String::from(default_message_color()),
            state: NoLink,
        }
    }
    pub fn set_link(self, link: &str) -> ColorBadgeBuilder<OneLink> {
        ColorBadgeBuilder {
            style: self.style,
            common_data: self.common_data,
            label_color: self.label_color,
            message_color: self.message_color,
            state: OneLink {
                link: link.to_string(),
            },
        }
    }
}

impl ColorBadgeBuilder<OneLink> {
    pub fn set_extra_link(self, extra: &str) -> ColorBadgeBuilder<TwoLink> {
        ColorBadgeBuilder {
            style: self.style,
            common_data: self.common_data,
            label_color: self.label_color,
            message_color: self.message_color,
            state: TwoLink {
                link: self.state.link,
                extra_link: extra.to_string(),
            },
        }
    }
}

impl<S> ColorBadgeBuilder<S> {
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
            logo: None,
            logo_color: None,
        });
    }
}

impl<S> ColorBadgeBuilder<S> {
    pub fn set_label(mut self, label: &str) -> Self {
        self.common_data.set_label_internal(label);
        self
    }
    pub fn set_message(mut self, message: &str) -> Self {
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
    // #[test]
    // fn test_builder() {
    //     let builder = Builder {};
    //     let badge = builder
    //         .flat()
    //         .set_label("test")
    //         .set_message("test")
    //         .set_label_color("#000000")
    //         .set_message_color("#FFFFFF")
    //         .build();
    //     assert_eq!(
    //         badge,
    //         r##"<svg xmlns="http://www.w3.org/2000/svg" width="62" height="20" role="img" aria-label="test: test"><title>test: test</title><linearGradient id="s" x2="0" y2="100%"><stop offset="0" stop-color="#bbb" stop-opacity=".1"/><stop offset="1" stop-opacity=".1"/></linearGradient><clipPath id="r"><rect width="62" height="20" rx="3" fill="#fff"/></clipPath><g clip-path="url(#r)"><rect width="31" height="20" fill="#000000"/><rect x="31" width="31" height="20" fill="#ffffff"/><rect width="62" height="20" fill="url(#s)"/></g><g fill="#fff" text-anchor="middle" font-family="Verdana,Geneva,DejaVu Sans,sans-serif" text-rendering="geometricPrecision" font-size="110"><text aria-hidden="true" x="165" y="150" fill="#010101" fill-opacity=".3" transform="scale(.1)" textLength="210">test</text><text x="165" y="140" transform="scale(.1)" fill="#fff" textLength="210">test</text><text aria-hidden="true" x="455" y="150" fill="#ccc" fill-opacity=".3" transform="scale(.1)" textLength="210">test</text><text x="455" y="140" transform="scale(.1)" fill="#333" textLength="210">test</text></g></svg>"##
    //     );
    // }
    #[test]
    fn test_builder_social_with_link() {
        let builder = Builder {};
        let badge = builder
            .social()
            .set_label("test")
            .set_link("https://example.com")
            .set_message("test")
            .set_extra_link("https://example.com/extra")
            .build();
        // assert_eq!(
        //     badge,
        //     r##"<svg xmlns="http://www.w3.org/2000/svg" width="67" height="20"><style>a:hover #llink{fill:url(#b);stroke:#ccc}a:hover #rlink{fill:#4183c4}</style><linearGradient id="a" x2="0" y2="100%"><stop offset="0" stop-color="#fcfcfc" stop-opacity="0"/><stop offset="1" stop-opacity=".1"/></linearGradient><linearGradient id="b" x2="0" y2="100%"><stop offset="0" stop-color="#ccc" stop-opacity=".1"/><stop offset="1" stop-opacity=".1"/></linearGradient><g stroke="#d5d5d5"><rect stroke="none" fill="#fcfcfc" x="0.5" y="0.5" width="33" height="19" rx="2"/><rect x="39.5" y="0.5" width="27" height="19" rx="2" fill="#fafafa"/><rect x="39" y="7.5" width="0.5" height="5" stroke="#fafafa"/><path d="M39.5 6.5 l-3 3v1 l3 3" stroke="d5d5d5" fill="#fafafa"/></g><g aria-hidden="false" fill="#333" text-anchor="middle" font-family="Helvetica Neue,Helvetica,Arial,sans-serif" text-rendering="geometricPrecision" font-weight="700" font-size="110px" line-height="14px"><a target="_blank" href="https://example.com"><text aria-hidden="true" x="165" y="150" fill="#fff" transform="scale(.1)" textLength="230">Test</text><text x="165" y="140" transform="scale(.1)" textLength="230">Test</text><rect id="llink" stroke="#d5d5d5" fill="url(#a)" x=".5" y=".5" width="33" height="19" rx="2"/></a><a target="_blank" href="https://example.com/extra"><rect width="28" x="39" height="20" fill="rgba(0,0,0,0)"/><text aria-hidden="true" x="525" y="150" fill="#fff" transform="scale(.1)" textLength="190">test</text><text id="rlink" x="525" y="140" transform="scale(.1)" textLength="190">test</text></a></g></svg>"##
        // )
    }

    #[test]
    fn test_builder_plastic_with_link() {
        let builder = Builder {};
        let badge = builder
            .plastic()
            .set_label("test")
            .set_link("https://example.com")
            .set_message("test")
            .build();
        // assert_eq!(
        //     badge,
        //     r##"<svg xmlns="http://www.w3.org/2000/svg" width="62" height="18" role="img" aria-label="test: test"><title>test: test</title><linearGradient id="s" x2="0" y2="100%"><stop offset="0" stop-color="#fff" stop-opacity=".7"/><stop offset=".1" stop-color="#aaa" stop-opacity=".1"/><stop offset=".9" stop-color="#000" stop-opacity=".3"/><stop offset="1" stop-color="#000" stop-opacity=".5"/></linearGradient><clipPath id="r"><rect width="62" height="18" rx="4" fill="#fff"/></clipPath><g clip-path="url(#r)"><rect width="31" height="18" fill="#555"/><rect x="31" width="31" height="18" fill="#007ec6"/><rect width="62" height="18" fill="url(#s)"/></g><g fill="#fff" text-anchor="middle" font-family="Verdana,Geneva,DejaVu Sans,sans-serif" text-rendering="geometricPrecision" font-size="110"><text aria-hidden="true" x="165" y="140" fill="#010101" fill-opacity=".3" transform="scale(.1)" textLength="210">test</text><text x="165" y="130" transform="scale(.1)" fill="#fff" textLength="210">test</text><text aria-hidden="true" x="455" y="140" fill="#010101" fill-opacity=".3" transform="scale(.1)" textLength="210">test</text><text x="455" y="130" transform="scale(.1)" fill="#fff" textLength="210">test</text></g></svg>"##
        // );
    }
}
