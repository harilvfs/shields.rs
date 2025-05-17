// 声明 measurer 模块为公有，确保 crate::measurer 可用
pub mod measurer;
/// shields.rs —— 纯 SVG 徽章生成库
/// 只包含 SVG 生成逻辑，不涉及 web、IO、API
use serde::Deserialize;

/// 字体宽度计算 trait，主项目需实现并注入
pub trait FontMetrics {
    /// 支持 font-family 顺序 fallback
    fn get_text_width_px(&self, text: &str, font_family: &str) -> f32;
}

/// 计算文本在 Verdana 11px 下的宽度（像素）
///
/// - 仅需传入 text，内部自动加载并复用宽度表
/// - 高效惰性初始化，避免重复 IO
/// - 可直接用于 SVG 徽章等场景
pub fn get_text_width(text: &str) -> f64 {
    use crate::measurer::CharWidthMeasurer;
    use once_cell::sync::Lazy;

    // 静态全局，首次调用时加载 JSON，后续复用
    static VERDANA_WIDTH_TABLE: Lazy<CharWidthMeasurer> = Lazy::new(|| {
        CharWidthMeasurer::load_sync("assets/fonts/verdana_11px.json")
            .expect("无法加载 Verdana 11px 宽度表")
    });

    VERDANA_WIDTH_TABLE.width_of(text, true)
}

macro_rules! round_up_to_odd_float {
    ($func:ident, $float:ty) => {
        fn $func(n: $float) -> u32 {
            let n_rounded = n.floor() as u32;
            if n_rounded % 2 == 0 {
                n_rounded + 1
            } else {
                n_rounded
            }
        }
    };
}

round_up_to_odd_float!(round_up_to_odd_f64, f64);
const BADGE_HEIGHT: u32 = 20;
const HORIZONTAL_PADDING: u32 = 5;
const FONT_FAMILY: &str = "Verdana,Geneva,DejaVu Sans,sans-serif";
const FONT_SIZE_SCALED: u32 = 110;

pub(crate) fn preferred_width_of(text: &str) -> u32 {
    use lru::LruCache;
    use once_cell::sync::Lazy;
    use std::num::NonZeroUsize;
    use std::sync::Mutex;

    static WIDTH_CACHE: Lazy<Mutex<LruCache<String, u32>>> =
        Lazy::new(|| Mutex::new(LruCache::new(NonZeroUsize::new(1024).unwrap())));

    {
        let mut cache = WIDTH_CACHE.lock().unwrap();
        if let Some(&cached) = cache.get(text) {
            return cached;
        }
    }

    let width = get_text_width(text);
    let rounded = round_up_to_odd_f64(width);

    if text.len() <= 1024 {
        let mut cache = WIDTH_CACHE.lock().unwrap();
        cache.put(text.to_string(), rounded);
    }

    rounded
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum BaseBadgeStyle {
    Flat,
    FlatSquare,
    Plastic,
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum BadgeStyle {
    Base(BaseBadgeStyle),
    Social,
}

impl Default for BadgeStyle {
    fn default() -> Self {
        BadgeStyle::Base(BaseBadgeStyle::Flat)
    }
}

impl BadgeStyle {
    pub const fn flat() -> Self {
        BadgeStyle::Base(BaseBadgeStyle::Flat)
    }
    pub const fn flat_square() -> Self {
        BadgeStyle::Base(BaseBadgeStyle::FlatSquare)
    }
    pub const fn plastic() -> Self {
        BadgeStyle::Base(BaseBadgeStyle::Plastic)
    }
    pub const fn social() -> Self {
        BadgeStyle::Social
    }
}

pub fn default_label_color() -> &'static str {
    "#555"
}
pub fn default_message_color() -> &'static str {
    "#007ec6"
}

#[derive(Deserialize, Debug)]
pub struct RenderBadgeParams<'a> {
    #[serde(default)]
    pub style: BadgeStyle,
    pub label: Option<&'a str>,
    pub message: &'a str,
    #[serde(default = "default_label_color")]
    pub label_color: &'a str,
    #[serde(default = "default_message_color")]
    pub message_color: &'a str,
}

