use askama::Template;

/// SVG 渲染模板上下文，字段需与 badge_svg_template_askama.svg 中变量一一对应
#[derive(Template)]
#[template(path = "badge_svg_template.svg", escape = "none")]
struct BadgeSvgTemplateContext<'a> {
    total_width: i32,
    badge_height: i32,
    accessible_text: &'a str,
    left_width: i32,
    right_width: i32,
    label_color: &'a str,
    message_color: &'a str,
    rx: i32,
    font_family: &'a str,
    font_size_scaled: i32,

    has_label: bool,

    label: &'a str,
    label_x: f32,
    label_width_scaled: i32,
    label_text_color: &'a str,
    label_shadow_color: &'a str,

    message: &'a str,
    message_x: f32,
    message_shadow_color: &'a str,
    message_text_color: &'a str,
    message_width_scaled: i32,
}
// 声明 measurer 模块为公有，确保 crate::measurer 可用
pub mod measurer;
/// shields.rs —— 纯 SVG 徽章生成库
/// 只包含 SVG 生成逻辑，不涉及 web、IO、API
use serde::Deserialize;

// --- 颜色处理工具模块 ---
// 支持命名色、别名、hex、CSS 颜色输入的标准化与 SVG 输出

mod color_util {
    use lru::LruCache;
    use once_cell::sync::Lazy;
    use regex::Regex;
    use std::collections::HashMap;
    use std::num::NonZeroUsize;
    use std::sync::Mutex;

    // 命名色映射
    pub static NAMED_COLORS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
        HashMap::from([
            ("brightgreen", "#4c1"),
            ("green", "#97ca00"),
            ("yellow", "#dfb317"),
            ("yellowgreen", "#a4a61d"),
            ("orange", "#fe7d37"),
            ("red", "#e05d44"),
            ("blue", "#007ec6"),
            ("grey", "#555"),
            ("lightgrey", "#9f9f9f"),
        ])
    });

    // 别名映射
    pub static ALIASES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
        HashMap::from([
            ("gray", "grey"),
            ("lightgray", "lightgrey"),
            ("critical", "red"),
            ("important", "orange"),
            ("success", "brightgreen"),
            ("informational", "blue"),
            ("inactive", "lightgrey"),
        ])
    });

    // 3/6位hex校验
    pub fn is_valid_hex(s: &str) -> bool {
        let s = s.trim_start_matches('#');
        let len = s.len();
        (len == 3 || len == 6) && s.chars().all(|c| c.is_ascii_hexdigit())
    }

    // 简化版CSS颜色校验（支持 rgb(a)、hsl(a)、常见格式）
    pub fn is_css_color(s: &str) -> bool {
        static CSS_COLOR_RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^(rgb|rgba|hsl|hsla)\s*\(").unwrap());
        CSS_COLOR_RE.is_match(s.trim())
    }

    /// 标准化颜色输入，返回可用于SVG的字符串或None
    pub fn normalize_color(color: &str) -> Option<String> {
        static CACHE: Lazy<Mutex<LruCache<String, Option<String>>>> =
            Lazy::new(|| Mutex::new(LruCache::new(NonZeroUsize::new(256).unwrap())));
        let color = color.trim();
        if color.is_empty() {
            return None;
        }
        let key = color.to_ascii_lowercase();
        // 先查缓存
        if let Some(cached) = {
            let mut cache = CACHE.lock().unwrap();
            cache.get(&key).cloned()
        } {
            return cached;
        }
        // 只在有大写字母时分配
        let lower = color.to_ascii_lowercase();
        let result = if NAMED_COLORS.contains_key(lower.as_str()) {
            Some(lower.to_string())
        } else if let Some(&alias) = ALIASES.get(lower.as_str()) {
            Some(alias.to_string())
        } else if is_valid_hex(lower.as_str()) {
            let hex = lower.trim_start_matches('#');
            Some(format!("#{}", hex))
        } else if is_css_color(lower.as_str()) {
            Some(lower.to_string())
        } else {
            None
        };
        let mut cache = CACHE.lock().unwrap();
        cache.put(key, result.clone());
        result
    }

    /// 输出SVG可用颜色（hex字符串），优先命名色、别名，否则原样
    pub fn to_svg_color(color: &str) -> Option<String> {
        static CACHE: Lazy<Mutex<LruCache<String, Option<String>>>> =
            Lazy::new(|| Mutex::new(LruCache::new(NonZeroUsize::new(256).unwrap())));
        let key = color.to_ascii_lowercase();
        if let Some(cached) = {
            let mut cache = CACHE.lock().unwrap();
            cache.get(&key).cloned()
        } {
            return cached;
        }
        let normalized = normalize_color(color)?;
        let result = if let Some(&hex) = NAMED_COLORS.get(normalized.as_str()) {
            Some(hex.to_string())
        } else if let Some(&alias) = ALIASES.get(normalized.as_str()) {
            NAMED_COLORS.get(alias).map(|&h| h.to_string())
        } else {
            Some(normalized)
        };
        let mut cache = CACHE.lock().unwrap();
        cache.put(key, result.clone());
        result
    }
}
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

