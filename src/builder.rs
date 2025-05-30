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
    logo: Option<String>,
    logo_color: Option<String>,
}

impl CommonBadgeData {
    fn new() -> Self {
        Self {
            label: String::new(),
            message: String::new(),
            logo: None,
            logo_color: None,
        }
    }

    fn set_label_internal(&mut self, label: &str) {
        self.label = label.to_string();
    }

    fn set_message_internal(&mut self, message: &str) {
        self.message = message.to_string();
    }

    fn set_logo_internal(&mut self, logo: &str) {
        self.logo = Some(logo.to_string());
    }

    fn set_logo_color_internal(&mut self, color: &str) {
        self.logo_color = Some(color.to_string());
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
    /// 设置 logo，支持链式调用
    pub fn set_logo(&mut self, logo: &str) -> &mut Self {
        self.common_data.logo = Some(logo.to_string());
        self
    }
    /// 设置 logo 颜色，支持 hex 校验和链式调用
    pub fn set_logo_color(&mut self, color: &str) -> &mut Self {
        self.common_data.logo_color = Some(color.to_string());
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
                logo: self.common_data.logo.clone(),
                logo_color: self.common_data.logo_color.clone(),
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
            logo: self.common_data.logo.as_deref(),
            logo_color: self.common_data.logo_color.as_deref(),
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
            logo: self.common_data.logo.as_deref(),
            label_color: None,
            message_color: &"",
            logo_color: self.common_data.logo_color.as_deref(),
            link: Some(&self.state.link),
            extra_link: None,
        })
    }
}

