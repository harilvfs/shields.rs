use shields::{BadgeParams, BadgeStyle, BaseBadgeStyle, render_badge_svg};

use pretty_assertions::assert_eq;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;

fn shields_io_url(params: &BadgeParams) -> String {
    let style = match params.style {
        BadgeStyle::Base(BaseBadgeStyle::Flat) => "flat",
        BadgeStyle::Base(BaseBadgeStyle::Plastic) => "plastic",
        BadgeStyle::Base(BaseBadgeStyle::FlatSquare) => "flat-square",
        BadgeStyle::Social => "social",
    };
    let url = if params.label.is_some() {
        format!(
            "https://img.shields.io/badge/{}-{}-blue?style={}",
            params.label.as_ref().unwrap(),
            params.message.replace(" ", "%20"),
            style
        )
    } else {
        format!(
            "https://img.shields.io/badge/{}-blue?style={}",
            params.message.replace(" ", "%20"),
            style
        )
    };
    let queries = [
        ("labelColor", params.label_color.unwrap_or("")),
        ("color", params.message_color),
        ("link", params.link.unwrap_or("")),
        ("link", params.extra_link.unwrap_or("")),
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
    use std::collections::BTreeMap;
    use xmltree::{Element, XMLNode};

    // 过滤和排序属性
    fn filter_and_sort_attrs(attrs: &mut BTreeMap<String, String>) {
        let geom_keys = [
            "x", "y", "width", "height", "cx", "cy", "r", "rx", "ry", "d", "points", "x1", "y1",
            "x2", "y2",
        ];
        let mut keep = BTreeMap::new();
        // 先收集需要的属性
        for (k, v) in attrs.iter() {
            if k.starts_with("data-")
                || k.contains(':') && (k.starts_with("inkscape:") || k.starts_with("sodipodi:"))
            {
                continue;
            }
            keep.insert(k.clone(), v.clone());
        }
        attrs.clear();
        // 按优先级排序
        let mut ordered = Vec::new();
        for key in ["id", "class", "name"] {
            if let Some(v) = keep.remove(key) {
                ordered.push((key.to_string(), v));
            }
        }
        for key in geom_keys {
            if let Some(v) = keep.remove(key) {
                ordered.push((key.to_string(), v));
            }
        }
        if let Some(v) = keep.remove("style") {
            ordered.push(("style".to_string(), v));
        }
        // 剩余按字典序
        let mut rest: Vec<_> = keep.into_iter().collect();
        rest.sort_by(|a, b| a.0.cmp(&b.0));
        ordered.extend(rest);
        for (k, v) in ordered {
            attrs.insert(k, v);
        }
    }

    // 合并多余空格
    fn normalize_text(text: &str, in_text_tag: bool) -> String {
        if in_text_tag {
            // <text> 内保留有意义空格，合并多余空格
            let mut s = String::new();
            let mut last_space = false;
            for c in text.chars() {
                if c.is_whitespace() {
                    if !last_space {
                        s.push(' ');
                        last_space = true;
                    }
                } else {
                    s.push(c);
                    last_space = false;
                }
            }
            s
        } else {
            text.split_whitespace().collect::<Vec<_>>().join(" ")
        }
    }

    // 递归格式化节点
    fn format_element(elem: &Element, indent: usize, out: &mut String) {
        let indent_str = "  ".repeat(indent);
        out.push_str(&indent_str);
        out.push('<');
        out.push_str(&elem.name);

        // 处理属性
        let mut attrs: BTreeMap<String, String> = elem.attributes.clone().into_iter().collect();
        filter_and_sort_attrs(&mut attrs);
        for (k, v) in attrs {
            out.push(' ');
            out.push_str(&k);
            out.push_str("=\"");
            out.push_str(&v.replace('"', "&quot;"));
            out.push('"');
        }

        // 处理子节点
        let mut has_children = false;
        let mut text_content = String::new();
        for node in &elem.children {
            match node {
                XMLNode::Element(_) => {
                    has_children = true;
                }
                XMLNode::Text(t) => {
                    if t.trim().is_empty() {
                        continue;
                    }
                    text_content.push_str(t);
                }
                _ => {}
            }
        }
        let is_text_tag = elem.name == "text";
        if elem.children.is_empty() {
            out.push_str(" />\n");
        } else if !has_children && !text_content.is_empty() {
            // 只有文本
            out.push('>');
            out.push_str(&normalize_text(&text_content, is_text_tag));
            out.push_str("</");
            out.push_str(&elem.name);
            out.push_str(">\n");
        } else {
            out.push_str(">\n");
            for node in &elem.children {
                match node {
                    XMLNode::Element(e) => {
                        format_element(e, indent + 1, out);
                    }
                    XMLNode::Text(t) => {
                        let txt = normalize_text(t, is_text_tag);
                        if !txt.is_empty() {
                            out.push_str(&"  ".repeat(indent + 1));
                            out.push_str(&txt);
                            out.push('\n');
                        }
                    }
                    _ => {}
                }
            }
            out.push_str(&indent_str);
            out.push_str("</");
            out.push_str(&elem.name);
            out.push_str(">\n");
        }
    }

    // 解析 SVG
    let mut reader = svg.as_bytes();
    let doc = match Element::parse(&mut reader) {
        Ok(e) => e,
        Err(_) => return svg.to_string(),
    };

    // 检查 XML 声明
    let xml_decl = if svg.trim_start().starts_with("<?xml") {
        Some("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n")
    } else {
        None
    };

    // 格式化输出
    let mut out = String::new();
    if let Some(decl) = xml_decl {
        out.push_str(decl);
    }
    format_element(&doc, 0, &mut out);
    out
}

#[test]
fn test_svg_compare() {
    let label_selections = vec![Some("label"), Some("label with spaces"), Some(""), None];
    let message_selections = vec!["message", "message with spaces", ""];
    let label_color_selections = vec![Some("blue"), Some("#4c1"), Some("#4c3232"), Some(""), None];
    let message_color_selections = vec!["#4c1", "#e05d44", "#4c3232", ""];
    let link_selections = vec![
        Some("https://example.com"),
        Some("https://example.com/longer-link"),
        Some(""),
        None,
    ];
    let extra_link_selections = vec![None];

    let mut test_cases = vec![];
    for label in label_selections.iter() {
        for message in message_selections.iter() {
            for label_color in label_color_selections.iter() {
                for message_color in message_color_selections.iter() {
                    for link in link_selections.iter() {
                        for extra_link in extra_link_selections.iter() {
                            let params = BadgeParams {
                                style: BadgeStyle::Base(BaseBadgeStyle::Flat),
                                label: *label,
                                message,
                                label_color: *label_color,
                                message_color,
                                link: *link,
                                extra_link: *extra_link,
                            };
                            test_cases.push(params);
                        }
                    }
                }
            }
        }
    }

    for params in test_cases {
        println!("测试参数: {:?}", params);
        let local_svg = render_badge_svg(&params);
        let url = shields_io_url(&params);
        println!("url: {:?}", url);
        let local_svg_norm = normalize_svg(&local_svg);
        let shields_svg = get_shields_svg_with_cache(&params, &url);
        let shields_svg_norm = normalize_svg(&shields_svg);

        // Save
        let cache_dir = Path::new("tests/cache");
        if !cache_dir.exists() {
            fs::create_dir_all(cache_dir).expect("创建 cache 目录失败");
        }

        let file_name_local = format!("tests/svg_local.svg");
        let file_name_shields = format!("tests/svg_shields.svg");
        let mut file_local = fs::File::create(&file_name_local).expect("创建本地 SVG 文件失败");
        file_local
            .write_all(local_svg.as_bytes())
            .expect("写入本地 SVG 文件失败");
        let mut file_shields =
            fs::File::create(&file_name_shields).expect("创建 shields SVG 文件失败");
        file_shields
            .write_all(shields_svg.as_bytes())
            .expect("写入 shields SVG 文件失败");

        assert_eq!(
            local_svg_norm, shields_svg_norm,
            "SVG 不一致\n参数: {:?}\n本地 SVG:\n{}\nshields.io SVG:\n{}",
            params, local_svg, shields_svg
        );
    }
}