/// 公开 API：生成 SVG 字符串
pub fn render_badge_svg(params: &RenderBadgeParams) -> String {
    // 新增 variant 参数传递
    render_badge(
        params.label,
        params.message,
        params.label_color,
        params.message_color,
        params.style,
    )
}

fn create_accessible_text(label: Option<&str>, message: &str) -> String {
    let label_len = label.map_or(0, |l| l.len() + 2); // +2 for ": "
    let mut buf = String::with_capacity(label_len + message.len());
    if let Some(label) = label {
        buf.push_str(label);
        buf.push_str(": ");
    }
    buf.push_str(message);
    buf
}

// --- 通用 Badge 渲染函数 ---
fn render_badge(
    label: Option<&str>,
    message: &str,
    label_color: &str,
    message_color: &str,
    style: BadgeStyle,
) -> String {
    match style {
        BadgeStyle::Base(base) => {
            let rx = match base {
                BaseBadgeStyle::FlatSquare => 0,
                _ => 3,
            };

            let logo_width = 0;
            let logo_padding = 0;
            let has_logo = false;
            let total_logo_width = logo_width + logo_padding;
            let accessible_text = create_accessible_text(label, message);
            let has_label = label.is_some();
            let label_margin = total_logo_width + 1;
            let label_width = if has_label {
                preferred_width_of(label.unwrap())
            } else {
                0
            };
            let left_width = if has_label {
                label_width + 2 * HORIZONTAL_PADDING + total_logo_width
            } else {
                0
            };
            let message_width = preferred_width_of(message);
            let mut message_margin: i32 =
                left_width as i32 - if message.is_empty() { 0 } else { 1 };
            if !has_label {
                if has_logo {
                    message_margin = message_margin + (total_logo_width + HORIZONTAL_PADDING) as i32
                } else {
                    message_margin = message_margin + 1
                }
            }
            let mut right_width = message_width + 2 * HORIZONTAL_PADDING;
            if has_logo && !has_label {
                right_width += total_logo_width
                    + if !message.is_empty() {
                        HORIZONTAL_PADDING - 1
                    } else {
                        0
                    };
            }
            let total_width = left_width + right_width;

            let message_x = 10.0
                * (message_margin as f32
                    + (0.5 * message_width as f32)
                    + HORIZONTAL_PADDING as f32);
            let message_width_scaled = message_width * 10;

            let label_x = 10.0
                * (label_margin as f32 + (0.5 * label_width as f32) + HORIZONTAL_PADDING as f32);
            let label_color = if has_label { label_color } else { "#e05d44" };
            let label_width_scaled = label_width * 10;
            match base {
                BaseBadgeStyle::FlatSquare => {
                    let label_svg = if has_label {
                        let label = label.unwrap();
                        format!(
                            r##"<text x="{label_x}" y="140" transform="scale(.1)" fill="#fff" textLength="{label_width_scaled}">{label}</text>"##,
                        )
                    } else {
                        String::new()
                    };
                    format!(
                        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{total_width}" height="{BADGE_HEIGHT}" role="img"
                                aria-label="{accessible_text}">
                                <title>{accessible_text}</title>
                                <g shape-rendering="crispEdges">
                                    <rect width="{left_width}" height="20" fill="{label_color}" />
                                    <rect x="{left_width}" width="{right_width}" height="20" fill="{message_color}" />
                                </g>
                                <g fill="#fff" text-anchor="middle" font-family="Verdana,Geneva,DejaVu Sans,sans-serif"
                                    text-rendering="geometricPrecision" font-size="110">
                                    {label_svg}
                                    <text x="{message_x}" y="140" transform="scale(.1)" fill="#fff" textLength="{message_width_scaled}">{message}</text>
                                </g>
                            </svg>"##
                    )
                }
                BaseBadgeStyle::Flat => {
                    let label_svg = if has_label {
                        let label = label.unwrap();
                        format!(
                            r##"<text aria-hidden="true" x="{label_x}" y="150" fill="#010101" fill-opacity=".3" transform="scale(.1)" textLength="{label_width_scaled}">{label}</text>
                            <text x="{label_x}" y="140" transform="scale(.1)" fill="#fff" textLength="{label_width_scaled}">{label}</text>"##,
                        )
                    } else {
                        String::new()
                    };

                    format!(
                        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{total_width}" height="{BADGE_HEIGHT}" role="img" aria-label="{accessible_text}">
                            <title>{accessible_text}</title>
                            <linearGradient id="s" x2="0" y2="100%">
                                <stop offset="0" stop-color="#bbb" stop-opacity=".1"/>
                                <stop offset="1" stop-opacity=".1"/>
                            </linearGradient>
                            <clipPath id="r">
                                <rect width="{total_width}" height="{BADGE_HEIGHT}" rx="{rx}" fill="#fff"/>
                            </clipPath>
                            <g clip-path="url(#r)">
                                <rect width="{left_width}" height="{BADGE_HEIGHT}" fill="{label_color}"/>
                                <rect x="{left_width}" width="{right_width}" height="{BADGE_HEIGHT}" fill="{message_color}"/>
                                <rect width="{total_width}" height="{BADGE_HEIGHT}" fill="url(#s)"/>
                            </g>
                            <g fill="#fff" text-anchor="middle" font-family="{FONT_FAMILY}" text-rendering="geometricPrecision" font-size="{FONT_SIZE_SCALED}">
                                {label_svg}
                                <text aria-hidden="true" x="{message_x}" y="150" fill="#010101" fill-opacity=".3" transform="scale(.1)" textLength="{message_width_scaled}">{message}</text>
                                <text x="{message_x}" y="140" transform="scale(.1)" fill="#fff" textLength="{message_width_scaled}">{message}</text>
                            </g>
                        </svg>"##,
                    )
                }
                BaseBadgeStyle::Plastic => {
                    let label_color = if has_label {
                        label_color
                    } else {
                        message_color
                    };
                    let label_is_some_and_not_empty = label.map_or(false, |l| !l.is_empty());
                    if label_is_some_and_not_empty {
                        // label 存在，保持原 SVG 结构
                        format!(
                            r##"<svg xmlns="http://www.w3.org/2000/svg" width="{total_width}" height="18" role="img" aria-label="{label}: {message}">
  <title>{label}: {message}</title>
  <linearGradient id="s" x2="0" y2="100%">
    <stop offset="0" stop-color="#fff" stop-opacity=".7"/>
    <stop offset=".1" stop-color="#aaa" stop-opacity=".1"/>
    <stop offset=".9" stop-color="#000" stop-opacity=".3"/>
    <stop offset="1" stop-color="#000" stop-opacity=".5"/>
  </linearGradient>
  <clipPath id="r">
    <rect width="{total_width}" height="18" rx="4" fill="#fff"/>
  </clipPath>
  <g clip-path="url(#r)">
    <rect width="{label_width}" height="18" fill="{label_color}"/>
    <rect x="{label_width}" width="{message_width}" height="18" fill="{message_color}"/>
    <rect width="{total_width}" height="18" fill="url(#s)"/>
  </g>
  <g fill="#fff" text-anchor="middle" font-family="Verdana,Geneva,DejaVu Sans,sans-serif" text-rendering="geometricPrecision" font-size="110">
    <text aria-hidden="true" x="{label_text_x}" y="140" fill="#010101" fill-opacity=".3" transform="scale(.1)" textLength="{label_text_length}">{label}</text>
    <text x="{label_text_x}" y="130" transform="scale(.1)" fill="#fff" textLength="{label_text_length}">{label}</text>
    <text aria-hidden="true" x="{message_text_x}" y="140" fill="#ccc" fill-opacity=".3" transform="scale(.1)" textLength="{message_text_length}">{message}</text>
    <text x="{message_text_x}" y="130" transform="scale(.1)" fill="#333" textLength="{message_text_length}">{message}</text>
  </g>
</svg>"##,
                            label = label.unwrap(),
                            label_width = left_width,
                            message_width = right_width,
                            label_text_x = label_x,
                            message_text_x = message_x,
                            label_text_length = label_width_scaled,
                            message_text_length = message_width_scaled
                        )
                    } else {
                        // label 为空或 None，仅渲染 message 区域
                        format!(
                            r##"<svg xmlns="http://www.w3.org/2000/svg" width="{total_width}" height="18" role="img" aria-label="{message}">
  <title>{message}</title>
  <linearGradient id="s" x2="0" y2="100%">
    <stop offset="0" stop-color="#fff" stop-opacity=".7"/>
    <stop offset=".1" stop-color="#aaa" stop-opacity=".1"/>
    <stop offset=".9" stop-color="#000" stop-opacity=".3"/>
    <stop offset="1" stop-color="#000" stop-opacity=".5"/>
  </linearGradient>
  <clipPath id="r">
    <rect width="{total_width}" height="18" rx="4" fill="#fff"/>
  </clipPath>
  <g clip-path="url(#r)">
    <rect width="{label_width}" height="18" fill="{label_color}"/>
    <rect x="0" width="{total_width}" height="18" fill="{message_color}"/>
    <rect width="{total_width}" height="18" fill="url(#s)"/>
  </g>
  <g fill="#fff" text-anchor="middle" font-family="Verdana,Geneva,DejaVu Sans,sans-serif" text-rendering="geometricPrecision" font-size="110">
    <text aria-hidden="true" x="{message_text_x}" y="140" fill="#ccc" fill-opacity=".3" transform="scale(.1)" textLength="{message_text_length}">{message}</text>
    <text x="{message_text_x}" y="130" transform="scale(.1)" fill="#333" textLength="{message_text_length}">{message}</text>
  </g>
</svg>"##,
                            message_text_x = message_x,
                            message_text_length = message_width_scaled
                        )
                    }
                }
            }
        }
        _ => "".to_string(),
    }
}