/// 根据背景色动态计算前景色与阴影色（等价 JS colorsForBackground）
///
/// - 输入：hex 颜色字符串（支持 3/6 位，如 "#4c1"、"#007ec6"）
/// - 算法：
///   1. 解析 hex 为 RGB
///   2. 计算亮度 brightness = (0.299*R + 0.587*G + 0.114*B) / 255
///   3. 若亮度 ≤ 0.69，返回 ("#fff", "#010101")，否则 ("#333", "#ccc")
pub fn colors_for_background(hex: &str) -> (&'static str, &'static str) {
    // 去除前导 #
    let hex = hex.trim_start_matches('#');
    // 解析 RGB
    let (r, g, b) = match hex.len() {
        3 => (
            {
                let c = hex.as_bytes()[0];
                let v = match c {
                    b'0'..=b'9' => c - b'0',
                    b'a'..=b'f' => c - b'a' + 10,
                    b'A'..=b'F' => c - b'A' + 10,
                    _ => 0,
                };
                (v << 4) | v
            },
            {
                let c = hex.as_bytes()[1];
                let v = match c {
                    b'0'..=b'9' => c - b'0',
                    b'a'..=b'f' => c - b'a' + 10,
                    b'A'..=b'F' => c - b'A' + 10,
                    _ => 0,
                };
                (v << 4) | v
            },
            {
                let c = hex.as_bytes()[2];
                let v = match c {
                    b'0'..=b'9' => c - b'0',
                    b'a'..=b'f' => c - b'a' + 10,
                    b'A'..=b'F' => c - b'A' + 10,
                    _ => 0,
                };
                (v << 4) | v
            },
        ),
        6 => (
            u8::from_str_radix(&hex[0..2], 16).unwrap_or(0),
            u8::from_str_radix(&hex[2..4], 16).unwrap_or(0),
            u8::from_str_radix(&hex[4..6], 16).unwrap_or(0),
        ),
        _ => (0, 0, 0), // 非法输入，返回黑色
    };
    // W3C 推荐亮度公式
    let brightness = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) / 255.0;
    if brightness <= 0.69 {
        ("#fff", "#010101")
    } else {
        ("#333", "#ccc")
    }
}
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
    // 颜色标准化处理，兼容命名色、别名、hex、CSS
    use crate::color_util::to_svg_color;
    let label_color =
        to_svg_color(params.label_color).unwrap_or_else(|| default_label_color().to_string());
    let message_color =
        to_svg_color(params.message_color).unwrap_or_else(|| default_message_color().to_string());
    render_badge(
        params.label,
        params.message,
        &label_color,
        &message_color,
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
                BaseBadgeStyle::Flat => {
                    // 计算 label/message 区域的前景色与阴影色
                    let (label_text_color, label_shadow_color) = colors_for_background(label_color);
                    let (message_text_color, message_shadow_color) =
                        colors_for_background(message_color);
                    let _label_svg = if has_label {
                        let label = label.unwrap();
                        format!(
                            r##"<text aria-hidden="true" x="{label_x}" y="150" fill="{label_shadow_color}" fill-opacity=".3" transform="scale(.1)" textLength="{label_width_scaled}">{label}</text>
                            <text x="{label_x}" y="140" transform="scale(.1)" fill="{label_text_color}" textLength="{label_width_scaled}">{label}</text>"##,
                            label_shadow_color = label_shadow_color,
                            label_text_color = label_text_color,
                        )
                    } else {
                        String::new()
                    };

                    BadgeSvgTemplateContext {
                        total_width: total_width as i32,
                        badge_height: BADGE_HEIGHT as i32,
                        accessible_text: accessible_text.as_str(),
                        left_width: left_width as i32,
                        right_width: right_width as i32,
                        label_color,
                        message_color,
                        rx,
                        font_family: FONT_FAMILY,
                        font_size_scaled: FONT_SIZE_SCALED as i32,
                        has_label: has_label,
                        label: label.unwrap_or(""),
                        label_x: label_x,
                        label_width_scaled: label_width_scaled as i32,
                        label_text_color,
                        label_shadow_color,
                        message_x,
                        message_shadow_color,
                        message_text_color,
                        message_width_scaled: message_width_scaled as i32,
                        message,
                    }
                    .render()
                    .unwrap_or_else(|e| format!("<!-- Askama render error: {} -->", e))
                }
                BaseBadgeStyle::FlatSquare => {
                    // 计算 label/message 区域的前景色
                    let (label_text_color, _) = colors_for_background(label_color);
                    let (message_text_color, _) = colors_for_background(message_color);
                    let label_svg = if has_label {
                        let label = label.unwrap();
                        format!(
                            r##"<text x="{label_x}" y="140" transform="scale(.1)" fill="{label_text_color}" textLength="{label_width_scaled}">{label}</text>"##,
                            label_text_color = label_text_color,
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
                                    <text x="{message_x}" y="140" transform="scale(.1)" fill="{message_text_color}" textLength="{message_width_scaled}">{message}</text>
                                </g>
                            </svg>"##,
                        label_svg = label_svg,
                        message_x = message_x,
                        message_text_color = message_text_color,
                        message_width_scaled = message_width_scaled,
                        total_width = total_width,
                        BADGE_HEIGHT = BADGE_HEIGHT,
                        accessible_text = accessible_text.as_str(),
                        left_width = left_width,
                        right_width = right_width,
                        label_color = label_color,
                        message_color = message_color,
                    )
                }

                BaseBadgeStyle::Plastic => {
                    let label_color = if has_label {
                        label_color
                    } else {
                        message_color
                    };
                    let (label_text_color, label_shadow_color) = colors_for_background(label_color);
                    let (message_text_color, message_shadow_color) =
                        colors_for_background(message_color);
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
      <text aria-hidden="true" x="{label_text_x}" y="140" fill="{label_shadow_color}" fill-opacity=".3" transform="scale(.1)" textLength="{label_text_length}">{label}</text>
      <text x="{label_text_x}" y="130" transform="scale(.1)" fill="{label_text_color}" textLength="{label_text_length}">{label}</text>
      <text aria-hidden="true" x="{message_text_x}" y="140" fill="{message_shadow_color}" fill-opacity=".3" transform="scale(.1)" textLength="{message_text_length}">{message}</text>
      <text x="{message_text_x}" y="130" transform="scale(.1)" fill="{message_text_color}" textLength="{message_text_length}">{message}</text>
  </g>
</svg>"##,
                            label = label.unwrap(),
                            label_width = left_width,
                            message_width = right_width,
                            label_text_x = label_x,
                            message_text_x = message_x,
                            label_text_length = label_width_scaled,
                            message_text_length = message_width_scaled,
                            total_width = total_width,
                            message_text_color = message_text_color,
                            message_shadow_color = message_shadow_color,
                            label_text_color = label_text_color,
                            label_shadow_color = label_shadow_color,
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
      <text aria-hidden="true" x="{message_text_x}" y="140" fill="{message_shadow_color}" fill-opacity=".3" transform="scale(.1)" textLength="{message_text_length}">{message}</text>
      <text x="{message_text_x}" y="130" transform="scale(.1)" fill="{message_text_color}" textLength="{message_text_length}">{message}</text>
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
    #[test]
    fn test_named_color() {
        let params = RenderBadgeParams {
            style: BadgeStyle::flat_square(),
            label: Some("status"),
            message: "ok",
            label_color: "brightgreen",
            message_color: "blue",
        };
        let svg = render_badge_svg(&params);
        assert!(
            svg.contains("fill=\"#4c1\""),
            "命名色 brightgreen 未正确映射"
        );
        assert!(svg.contains("fill=\"#007ec6\""), "命名色 blue 未正确映射");
    }

    #[test]
    fn test_alias_color() {
        let params = RenderBadgeParams {
            style: BadgeStyle::flat_square(),
            label: Some("status"),
            message: "ok",
            label_color: "gray",
            message_color: "critical",
        };
        let svg = render_badge_svg(&params);
        assert!(svg.contains("fill=\"#555\""), "别名 gray 未正确映射");
        assert!(svg.contains("fill=\"#e05d44\""), "别名 critical 未正确映射");
    }

    #[test]
    fn test_hex_color() {
        let params = RenderBadgeParams {
            style: BadgeStyle::flat_square(),
            label: Some("hex"),
            message: "ok",
            label_color: "#4c1",
            message_color: "dfb317",
        };
        let svg = render_badge_svg(&params);
        assert!(svg.contains("fill=\"#4c1\""), "3位hex未正确处理");
        assert!(svg.contains("fill=\"#dfb317\""), "6位hex未正确处理");
    }

    #[test]
    fn test_css_color() {
        let params = RenderBadgeParams {
            style: BadgeStyle::flat_square(),
            label: Some("css"),
            message: "ok",
            label_color: "rgb(0,128,0)",
            message_color: "hsl(120,100%,25%)",
        };
        let svg = render_badge_svg(&params);
        assert!(
            svg.contains(r#"fill="rgb(0,128,0)""#),
            "CSS rgb 颜色未正确处理"
        );
        assert!(
            svg.contains(r#"fill="hsl(120,100%,25%)""#),
            "CSS hsl 颜色未正确处理"
        );
    }

    #[test]
    fn test_invalid_color_fallback() {
        let params = RenderBadgeParams {
            style: BadgeStyle::flat_square(),
            label: Some("bad"),
            message: "ok",
            label_color: "notacolor",
            message_color: "",
        };
        let svg = render_badge_svg(&params);
        assert!(
            svg.contains("fill=\"#555\""),
            "非法 label_color 未 fallback 到默认色"
        );
        assert!(
            svg.contains("fill=\"#007ec6\""),
            "空 message_color 未 fallback 到默认色"
        );
    }
}
