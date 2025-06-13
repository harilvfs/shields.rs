use std::str::FromStr;

use askama::{Template, filters::capitalize};
pub mod builder;
pub mod measurer;
use base64::Engine;
use color_util::to_svg_color;
use csscolorparser::Color;
use serde::Deserialize;

/// SVG rendering template context, fields must correspond to variables in badge_svg_template_askama.svg
#[derive(Template)]
#[template(path = "flat_badge_template.min.svg", escape = "none")]
struct FlatBadgeSvgTemplateContext<'a> {
    total_width: i32,
    badge_height: i32,
    accessible_text: &'a str,
    left_width: i32,
    right_width: i32,
    label_color: &'a str,
    message_color: &'a str,
    font_family: &'a str,
    font_size_scaled: i32,

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

    link: &'a str,
    extra_link: &'a str,

    logo: &'a str,
    rect_offset: i32,

    message_link_x: i32,
}
/// flat-square SVG rendering template context
#[derive(Template)]
#[template(path = "flat_square_badge_template.min.svg", escape = "none")]
struct FlatSquareBadgeSvgTemplateContext<'a> {
    total_width: i32,
    badge_height: i32,
    accessible_text: &'a str,
    left_width: i32,
    right_width: i32,
    label_color: &'a str,
    message_color: &'a str,
    font_family: &'a str,
    font_size_scaled: i32,

    label: &'a str,
    label_x: f32,
    label_width_scaled: i32,
    label_text_color: &'a str,

    message: &'a str,
    message_x: f32,
    message_text_color: &'a str,
    message_width_scaled: i32,

    link: &'a str,
    extra_link: &'a str,
    logo: &'a str,
    rect_offset: i32,

    message_link_x: i32,
}
/// plastic SVG rendering template context
#[derive(Template)]
#[template(path = "plastic_badge_template.min.svg", escape = "none")]
struct PlasticBadgeSvgTemplateContext<'a> {
    total_width: i32,
    accessible_text: &'a str,
    left_width: i32,
    right_width: i32,
    // gradient
    label: &'a str,
    label_x: f32,
    label_text_length: i32,
    label_text_color: &'a str,
    label_shadow_color: &'a str,
    message: &'a str,
    message_x: f32,
    message_text_length: i32,
    message_text_color: &'a str,
    message_shadow_color: &'a str,
    label_color: &'a str,
    message_color: &'a str,

    link: &'a str,
    extra_link: &'a str,

    logo: &'a str,
    rect_offset: i32,

    message_link_x: i32,
}

/// social SVG rendering template context
#[derive(Template)]
#[template(path = "social_badge_template.min.svg", escape = "none")]
struct SocialBadgeSvgTemplateContext<'a> {
    total_width: i32,
    total_height: i32,
    internal_height: u32,
    accessible_text: &'a str,
    label_rect_width: i32,
    message_bubble_main_x: f32,
    message_rect_width: u32,
    message_bubble_notch_x: i32,
    label_text_x: f32,
    label_text_length: u32,
    label: &'a str,
    message_text_x: f32,
    message_text_length: u32,
    message: &'a str,

    link: &'a str,
    extra_link: &'a str,

    logo: &'a str,
}

// --- Color processing utility module ---
// Supports standardization and SVG output of named colors, aliases, hex, and CSS color inputs

mod color_util {
    use csscolorparser::Color;
    use lru::LruCache;
    use once_cell::sync::Lazy;
    use std::collections::HashMap;
    use std::num::NonZeroUsize;
    use std::str::FromStr;
    use std::sync::Mutex;

    // Named color mapping
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

    // Alias mapping
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

    // 3/6 digit hex validation
    pub fn is_valid_hex(s: &str) -> bool {
        let s = s.trim_start_matches('#');
        let len = s.len();
        (len == 3 || len == 6) && s.chars().all(|c| c.is_ascii_hexdigit())
    }

