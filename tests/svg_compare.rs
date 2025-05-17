use shields::{BadgeStyle, BaseBadgeStyle, RenderBadgeParams, render_badge_svg};

use pretty_assertions::assert_eq;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;

fn shields_io_url(params: &RenderBadgeParams) -> String {
    let style = match params.style {
        BadgeStyle::Base(BaseBadgeStyle::Flat) => "flat",
        BadgeStyle::Base(BaseBadgeStyle::Plastic) => "plastic",
        BadgeStyle::Base(BaseBadgeStyle::FlatSquare) => "flat-square",
        BadgeStyle::Social => "social",
    };
    if params.label.is_none() {
        return format!(
            "https://img.shields.io/badge/{}-{}?style={}",
            urlencoding::encode(params.message),
            params.message_color.trim_start_matches('#'),
            style
        );
    }
    format!(
        "https://img.shields.io/badge/{}-{}-{}?style={}&labelColor={}",
        urlencoding::encode(params.label.unwrap()),
        urlencoding::encode(params.message),
        params.message_color.trim_start_matches('#'),
        style,
        params.label_color.trim_start_matches('#')
    )
}

/**
 * 生成唯一缓存文件名（基于参数文本，避免非法字符）
 */
fn cache_file_name_from_params(params: &RenderBadgeParams) -> String {
    let style = match params.style {
        BadgeStyle::Base(BaseBadgeStyle::Flat) => "flat",
        BadgeStyle::Base(BaseBadgeStyle::Plastic) => "plastic",
        BadgeStyle::Base(BaseBadgeStyle::FlatSquare) => "flat-square",
        BadgeStyle::Social => "social",
    };
    let label = params.label.unwrap_or("");
    let message = params.message;
    let label_color = params.label_color;
    let message_color = params.message_color;
    // 拼接参数并做 URL 编码，避免非法文件名
    let file_name = format!(
        "label={}&message={}&label_color={}&message_color={}&style={}.svg",
        urlencoding::encode(label),
        urlencoding::encode(message),
        urlencoding::encode(label_color),
        urlencoding::encode(message_color),
        style
    );
    file_name
}

/// 获取 shields.io SVG，带本地缓存
fn get_shields_svg_with_cache(params: &RenderBadgeParams, url: &str) -> String {
    let cache_dir = Path::new("tests/cache");
    if !cache_dir.exists() {
        fs::create_dir_all(cache_dir).expect("创建 cache 目录失败");
    }
    let file_name = cache_file_name_from_params(params);
    let cache_path = cache_dir.join(file_name);

    // 优先读取缓存
    if cache_path.exists() {
        let mut file = fs::File::open(&cache_path).expect("读取缓存文件失败");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("读取缓存内容失败");
        return contents;
    }

    // 无缓存则请求并写入缓存
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .expect("创建 HTTP 客户端失败");
    let resp = client.get(url).send();
    let resp = match resp {
        Ok(r) => r,
        Err(e) => panic!(
            "HTTP 请求失败: {}\n请检查网络连接，或手动访问 shields.io 以生成缓存。\n错误详情: {}",
            url, e
        ),
    };
    assert!(
        resp.status().is_success(),
        "shields.io 请求失败: {}\nHTTP 状态: {}\n请检查 shields.io 是否可用。",
        url,
        resp.status()
    );
    let svg = resp.text().unwrap_or_else(|e| {
        panic!(
            "读取 SVG 失败: {}\n请检查 shields.io 响应内容。\n错误详情: {}",
            url, e
        )
    });

    let mut file = fs::File::create(&cache_path).expect("写入缓存文件失败");
    file.write_all(svg.as_bytes()).expect("写入缓存内容失败");

    svg
}

fn normalize_svg(svg: &str) -> String {
    // 可根据需要扩展：去除生成时间、id 等无关属性
    let mut normalized_svg = svg.replace("\n", "");
    normalized_svg = normalized_svg.replace("\r", "");
    normalized_svg = normalized_svg.replace(" ", "");
    normalized_svg = normalized_svg.replace("'", "\"");
    normalized_svg = normalized_svg.replace("xmlns=\"http://www.w3.org/2000/svg\"", "");
    return normalized_svg;
}

#[test]
fn test_svg_compare() {
    let test_cases = vec![
        // 常规
        RenderBadgeParams {
            style: BadgeStyle::flat(),
            label: Some("build"),
            message: "passing",
            label_color: "#555",
            message_color: "#4c1",
        },
        // 边界：空 label
        RenderBadgeParams {
            style: BadgeStyle::flat(),
            label: None,
            message: "nonasdasdas31dde",
            label_color: "#555",
            message_color: "#e05d44",
        },
        // 边界：空 label
        RenderBadgeParams {
            style: BadgeStyle::flat_square(),
            label: None,
            message: "nonasdasdas31dde",
            label_color: "#555",
            message_color: "#e05d44",
        },
        // 边界：长 message
        RenderBadgeParams {
            style: BadgeStyle::flat_square(),
            label: Some("coverage"),
            message: "100% coverage achieved",
            label_color: "#007ec6",
            message_color: "#007ec6",
        },
        // // 特殊字符
        RenderBadgeParams {
            style: BadgeStyle::flat(),
            label: Some("build"),
            message: "✓",
            label_color: "#555",
            message_color: "#44cc11",
        },
        RenderBadgeParams {
            style: BadgeStyle::flat_square(),
            label: Some("build"),
            message: "✓",
            label_color: "#555",
            message_color: "#44cc11",
        },
        // 不同 style
        RenderBadgeParams {
            style: BadgeStyle::plastic(),
            label: Some("stars"),
            message: "1234",
            label_color: "#555",
            message_color: "#f7b93e",
        },
        RenderBadgeParams {
            style: BadgeStyle::plastic(),
            label: None,
            message: "1234",
            label_color: "#555",
            message_color: "#f7b93e",
        },
    ];

    for params in test_cases {
        let local_svg = render_badge_svg(&params);
        let url = shields_io_url(&params);
        let local_svg_norm = normalize_svg(&local_svg);
        let shields_svg = get_shields_svg_with_cache(&params, &url);
        let shields_svg_norm = normalize_svg(&shields_svg);
        assert_eq!(
            local_svg_norm, shields_svg_norm,
            "SVG 不一致\n参数: {:?}\n本地 SVG:\n{}\nshields.io SVG:\n{}",
            params, local_svg, shields_svg
        );
    }
}