// --- Badge 结构体及链式 API 实现 ---
#[derive(Debug, Clone)]
pub struct Badge {
    style: BadgeStyle,
    label: Option<String>,
    message: String,
    label_color: String,
    message_color: String,
}

impl Badge {
    /// 创建默认 Badge 实例
    pub fn new() -> Self {
        Badge {
            style: BadgeStyle::default(),
            label: None,
            message: String::new(),
            label_color: default_label_color().to_string(),
            message_color: default_message_color().to_string(),
        }
    }

    /// 设置 label
    pub fn set_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// 设置 message
    pub fn set_message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    /// 设置 style
    pub fn set_style(mut self, style: BadgeStyle) -> Self {
        self.style = style;
        self
    }

    /// 设置 label_color
    pub fn set_label_color(mut self, color: impl Into<String>) -> Self {
        self.label_color = color.into();
        self
    }

    /// 设置 message_color
    pub fn set_message_color(mut self, color: impl Into<String>) -> Self {
        self.message_color = color.into();
        self
    }

    /// 渲染 SVG 字符串
    pub fn render(&self) -> String {
        let params = RenderBadgeParams {
            style: self.style,
            label: self.label.as_deref(),
            message: &self.message,
            label_color: &self.label_color,
            message_color: &self.message_color,
        };
        render_badge_svg(&params)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_svg() {
        // 测试 SVG 渲染
        let params = RenderBadgeParams {
            style: BadgeStyle::flat_square(),
            label: Some("build"),
            message: "passing",
            label_color: "#333",
            message_color: "#4c1",
        };
        let svg = render_badge_svg(&params);
        assert!(!svg.is_empty(), "SVG 渲染失败");
    }

    #[test]
    fn test_svg_chain() {
        // 测试链式 API
        let svg = Badge::new()
            .set_label("build")
            .set_message("passing")
            .set_style(BadgeStyle::flat_square())
            .set_label_color("#333")
            .set_message_color("#4c1")
            .render();
        assert!(!svg.is_empty(), "SVG 渲染失败");
    }
}