    // Simplified CSS color validation (supports rgb(a), hsl(a), common formats)
    pub fn is_css_color(s: &str) -> bool {
        Color::from_str(s).is_ok()
    }

    /// Standardizes color input, returning a string usable in SVG or None
    pub fn normalize_color(color: &str) -> Option<String> {
        static CACHE: Lazy<Mutex<LruCache<String, Option<String>>>> =
            Lazy::new(|| Mutex::new(LruCache::new(NonZeroUsize::new(512).unwrap())));
        let color = color.trim();
        if color.is_empty() {
            return None;
        }
        let key = color.to_ascii_lowercase();
        // Check cache first
        if let Some(cached) = {
            let mut cache = CACHE.lock().unwrap();
            cache.get(&key).cloned()
        } {
            return cached;
        }
        // Allocate only if there are uppercase letters
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

    /// Outputs SVG-compatible color (hex string), prioritizing named colors and aliases, otherwise original
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
/// Font width calculation trait, to be implemented and injected by the main project
pub trait FontMetrics {
    /// Supports font-family fallback
    fn get_text_width_px(&self, text: &str, font_family: &str) -> f32;
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Font {
    VerdanaNormal,
    HelveticaBold,
}

/// Calculates the width of text in Verdana 11px (in pixels)
///
/// - Only the text needs to be passed in, the width table is loaded and reused internally
/// - Efficient lazy initialization to avoid repeated IO
/// - Can be directly used in scenarios like SVG badges
pub fn get_text_width(text: &str, font: Font) -> f64 {
    use crate::measurer::CharWidthMeasurer;
    use once_cell::sync::Lazy;

    // 在编译时直接将 JSON 文件内容作为字符串嵌入
    const VERDANA_JSON_DATA: &str = include_str!("../assets/fonts/verdana-11px-normal.json");
    const HELVETICA_JSON_DATA: &str = include_str!("../assets/fonts/helvetica-11px-bold.json");

    static VERDANA_WIDTH_TABLE: Lazy<CharWidthMeasurer> = Lazy::new(|| {
        // 从嵌入的字符串加载数据，而不是从文件系统
        CharWidthMeasurer::load_from_str(VERDANA_JSON_DATA)
            .expect("Unable to parse Verdana 11px width table")
    });

    static HELVETICA_WIDTH_TABLE: Lazy<CharWidthMeasurer> = Lazy::new(|| {
        // 从嵌入的字符串加载数据
        CharWidthMeasurer::load_from_str(HELVETICA_JSON_DATA)
            .expect("Unable to parse Helvetica Bold width table")
    });

    match font {
        Font::VerdanaNormal => VERDANA_WIDTH_TABLE.width_of(text, true),
        Font::HelveticaBold => HELVETICA_WIDTH_TABLE.width_of(text, true),
    }
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
const FONT_SCALE_UP_FACTOR: u32 = 10;
/// Dynamically calculates foreground and shadow colors based on background color (equivalent to JS colorsForBackground)
///
/// - Input: hex color string (supports 3/6 digits, e.g. "#4c1", "#007ec6")
/// - Algorithm:
///   1. Parses hex to RGB
///   2. Calculates brightness = (0.299*R + 0.587*G + 0.114*B) / 255
///   3. If brightness ≤ 0.69, returns ("#fff", "#010101"), otherwise ("#333", "#ccc")
pub fn colors_for_background(hex: &str) -> (&'static str, &'static str) {
    // Remove leading #
    let hex = hex.trim_start_matches('#');
    // Parse RGB
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
        _ => (0, 0, 0), // Invalid input, return black
    };
    // W3C recommended brightness formula
    let brightness = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) / 255.0;
    if brightness <= 0.69 {
        ("#fff", "#010101")
    } else {
        ("#333", "#ccc")
    }
}
pub(crate) fn preferred_width_of(text: &str, font: Font) -> u32 {
    use lru::LruCache;
    use once_cell::sync::Lazy;
    use std::num::NonZeroUsize;
    use std::sync::Mutex;

    // Create a cache that includes font information in the key
    static WIDTH_CACHE: Lazy<Mutex<LruCache<(String, Font), u32>>> =
        Lazy::new(|| Mutex::new(LruCache::new(NonZeroUsize::new(1024).unwrap())));

    let cache_key = (text.to_string(), font.clone());

    {
        let mut cache = WIDTH_CACHE.lock().unwrap();
        if let Some(&cached) = cache.get(&cache_key) {
            return cached;
        }
    }

    let width = get_text_width(text, font);
    let rounded = round_up_to_odd_f64(width);

    if text.len() <= 1024 {
        let mut cache = WIDTH_CACHE.lock().unwrap();
        cache.put(cache_key, rounded);
    }

    rounded
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum BadgeStyle {
    Flat,
    FlatSquare,
    Plastic,
    Social,
}

impl Default for BadgeStyle {
    fn default() -> Self {
        BadgeStyle::Flat
    }
}

pub fn default_message_color() -> &'static str {
    "#007ec6"
}

pub fn default_label_color() -> &'static str {
    "#555"
}

#[derive(Deserialize, Debug)]
pub struct BadgeParams<'a> {
    #[serde(default)]
    pub style: BadgeStyle,
    pub label: Option<&'a str>,
    pub message: &'a str,
    pub label_color: Option<&'a str>,
    pub message_color: &'a str,
    pub link: Option<&'a str>,
    pub extra_link: Option<&'a str>,
    pub logo: Option<&'a str>,
    pub logo_color: Option<&'a str>,
}

