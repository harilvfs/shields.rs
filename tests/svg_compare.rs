use shields::{BadgeParams, BadgeStyle, render_badge_svg};

use pretty_assertions::assert_eq;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;

fn shields_io_url(params: &BadgeParams) -> String {
    let style = match params.style {
        BadgeStyle::Flat => "flat",
        BadgeStyle::Plastic => "plastic",
        BadgeStyle::FlatSquare => "flat-square",
        BadgeStyle::Social => "social",
    };
    let url = if params.label.is_some() {
        format!(
            "https://img.shields.io/badge/{}-{}-blue?style={}",
            params.label.as_ref().unwrap(),
            params.message.unwrap_or("").replace(" ", "%20"),
            style
        )
    } else {
        format!(
            "https://img.shields.io/badge/{}-blue?style={}",
            params.message.unwrap_or("").replace(" ", "%20"),
            style
        )
    };
    let queries = [
        ("labelColor", params.label_color.unwrap_or("")),
        ("color", params.message_color.unwrap_or("")),
        ("link", params.link.unwrap_or("")),
        ("link", params.extra_link.unwrap_or("")),
        ("logo", params.logo.unwrap_or("")),
        ("logoColor", params.logo_color.unwrap_or("")),
    ];
    let mut url = format!("{}&", url);
    for (key, value) in queries.iter() {
        if !value.is_empty() {
            url.push_str(&format!("{}={}&", key, urlencoding::encode(value)));
        }
    }
    url.pop();
    url
}

/**
 * 生成唯一缓存文件名（基于参数文本，避免非法字符）
 */
fn cache_file_name_from_params(params: &BadgeParams) -> String {
    let url = shields_io_url(params);
    urlencoding::encode(&url)
        .replace("%", "_")
        .replace("/", "_")
        .replace(":", "_")
        .replace("?", "_")
        .replace("&", "_")
        .replace("=", "_")
        + ".svg"
}

/// 获取 shields.io SVG，带本地缓存
fn get_shields_svg_with_cache(params: &BadgeParams, url: &str) -> String {
    let cache_dir = Path::new("target/tmp/cache");
    if !cache_dir.exists() {
        fs::create_dir_all(cache_dir).expect("创建 cache 目录失败");
    }
    let file_name = cache_file_name_from_params(params);
    // 编码文件成 MD5
    let file_name = format!("{:x}.svg", md5::compute(file_name));
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

    let mut file = fs::File::create(&cache_path)
        .expect(format!("创建缓存文件失败: {:?}", cache_path.display()).as_str());
    file.write_all(svg.as_bytes()).expect("写入缓存内容失败");

    svg
}

#[test]
fn test_svg_compare() {
    let label_selections = vec![Some("label"), Some(""), None];
    let message_selections = vec!["message", ""];
    let label_color_selections = vec![Some("blue"), Some("#4c1"), Some(""), None, Some("#FFF")];
    let message_color_selections = vec!["blue", "#4c3232", "", "#FFF"];
    let links_selections = vec![
        vec![None, None],
        vec![Some(""), None],
        vec![Some("https://example.com"), None],
        vec![Some("https://example.com"), Some("https://example2.com")],
        vec![Some("https://example.com"), Some("")],
    ];
    let logo_selections = vec![Some("rust"), Some(""), None];
    let style_selections = vec![
        BadgeStyle::Flat,
        BadgeStyle::Plastic,
        BadgeStyle::FlatSquare,
        BadgeStyle::Social,
    ];
    let logo_color_selections = vec![Some("blue"), None];
    let mut test_cases = vec![];
    for label in label_selections.iter() {
        for message in message_selections.iter() {
            for label_color in label_color_selections.iter() {
                for message_color in message_color_selections.iter() {
                    for links in links_selections.iter() {
                        for logo in logo_selections.iter() {
                            for logo_color in logo_color_selections.iter() {
                                for style in style_selections.iter() {
                                    if links.len() < 2 {
                                        continue;
                                    }
                                    let link = links[0].clone();
                                    let extra_link = links[1].clone();
                                    if link.is_none() && extra_link.is_none() {
                                        continue;
                                    }
                                    let params = BadgeParams {
                                        style: *style,
                                        label: *label,
                                        message: Some(message),
                                        label_color: *label_color,
                                        message_color: Some(message_color),
                                        link: links[0],
                                        extra_link: links[1],
                                        logo: *logo,
                                        logo_color: *logo_color,
                                    };
                                    test_cases.push(params);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // let mut test_case_count = 0;
    for params in test_cases {
        let local_svg = render_badge_svg(&params);
        let url = shields_io_url(&params);
        let shields_svg = get_shields_svg_with_cache(&params, &url);

        // Save
        let cache_dir = Path::new("target/tmp/cache");
        if !cache_dir.exists() {
            fs::create_dir_all(cache_dir).expect("创建 cache 目录失败");
        }

        assert_eq!(
            local_svg, shields_svg,
            "SVG 不一致\n参数: {:?}\n本地 SVG:\n{}\nshields.io SVG:\n{}",
            params, local_svg, shields_svg
        );
    }
}
#[test]
fn test_svg_fast_compare() {
    let params = BadgeParams {
        style: BadgeStyle::Flat,
        label: Some("label"),
        message: Some("message"),
        label_color: Some("white"),
        message_color: Some("fff"),
        link: None,
        extra_link: None,
        logo: Some("rust"),
        logo_color: Some("blue"),
    };
    let local_svg = render_badge_svg(&params);
    let url = shields_io_url(&params);
    let shields_svg = get_shields_svg_with_cache(&params, &url);
    let file_name_local = format!("target/tmp/svg_local.svg");
    let file_name_shields = format!("target/tmp/svg_shields.svg");
    let mut file_local = fs::File::create(&file_name_local).expect("创建本地 SVG 文件失败");
    file_local
        .write_all(local_svg.as_bytes())
        .expect("写入本地 SVG 文件失败");
    let mut file_shields = fs::File::create(&file_name_shields).expect("创建 shields SVG 文件失败");
    file_shields
        .write_all(shields_svg.as_bytes())
        .expect("写入 shields SVG 文件失败");
    assert_eq!(
        local_svg, shields_svg,
        "SVG 不一致\n参数: {:?}\n本地 SVG:\n{}\nshields.io SVG:\n{}",
        params, local_svg, shields_svg
    );
}