impl SocialBadgeBuilder<TwoLink> {
    pub fn build(&self) -> String {
        render_badge_svg(&BadgeParams {
            style: BadgeStyle::Social,
            label: Some(self.common_data.label.as_str()),
            message: self.common_data.message.as_str(),
            logo: self.common_data.logo.as_deref(),
            label_color: None,
            message_color: &"",
            logo_color: self.common_data.logo_color.as_deref(),
            link: Some(&self.state.link),
            extra_link: Some(&self.state.extra_link),
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
}

impl ColorBadgeBuilder<NoLink> {
    pub fn build(&self) -> String {
        render_badge_svg(&BadgeParams {
            style: self.style,
            label: Some(self.common_data.label.as_str()),
            message: self.common_data.message.as_str(),
            label_color: Some(self.label_color.as_str()),
            message_color: self.message_color.as_str(),
            logo: self.common_data.logo.as_deref(),
            logo_color: self.common_data.logo_color.as_deref(),
            link: None,
            extra_link: None,
        })
    }
}
impl ColorBadgeBuilder<OneLink> {
    pub fn build(&self) -> String {
        render_badge_svg(&BadgeParams {
            style: self.style,
            label: Some(self.common_data.label.as_str()),
            message: self.common_data.message.as_str(),
            label_color: Some(self.label_color.as_str()),
            message_color: self.message_color.as_str(),
            link: Some(&self.state.link),
            extra_link: None,
            logo: self.common_data.logo.as_deref(),
            logo_color: self.common_data.logo_color.as_deref(),
        })
    }
}

impl ColorBadgeBuilder<TwoLink> {
    pub fn build(&self) -> String {
        render_badge_svg(&BadgeParams {
            style: self.style,
            label: Some(self.common_data.label.as_str()),
            message: self.common_data.message.as_str(),
            label_color: Some(self.label_color.as_str()),
            message_color: self.message_color.as_str(),
            link: Some(&self.state.link),
            extra_link: Some(&self.state.extra_link),
            logo: self.common_data.logo.as_deref(),
            logo_color: self.common_data.logo_color.as_deref(),
        })
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
    pub fn set_logo(mut self, logo: &str) -> Self {
        self.common_data.set_logo_internal(logo);
        self
    }
    pub fn set_logo_color(mut self, color: &str) -> Self {
        self.common_data.set_logo_color_internal(color);
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
        assert_eq!(
            badge,
            r##"<svg xmlns="http://www.w3.org/2000/svg" width="67" height="20"><style>a:hover #llink{fill:url(#b);stroke:#ccc}a:hover #rlink{fill:#4183c4}</style><linearGradient id="a" x2="0" y2="100%"><stop offset="0" stop-color="#fcfcfc" stop-opacity="0"/><stop offset="1" stop-opacity=".1"/></linearGradient><linearGradient id="b" x2="0" y2="100%"><stop offset="0" stop-color="#ccc" stop-opacity=".1"/><stop offset="1" stop-opacity=".1"/></linearGradient><g stroke="#d5d5d5"><rect stroke="none" fill="#fcfcfc" x="0.5" y="0.5" width="33" height="19" rx="2"/><rect x="39.5" y="0.5" width="27" height="19" rx="2" fill="#fafafa"/><rect x="39" y="7.5" width="0.5" height="5" stroke="#fafafa"/><path d="M39.5 6.5 l-3 3v1 l3 3" stroke="d5d5d5" fill="#fafafa"/></g><g aria-hidden="false" fill="#333" text-anchor="middle" font-family="Helvetica Neue,Helvetica,Arial,sans-serif" text-rendering="geometricPrecision" font-weight="700" font-size="110px" line-height="14px"><a target="_blank" href="https://example.com"><text aria-hidden="true" x="165" y="150" fill="#fff" transform="scale(.1)" textLength="230">Test</text><text x="165" y="140" transform="scale(.1)" textLength="230">Test</text><rect id="llink" stroke="#d5d5d5" fill="url(#a)" x=".5" y=".5" width="33" height="19" rx="2"/></a><a target="_blank" href="https://example.com/extra"><rect width="28" x="39" height="20" fill="rgba(0,0,0,0)"/><text aria-hidden="true" x="525" y="150" fill="#fff" transform="scale(.1)" textLength="190">test</text><text id="rlink" x="525" y="140" transform="scale(.1)" textLength="190">test</text></a></g></svg>"##
        )
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
        assert_eq!(
            badge,
            r##"<svg xmlns="http://www.w3.org/2000/svg" width="62" height="18" role="img" aria-label="test: test"><title>test: test</title><a target="_blank" href="https://example.com"><linearGradient id="s" x2="0" y2="100%"><stop offset="0" stop-color="#fff" stop-opacity=".7"/><stop offset=".1" stop-color="#aaa" stop-opacity=".1"/><stop offset=".9" stop-color="#000" stop-opacity=".3"/><stop offset="1" stop-color="#000" stop-opacity=".5"/></linearGradient><clipPath id="r"><rect width="62" height="18" rx="4" fill="#fff"/></clipPath><g clip-path="url(#r)"><rect width="31" height="18" fill="#555"/><rect x="31" width="31" height="18" fill="#007ec6"/><rect width="62" height="18" fill="url(#s)"/></g><g fill="#fff" text-anchor="middle" font-family="Verdana,Geneva,DejaVu Sans,sans-serif" text-rendering="geometricPrecision" font-size="110"><text aria-hidden="true" x="165" y="140" fill="#010101" fill-opacity=".3" transform="scale(.1)" textLength="210">test</text><text x="165" y="130" transform="scale(.1)" fill="#fff" textLength="210">test</text><text aria-hidden="true" x="455" y="140" fill="#010101" fill-opacity=".3" transform="scale(.1)" textLength="210">test</text><text x="455" y="130" transform="scale(.1)" fill="#fff" textLength="210">test</text></g></a></svg>"##
        );
    }

    #[test]
    fn test_builder_plastic_with_logo_and_logo_color() {
        let builder = Builder {};
        let badge = builder
            .plastic()
            .set_label("test")
            .set_logo("rust")
            .set_message("test")
            .set_logo_color("red")
            .build();
        assert_eq!(
            badge,
            r##"<svg xmlns="http://www.w3.org/2000/svg" width="79" height="18" role="img" aria-label="test: test"><title>test: test</title><linearGradient id="s" x2="0" y2="100%"><stop offset="0" stop-color="#fff" stop-opacity=".7"/><stop offset=".1" stop-color="#aaa" stop-opacity=".1"/><stop offset=".9" stop-color="#000" stop-opacity=".3"/><stop offset="1" stop-color="#000" stop-opacity=".5"/></linearGradient><clipPath id="r"><rect width="79" height="18" rx="4" fill="#fff"/></clipPath><g clip-path="url(#r)"><rect width="48" height="18" fill="#555"/><rect x="48" width="31" height="18" fill="#007ec6"/><rect width="79" height="18" fill="url(#s)"/></g><g fill="#fff" text-anchor="middle" font-family="Verdana,Geneva,DejaVu Sans,sans-serif" text-rendering="geometricPrecision" font-size="110"><image x="5" y="2" width="14" height="14" href="data:image/svg+xml;base64,PHN2ZyBmaWxsPSIjZTA1ZDQ0IiByb2xlPSJpbWciIHZpZXdCb3g9IjAgMCAyNCAyNCIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj48dGl0bGU+UnVzdDwvdGl0bGU+PHBhdGggZD0iTTIzLjgzNDYgMTEuNzAzM2wtMS4wMDczLS42MjM2YTEzLjcyNjggMTMuNzI2OCAwIDAwLS4wMjgzLS4yOTM2bC44NjU2LS44MDY5YS4zNDgzLjM0ODMgMCAwMC0uMTE1NC0uNTc4bC0xLjEwNjYtLjQxNGE4LjQ5NTggOC40OTU4IDAgMDAtLjA4Ny0uMjg1NmwuNjkwNC0uOTU4N2EuMzQ2Mi4zNDYyIDAgMDAtLjIyNTctLjU0NDZsLTEuMTY2My0uMTg5NGE5LjM1NzQgOS4zNTc0IDAgMDAtLjE0MDctLjI2MjJsLjQ5LTEuMDc2MWEuMzQzNy4zNDM3IDAgMDAtLjAyNzQtLjMzNjEuMzQ4Ni4zNDg2IDAgMDAtLjMwMDYtLjE1NGwtMS4xODQ1LjA0MTZhNi43NDQ0IDYuNzQ0NCAwIDAwLS4xODczLS4yMjY4bC4yNzIzLTEuMTUzYS4zNDcyLjM0NzIgMCAwMC0uNDE3LS40MTcybC0xLjE1MzIuMjcyNGExNC4wMTgzIDE0LjAxODMgMCAwMC0uMjI3OC0uMTg3M2wuMDQxNS0xLjE4NDVhLjM0NDIuMzQ0MiAwIDAwLS40OS0uMzI4bC0xLjA3Ni40OTFjLS4wODcyLS4wNDc2LS4xNzQyLS4wOTUyLS4yNjIzLS4xNDA3bC0uMTkwMy0xLjE2NzNBLjM0ODMuMzQ4MyAwIDAwMTYuMjU2Ljk1NWwtLjk1OTcuNjkwNWE4LjQ4NjcgOC40ODY3IDAgMDAtLjI4NTUtLjA4NmwtLjQxNC0xLjEwNjZhLjM0ODMuMzQ4MyAwIDAwLS41NzgxLS4xMTU0bC0uODA2OS44NjY2YTkuMjkzNiA5LjI5MzYgMCAwMC0uMjkzNi0uMDI4NEwxMi4yOTQ2LjE2ODNhLjM0NjIuMzQ2MiAwIDAwLS41ODkyIDBsLS42MjM2IDEuMDA3M2ExMy43MzgzIDEzLjczODMgMCAwMC0uMjkzNi4wMjg0TDkuOTgwMy4zMzc0YS4zNDYyLjM0NjIgMCAwMC0uNTc4LjExNTRsLS40MTQxIDEuMTA2NWMtLjA5NjIuMDI3NC0uMTkwMy4wNTY3LS4yODU1LjA4Nkw3Ljc0NC45NTVhLjM0ODMuMzQ4MyAwIDAwLS41NDQ3LjIyNThMNy4wMDkgMi4zNDhhOS4zNTc0IDkuMzU3NCAwIDAwLS4yNjIyLjE0MDdsLTEuMDc2Mi0uNDkxYS4zNDYyLjM0NjIgMCAwMC0uNDkuMzI4bC4wNDE2IDEuMTg0NWE3Ljk4MjYgNy45ODI2IDAgMDAtLjIyNzguMTg3M0wzLjg0MTMgMy40MjVhLjM0NzIuMzQ3MiAwIDAwLS40MTcxLjQxNzFsLjI3MTMgMS4xNTMxYy0uMDYyOC4wNzUtLjEyNTUuMTUwOS0uMTg2My4yMjY4bC0xLjE4NDUtLjA0MTVhLjM0NjIuMzQ2MiAwIDAwLS4zMjguNDlsLjQ5MSAxLjA3NjFhOS4xNjcgOS4xNjcgMCAwMC0uMTQwNy4yNjIybC0xLjE2NjIuMTg5NGEuMzQ4My4zNDgzIDAgMDAtLjIyNTguNTQ0NmwuNjkwNC45NTg3YTEzLjMwMyAxMy4zMDMgMCAwMC0uMDg3LjI4NTVsLTEuMTA2NS40MTRhLjM0ODMuMzQ4MyAwIDAwLS4xMTU1LjU3ODFsLjg2NTYuODA3YTkuMjkzNiA5LjI5MzYgMCAwMC0uMDI4My4yOTM1bC0xLjAwNzMuNjIzNmEuMzQ0Mi4zNDQyIDAgMDAwIC41ODkybDEuMDA3My42MjM2Yy4wMDguMDk4Mi4wMTgyLjE5NjQuMDI4My4yOTM2bC0uODY1Ni44MDc5YS4zNDYyLjM0NjIgMCAwMC4xMTU1LjU3OGwxLjEwNjUuNDE0MWMuMDI3My4wOTYyLjA1NjcuMTkxNC4wODcuMjg1NWwtLjY5MDQuOTU4N2EuMzQ1Mi4zNDUyIDAgMDAuMjI2OC41NDQ3bDEuMTY2Mi4xODkzYy4wNDU2LjA4OC4wOTIyLjE3NTEuMTQwOC4yNjIybC0uNDkxIDEuMDc2MmEuMzQ2Mi4zNDYyIDAgMDAuMzI4LjQ5bDEuMTgzNC0uMDQxNWMuMDYxOC4wNzY5LjEyMzUuMTUyOC4xODczLjIyNzdsLS4yNzEzIDEuMTU0MWEuMzQ2Mi4zNDYyIDAgMDAuNDE3MS40MTYxbDEuMTUzLS4yNzEzYy4wNzUuMDYzOC4xNTEuMTI1NS4yMjc5LjE4NjNsLS4wNDE1IDEuMTg0NWEuMzQ0Mi4zNDQyIDAgMDAuNDkuMzI3bDEuMDc2MS0uNDljLjA4Ny4wNDg2LjE3NDEuMDk1MS4yNjIyLjE0MDdsLjE5MDMgMS4xNjYyYS4zNDgzLjM0ODMgMCAwMC41NDQ3LjIyNjhsLjk1ODctLjY5MDRhOS4yOTkgOS4yOTkgMCAwMC4yODU1LjA4N2wuNDE0IDEuMTA2NmEuMzQ1Mi4zNDUyIDAgMDAuNTc4MS4xMTU0bC44MDc5LS44NjU2Yy4wOTcyLjAxMTEuMTk1NC4wMjAzLjI5MzYuMDI5NGwuNjIzNiAxLjAwNzNhLjM0NzIuMzQ3MiAwIDAwLjU4OTIgMGwuNjIzNi0xLjAwNzNjLjA5ODItLjAwOTEuMTk2NC0uMDE4My4yOTM2LS4wMjk0bC44MDY5Ljg2NTZhLjM0ODMuMzQ4MyAwIDAwLjU3OC0uMTE1NGwuNDE0MS0xLjEwNjZhOC40NjI2IDguNDYyNiAwIDAwLjI4NTUtLjA4N2wuOTU4Ny42OTA0YS4zNDUyLjM0NTIgMCAwMC41NDQ3LS4yMjY4bC4xOTAzLTEuMTY2MmMuMDg4LS4wNDU2LjE3NTEtLjA5MzEuMjYyMi0uMTQwN2wxLjA3NjIuNDlhLjM0NzIuMzQ3MiAwIDAwLjQ5LS4zMjdsLS4wNDE1LTEuMTg0NWE2LjcyNjcgNi43MjY3IDAgMDAuMjI2Ny0uMTg2M2wxLjE1MzEuMjcxM2EuMzQ3Mi4zNDcyIDAgMDAuNDE3MS0uNDE2bC0uMjcxMy0xLjE1NDJjLjA2MjgtLjA3NDkuMTI1NS0uMTUwOC4xODYzLS4yMjc4bDEuMTg0NS4wNDE1YS4zNDQyLjM0NDIgMCAwMC4zMjgtLjQ5bC0uNDktMS4wNzZjLjA0NzUtLjA4NzIuMDk1MS0uMTc0Mi4xNDA3LS4yNjIzbDEuMTY2Mi0uMTg5M2EuMzQ4My4zNDgzIDAgMDAuMjI1OC0uNTQ0N2wtLjY5MDQtLjk1ODcuMDg3LS4yODU1IDEuMTA2Ni0uNDE0YS4zNDYyLjM0NjIgMCAwMC4xMTU0LS41NzgxbC0uODY1Ni0uODA3OWMuMDEwMS0uMDk3Mi4wMjAyLS4xOTU0LjAyODMtLjI5MzZsMS4wMDczLS42MjM2YS4zNDQyLjM0NDIgMCAwMDAtLjU4OTJ6bS02Ljc0MTMgOC4zNTUxYS43MTM4LjcxMzggMCAwMS4yOTg2LTEuMzk2LjcxNC43MTQgMCAxMS0uMjk5NyAxLjM5NnptLS4zNDIyLTIuMzE0MmEuNjQ5LjY0OSAwIDAwLS43NzE1LjVsLS4zNTczIDEuNjY4NWMtMS4xMDM1LjUwMS0yLjMyODUuNzc5NS0zLjYxOTMuNzc5NWE4LjczNjggOC43MzY4IDAgMDEtMy42OTUxLS44MTRsLS4zNTc0LTEuNjY4NGEuNjQ4LjY0OCAwIDAwLS43NzE0LS40OTlsLTEuNDczLjMxNThhOC43MjE2IDguNzIxNiAwIDAxLS43NjEzLS44OThoNy4xNjc2Yy4wODEgMCAuMTM1Ni0uMDE0MS4xMzU2LS4wODh2LTIuNTM2YzAtLjA3NC0uMDUzNi0uMDg4MS0uMTM1Ni0uMDg4MWgtMi4wOTY2di0xLjYwNzdoMi4yNjc3Yy4yMDY1IDAgMS4xMDY1LjA1ODcgMS4zOTQgMS4yMDg4LjA5MDEuMzUzMy4yODc1IDEuNTA0NC40MjMyIDEuODcyOS4xMzQ2LjQxMy42ODMzIDEuMjM4MSAxLjI2ODUgMS4yMzgxaDMuNTcxNmEuNzQ5Mi43NDkyIDAgMDAuMTI5Ni0uMDEzMSA4Ljc4NzQgOC43ODc0IDAgMDEtLjgxMTkuOTUyNnpNNi44MzY5IDIwLjAyNGEuNzE0LjcxNCAwIDExLS4yOTk3LTEuMzk2LjcxNC43MTQgMCAwMS4yOTk3IDEuMzk2ek00LjExNzcgOC45OTcyYS43MTM3LjcxMzcgMCAxMS0xLjMwNC41NzkxLjcxMzcuNzEzNyAwIDAxMS4zMDQtLjU3OXptLS44MzUyIDEuOTgxM2wxLjUzNDctLjY4MjRhLjY1LjY1IDAgMDAuMzMtLjg1ODVsLS4zMTU4LS43MTQ3aDEuMjQzMnY1LjYwMjVIMy41NjY5YTguNzc1MyA4Ljc3NTMgMCAwMS0uMjgzNC0zLjM0OHptNi43MzQzLS41NDM3VjguNzgzNmgyLjk2MDFjLjE1MyAwIDEuMDc5Mi4xNzcyIDEuMDc5Mi44Njk3IDAgLjU3NS0uNzEwNy43ODE1LTEuMjk0OC43ODE1em0xMC43NTc0IDEuNDg2MmMwIC4yMTg3LS4wMDguNDM2My0uMDI0My42NTFoLS45Yy0uMDkgMC0uMTI2NS4wNTg2LS4xMjY1LjE0Nzd2LjQxM2MwIC45NzMtLjU0ODcgMS4xODQ2LTEuMDI5NiAxLjIzODItLjQ1NzYuMDUxNy0uOTY0OC0uMTkxMy0xLjAyNzUtLjQ3MTctLjI3MDQtMS41MTg2LS43MTk4LTEuODQzNi0xLjQzMDUtMi40MDM0Ljg4MTctLjU1OTkgMS43OTktMS4zODYgMS43OTktMi40OTE1IDAtMS4xOTM2LS44MTktMS45NDU4LTEuMzc2OS0yLjMxNTMtLjc4MjUtLjUxNjMtMS42NDkxLS42MTk1LTEuODgzLS42MTk1SDUuNDY4MmE4Ljc2NTEgOC43NjUxIDAgMDE0LjkwNy0yLjc2OTlsMS4wOTc0IDEuMTUxYS42NDguNjQ4IDAgMDAuOTE4Mi4wMjEzbDEuMjI3LTEuMTc0M2E4Ljc3NTMgOC43NzUzIDAgMDE2LjAwNDQgNC4yNzYybC0uODQwMyAxLjg5ODJhLjY1Mi42NTIgMCAwMC4zMy44NTg1bDEuNjE3OC43MTg4Yy4wMjgzLjI4NzUuMDQyNS41NzcuMDQyNS44NzE3em0tOS4zMDA2LTkuNTk5M2EuNzEyOC43MTI4IDAgMTEuOTg0IDEuMDMxNi43MTM3LjcxMzcgMCAwMS0uOTg0LTEuMDMxNnptOC4zMzg5IDYuNzFhLjcxMDcuNzEwNyAwIDAxLjkzOTUtLjM2MjUuNzEzNy43MTM3IDAgMTEtLjk0MDUuMzYzNXoiLz48L3N2Zz4="/><text aria-hidden="true" x="335" y="140" fill="#010101" fill-opacity=".3" transform="scale(.1)" textLength="210">test</text><text x="335" y="130" transform="scale(.1)" fill="#fff" textLength="210">test</text><text aria-hidden="true" x="625" y="140" fill="#010101" fill-opacity=".3" transform="scale(.1)" textLength="210">test</text><text x="625" y="130" transform="scale(.1)" fill="#fff" textLength="210">test</text></g></svg>"##
        )
    }
}