/// Public API: Generate SVG string
pub fn render_badge_svg(params: &BadgeParams) -> String {
    render_badge(
        params.style,
        params.label,
        params.message,
        params.label_color,
        params.message_color,
        params.link,
        params.extra_link,
        params.logo,
        params.logo_color,
    )
}

fn create_accessible_text(label: Option<&str>, message: &str) -> String {
    let use_label = match label {
        Some(l) if !l.is_empty() => Some(l),
        _ => None,
    };
    let label_len = use_label.map_or(0, |l| l.len() + 2); // +2 for ": "
    let mut buf = String::with_capacity(label_len + message.len());
    if let Some(label) = use_label {
        buf.push_str(label);
        buf.push_str(": ");
    }
    buf.push_str(message);
    buf
}
// --- General Badge Rendering Function ---
fn render_badge(
    style: BadgeStyle,
    label: Option<&str>,
    message: &str,
    label_color: Option<&str>,
    message_color: &str,
    link: Option<&str>,
    extra_link: Option<&str>,
    logo: Option<&str>,
    logo_color: Option<&str>,
) -> String {
    let default_logo_color = if style == BadgeStyle::Social {
        "#000000"
    } else {
        "whitesmoke"
    };

    let logo_color = logo_color.unwrap_or(default_logo_color);
    let logo_color = to_svg_color(logo_color).unwrap_or(default_logo_color.to_string());
    let icon_svg = match logo {
        Some(logo) => {
            let logo = logo.trim();
            if logo.is_empty() {
                ""
            } else {
                // let logo_color = logo_color.unwrap_or("#555");
                // let icon = to_svg_color(logo_color).unwrap_or("#555".to_string());
                let icon = logo;
                let svg = simpleicons::Icon::get_svg(icon);
                match svg {
                    Some(svg) => svg,
                    None => "",
                }
            }
        }
        None => "",
    };
    // 如果 logo 为 <svg 开头，则需要获取 base64 编码
    // 通过 cargo add base64 来引入 base64 crate
    let logo = if icon_svg.starts_with("<svg") {
        let logo_svg = icon_svg.replace("<svg", format!("<svg fill=\"{}\"", logo_color).as_str());
        let base64_logo = base64::engine::general_purpose::STANDARD.encode(logo_svg);
        format!("data:image/svg+xml;base64,{}", base64_logo)
    } else {
        icon_svg.to_string()
    };
    let has_logo = !logo.is_empty();
    let logo_width = 14;
    let mut logo_padding = 3;
    if label.is_some() && label.unwrap().is_empty() {
        logo_padding = 0;
    }

    let total_logo_width = if has_logo {
        logo_width + logo_padding
    } else {
        0
    };

    let has_label_color = !label_color.unwrap_or("").is_empty();
    let message_color = to_svg_color(message_color).unwrap_or("#007ec6".to_string());
    let mut label_color = label_color.unwrap_or("");

    // 参数: BadgeParams { style: Base(Flat), label: None    , message: "message", label_color: Some(""), message_color: "#4c1", link: Some(""), extra_link: None, logo: Some(""), logo_color: None }
    // 参数: BadgeParams { style: Base(Flat), label: Some(""), message: "message", label_color: Some(""), message_color: "#4c1", link: Some(""), extra_link: None, logo: Some(""), logo_color: None }
    if label.unwrap_or("").is_empty() && label_color.is_empty() {
        if has_logo {
            label_color = "#555";
        } else {
            label_color = message_color.as_str();
        }
    }

    let binding = to_svg_color(label_color).unwrap_or("#555".to_string());
    let label_color = binding.as_str();

    let message_color = message_color.as_str();

    match style {
        BadgeStyle::Flat => {
            let accessible_text = create_accessible_text(label, message);
            let has_label_content = label.is_some() && !label.unwrap().is_empty();
            let has_label = has_label_content || has_label_color;
            let label_margin = total_logo_width + 1;

            let label_width = if has_label && label.is_some() {
                preferred_width_of(label.unwrap(), Font::VerdanaNormal)
            } else {
                0
            };

            let mut left_width = if has_label {
                (label_width + 2 * HORIZONTAL_PADDING + total_logo_width) as i32
            } else {
                0
            };

            if has_label && label.is_some() {
                let label = label.unwrap();
                if label.is_empty() {
                    left_width -= 1;
                }
            }
            let message_width = preferred_width_of(message, Font::VerdanaNormal);

            let offset = if label.is_none() && has_logo {
                -3 as i32
            } else {
                0
            };

            let left_width = left_width + offset as i32;
            let mut message_margin: i32 =
                left_width as i32 - if message.is_empty() { 0 } else { 1 };
            if !has_label {
                if has_logo {
                    message_margin = message_margin + (total_logo_width + HORIZONTAL_PADDING) as i32
                } else {
                    message_margin = message_margin + 1
                }
            }

            let mut right_width = (message_width + 2 * HORIZONTAL_PADDING) as i32;
            if has_logo && !has_label {
                right_width += total_logo_width as i32
                    + if !message.is_empty() {
                        (HORIZONTAL_PADDING - 1) as i32
                    } else {
                        0 as i32
                    };
            }

            let label_x = 10.0
                * (label_margin as f32 + (0.5 * label_width as f32) + HORIZONTAL_PADDING as f32)
                + offset as f32;
            let label_width_scaled = label_width * 10;
            let total_width = left_width + right_width as i32;

            let right_width = right_width + if !has_label_color { offset } else { 0 };
            let hex_label_color = Color::from_str(label_color)
                .unwrap_or(Color::from_str("#555").unwrap())
                .to_css_hex();
            let hex_label_color = hex_label_color.as_str();
            let hex_message_color = Color::from_str(message_color)
                .unwrap_or(Color::from_str("#007ec6").unwrap())
                .to_css_hex();
            let hex_message_color = hex_message_color.as_str();
            let (label_text_color, label_shadow_color) = colors_for_background(hex_label_color);
            let (message_text_color, message_shadow_color) =
                colors_for_background(hex_message_color);
            let rect_offset = if has_logo { 19 } else { 0 };

            let message_link_x = if has_logo
                && !has_label
                && (extra_link.is_none() || !extra_link.unwrap().is_empty())
            {
                total_logo_width as i32 + HORIZONTAL_PADDING as i32
            } else {
                left_width
            };

            let has_extra_link = !extra_link.unwrap_or("").is_empty();
            let message_x = 10.0
                * (message_margin as f32
                    + (0.5 * message_width as f32)
                    + HORIZONTAL_PADDING as f32);
            let message_link_x = message_link_x
                + if !has_label && has_extra_link {
                    offset
                } else {
                    0
                } as i32;
            let message_width_scaled = message_width * 10;
            let left_width = if left_width < 0 { 0 } else { left_width };
            FlatBadgeSvgTemplateContext {
                font_family: FONT_FAMILY,

                accessible_text: accessible_text.as_str(),
                badge_height: BADGE_HEIGHT as i32,

                left_width: left_width as i32,
                right_width: right_width as i32,
                total_width: total_width as i32,

                label_color,
                message_color,

                font_size_scaled: FONT_SIZE_SCALED as i32,

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

                link: link.unwrap_or(""),
                extra_link: extra_link.unwrap_or(""),
                logo: logo.as_str(),

                rect_offset,
                message_link_x,
            }
            .render()
            .unwrap_or_else(|e| format!("<!-- Askama render error: {} -->", e))
        }
        BadgeStyle::FlatSquare => {
            let accessible_text = create_accessible_text(label, message);
            let has_label_content = label.is_some() && !label.unwrap().is_empty();
            let has_label = has_label_content || has_label_color;
            let label_margin = total_logo_width + 1;

            let label_width = if has_label && label.is_some() {
                preferred_width_of(label.unwrap(), Font::VerdanaNormal)
            } else {
                0
            };

            let mut left_width = if has_label {
                (label_width + 2 * HORIZONTAL_PADDING + total_logo_width) as i32
            } else {
                0
            };

            if has_label && label.is_some() {
                let label = label.unwrap();
                if label.is_empty() {
                    left_width -= 1;
                }
            }
            let message_width = preferred_width_of(message, Font::VerdanaNormal);

            let offset = if label.is_none() && has_logo {
                -3 as i32
            } else {
                0
            };

            let left_width = left_width + offset as i32;
            let mut message_margin: i32 =
                left_width as i32 - if message.is_empty() { 0 } else { 1 };
            if !has_label {
                if has_logo {
                    message_margin = message_margin + (total_logo_width + HORIZONTAL_PADDING) as i32
                } else {
                    message_margin = message_margin + 1
                }
            }

            let mut right_width = (message_width + 2 * HORIZONTAL_PADDING) as i32;
            if has_logo && !has_label {
                right_width += total_logo_width as i32
                    + if !message.is_empty() {
                        (HORIZONTAL_PADDING - 1) as i32
                    } else {
                        0 as i32
                    };
            }

            let label_x = 10.0
                * (label_margin as f32 + (0.5 * label_width as f32) + HORIZONTAL_PADDING as f32)
                + offset as f32;
            let label_width_scaled = label_width * 10;
            let total_width = left_width + right_width as i32;

            let right_width = right_width + if !has_label_color { offset } else { 0 };
            let hex_label_color = Color::from_str(label_color)
                .unwrap_or(Color::from_str("#555").unwrap())
                .to_css_hex();
            let hex_label_color = hex_label_color.as_str();
            let hex_message_color = Color::from_str(message_color)
                .unwrap_or(Color::from_str("#007ec6").unwrap())
                .to_css_hex();
            let hex_message_color = hex_message_color.as_str();
            let (label_text_color, _) = colors_for_background(hex_label_color);
            let (message_text_color, _) = colors_for_background(hex_message_color);
            let rect_offset = if has_logo { 19 } else { 0 };

            let message_link_x = if has_logo
                && !has_label
                && (extra_link.is_none() || !extra_link.unwrap().is_empty())
            {
                total_logo_width as i32 + HORIZONTAL_PADDING as i32
            } else {
                left_width
            };

            let has_extra_link = !extra_link.unwrap_or("").is_empty();
            let message_x = 10.0
                * (message_margin as f32
                    + (0.5 * message_width as f32)
                    + HORIZONTAL_PADDING as f32);
            let message_link_x = message_link_x
                + if !has_label && has_extra_link {
                    offset
                } else {
                    0
                } as i32;
            let message_width_scaled = message_width * 10;
            let left_width = if left_width < 0 { 0 } else { left_width };
            FlatSquareBadgeSvgTemplateContext {
                font_family: FONT_FAMILY,
                accessible_text: accessible_text.as_str(),
                badge_height: BADGE_HEIGHT as i32,
                left_width: left_width as i32,
                right_width: right_width as i32,
                total_width: total_width as i32,
                label_color,
                message_color,
                font_size_scaled: FONT_SIZE_SCALED as i32,
                label: label.unwrap_or(""),
                label_x,
                label_width_scaled: label_width_scaled as i32,
                label_text_color,
                message_x,
                message_text_color,
                message_width_scaled: message_width_scaled as i32,
                message,
                link: link.unwrap_or(""),
                extra_link: extra_link.unwrap_or(""),
                logo: logo.as_str(),
                rect_offset,
                message_link_x,
            }
            .render()
            .unwrap_or_else(|e| format!("<!-- Askama render error: {} -->", e))
        }
        BadgeStyle::Plastic => {
            let accessible_text = create_accessible_text(label, message);
            let has_label_content = label.is_some() && !label.unwrap().is_empty();
            let has_label = has_label_content || has_label_color;
            let label_margin = total_logo_width + 1;

            let label_width = if has_label && label.is_some() {
                preferred_width_of(label.unwrap(), Font::VerdanaNormal)
            } else {
                0
            };

            let mut left_width = if has_label {
                (label_width + 2 * HORIZONTAL_PADDING + total_logo_width) as i32
            } else {
                0
            };

            if has_label && label.is_some() {
                let label = label.unwrap();
                if label.is_empty() {
                    left_width -= 1;
                }
            }
            let message_width = preferred_width_of(message, Font::VerdanaNormal);

            let offset = if label.is_none() && has_logo {
                -3 as i32
            } else {
                0
            };

            let left_width = left_width + offset as i32;
            let mut message_margin: i32 =
                left_width as i32 - if message.is_empty() { 0 } else { 1 };
            if !has_label {
                if has_logo {
                    message_margin = message_margin + (total_logo_width + HORIZONTAL_PADDING) as i32
                } else {
                    message_margin = message_margin + 1
                }
            }

            let mut right_width = (message_width + 2 * HORIZONTAL_PADDING) as i32;
            if has_logo && !has_label {
                right_width += total_logo_width as i32
                    + if !message.is_empty() {
                        (HORIZONTAL_PADDING - 1) as i32
                    } else {
                        0 as i32
                    };
            }

            let label_x = 10.0
                * (label_margin as f32 + (0.5 * label_width as f32) + HORIZONTAL_PADDING as f32)
                + offset as f32;
            let label_width_scaled = label_width * 10;
            let total_width = left_width + right_width as i32;

            let right_width = right_width + if !has_label_color { offset } else { 0 };
            let hex_label_color = Color::from_str(label_color)
                .unwrap_or(Color::from_str("#555").unwrap())
                .to_css_hex();
            let hex_label_color = hex_label_color.as_str();
            let hex_message_color = Color::from_str(message_color)
                .unwrap_or(Color::from_str("#007ec6").unwrap())
                .to_css_hex();
            let hex_message_color = hex_message_color.as_str();
            let (label_text_color, label_shadow_color) = colors_for_background(hex_label_color);
            let (message_text_color, message_shadow_color) =
                colors_for_background(hex_message_color);
            let rect_offset = if has_logo { 19 } else { 0 };

            let message_link_x = if has_logo
                && !has_label
                && (extra_link.is_none() || !extra_link.unwrap().is_empty())
            {
                total_logo_width as i32 + HORIZONTAL_PADDING as i32
            } else {
                left_width
            };

            let has_extra_link = !extra_link.unwrap_or("").is_empty();
            let message_x = 10.0
                * (message_margin as f32
                    + (0.5 * message_width as f32)
                    + HORIZONTAL_PADDING as f32);
            let message_link_x = message_link_x
                + if !has_label && has_extra_link {
                    offset
                } else {
                    0
                } as i32;
            let message_width_scaled = message_width * 10;
            let left_width = if left_width < 0 { 0 } else { left_width };
            PlasticBadgeSvgTemplateContext {
                total_width: total_width as i32,
                left_width: left_width as i32,
                right_width: right_width as i32,
                accessible_text: accessible_text.as_str(),
                label: label.unwrap_or(""),
                label_x,
                label_text_length: label_width_scaled as i32,
                label_text_color,
                label_shadow_color,
                message,
                message_x,
                message_text_length: message_width_scaled as i32,
                message_text_color,
                message_shadow_color,
                label_color,
                message_color,
                link: link.unwrap_or(""),
                extra_link: extra_link.unwrap_or(""),
                logo: logo.as_str(),
                rect_offset,
                message_link_x,
            }
            .render()
            .unwrap_or_else(|e| format!("<!-- Askama render error: {} -->", e))
        }
        BadgeStyle::Social => {
            let label_is_none = label.is_none();

            let offset = if label_is_none && has_logo {
                -3 as i32
            } else {
                0 as i32
            };

            let label = label.unwrap_or("");
            let label = capitalize(label).unwrap().to_string();
            let label_str = label.as_str();
            let accessible_text = create_accessible_text(Some(label_str), message);
            let internal_height = 19;
            let label_horizontal_padding = 5;
            let message_horizontal_padding = 4;
            let horizontal_gutter = 6;

            let label_text_width = preferred_width_of(label_str, Font::HelveticaBold);

            let label_rect_width =
                (label_text_width + total_logo_width + 2 * label_horizontal_padding) as i32
                    + offset;

            let message_text_width = preferred_width_of(message, Font::HelveticaBold);

            let message_rect_width = message_text_width + 2 * message_horizontal_padding;
            let has_message = !message.is_empty();

            let message_bubble_main_x = label_rect_width as f32 + horizontal_gutter as f32 + 0.5;
            let message_bubble_notch_x = label_rect_width + horizontal_gutter;
            let label_text_x = FONT_SCALE_UP_FACTOR as f32
                * (total_logo_width as f32
                    + label_text_width as f32 / 2.0
                    + label_horizontal_padding as f32
                    + offset as f32);
            let message_text_x = FONT_SCALE_UP_FACTOR as f32
                * (label_rect_width as f32
                    + horizontal_gutter as f32
                    + message_rect_width as f32 / 2.0);
            let message_text_length = FONT_SCALE_UP_FACTOR * message_text_width;
            let label_text_length = FONT_SCALE_UP_FACTOR * label_text_width;

            let left_width = label_rect_width + 1;
            let right_width = if has_message {
                horizontal_gutter + message_rect_width as i32
            } else {
                0
            };

            let total_width = left_width + right_width as i32;

            SocialBadgeSvgTemplateContext {
                total_width: total_width,
                total_height: BADGE_HEIGHT as i32,
                internal_height: internal_height,
                accessible_text: accessible_text.as_str(),
                message_rect_width: message_rect_width,
                message_bubble_main_x,
                message_bubble_notch_x,
                label_text_length,
                label: label_str,
                message,
                label_text_x,
                message_text_x,
                message_text_length,
                label_rect_width,
                link: link.unwrap_or(""),
                extra_link: extra_link.unwrap_or(""),
                logo: logo.as_str(),
            }
            .render()
            .unwrap_or_else(|e| format!("<!-- Askama render error: {} -->", e))
        }
    }
}
#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use csscolorparser::Color;

    use super::*;
    #[test]
    fn test_svg() {
        // Test SVG rendering
        let params = BadgeParams {
            style: BadgeStyle::FlatSquare,
            label: Some("build"),
            message: "passing",
            label_color: Some("#333"),
            message_color: "#4c1",
            link: None,
            extra_link: None,
            logo: None,
            logo_color: None,
        };
        let svg = render_badge_svg(&params);
        assert!(!svg.is_empty(), "SVG rendering failed");
    }

    #[test]
    fn test_named_color() {
        let params = BadgeParams {
            style: BadgeStyle::FlatSquare,
            label: Some("status"),
            message: "ok",
            label_color: Some("brightgreen"),
            message_color: "blue",
            link: None,
            extra_link: None,
            logo: None,
            logo_color: None,
        };
        let svg = render_badge_svg(&params);
        assert!(
            svg.contains("fill=\"#4c1\""),
            "Named color brightgreen not correctly mapped"
        );
        assert!(
            svg.contains("fill=\"#007ec6\""),
            "Named color blue not correctly mapped"
        );
    }

    #[test]
    fn test_alias_color() {
        let params = BadgeParams {
            style: BadgeStyle::FlatSquare,
            label: Some("status"),
            message: "ok",
            label_color: Some("gray"),
            message_color: "critical",
            link: None,
            extra_link: None,
            logo: None,
            logo_color: None,
        };
        let svg = render_badge_svg(&params);
        assert!(
            svg.contains("fill=\"#555\""),
            "Alias gray not correctly mapped"
        );
        assert!(
            svg.contains("fill=\"#e05d44\""),
            "Alias critical not correctly mapped"
        );
    }

    #[test]
    fn test_hex_color() {
        let params = BadgeParams {
            style: BadgeStyle::FlatSquare,
            label: Some("hex"),
            message: "ok",
            label_color: Some("#4c1"),
            message_color: "dfb317",
            link: None,
            extra_link: None,
            logo: None,
            logo_color: None,
        };
        let svg = render_badge_svg(&params);
        assert!(
            svg.contains("fill=\"#4c1\""),
            "3-digit hex not correctly processed"
        );
        assert!(
            svg.contains("fill=\"#dfb317\""),
            "6-digit hex not correctly processed"
        );
    }

    #[test]
    fn test_css_color() {
        let params = BadgeParams {
            style: BadgeStyle::FlatSquare,
            label: Some("css"),
            message: "ok",
            label_color: Some("rgb(0,128,0)"),
            message_color: "hsl(120,100%,25%)",
            link: None,
            extra_link: None,
            logo: None,
            logo_color: None,
        };
        let svg = render_badge_svg(&params);
        assert!(
            svg.contains(r#"fill="rgb(0,128,0)""#),
            "CSS rgb color not correctly processed"
        );
        assert!(
            svg.contains(r#"fill="hsl(120,100%,25%)""#),
            "CSS hsl color not correctly processed"
        );
    }

    #[test]
    fn test_invalid_color_fallback() {
        let params = BadgeParams {
            style: BadgeStyle::FlatSquare,
            label: Some("bad"),
            message: "ok",
            label_color: Some("notacolor"),
            message_color: "",
            link: None,
            extra_link: None,
            logo: None,
            logo_color: None,
        };
        let svg = render_badge_svg(&params);
        assert!(
            svg.contains("fill=\"#555\""),
            "Invalid label_color did not fallback to default color"
        );
        assert!(
            svg.contains("fill=\"#007ec6\""),
            "Empty message_color did not fallback to default color"
        );
    }

    #[test]
    fn test_color() {
        // 解析名称
        let c = Color::from_str("red").unwrap();
        println!("{:?}", c);

        // 解析HEX
        let c = Color::from_str("#ff0080").unwrap();
        println!("{:?}", c);

        // 解析RGBA
        let c = Color::from_str("rgba(255,255,0,0.75)").unwrap();
        println!("{:?}", c);

        // 解析HSL
        let c = Color::from_str("hsl(120, 100%, 50%)").unwrap();
        println!("{:?}", c);

        let c = Color::from_str("notexists").is_err();
        println!("{:?}", c);
    }
}
