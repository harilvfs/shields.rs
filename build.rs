// build.rs
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    // 告诉 Cargo 如果模板源文件改变了，就重新运行此构建脚本
    println!("cargo:rerun-if-changed=build.rs");

    for file in [
        "templates/flat_badge_template.svg",
        "templates/flat_square_badge_template.svg",
        "templates/plastic_badge_template.svg",
        "templates/social_badge_template.svg",
    ] {
        println!("cargo:rerun-if-changed={}", file);
        let path = Path::new(file);
        let dest = path.with_extension("min.svg");

        // 读取源文件内容
        let content = fs::read_to_string(&path).unwrap();
        // 移除换行
        let min_content = content
            .lines()
            .map(|line| line.trim())
            .collect::<Vec<_>>()
            .join("");
        // 连续空格合并为一个
        let min_content = min_content
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        // " />" 替换为 "/>"
        let min_content = min_content.replace(" />", "/>");
        // "> <" 替换为 "><"
        let min_content = min_content.replace("> <", "><");
        
        // 将压缩后的内容写入目标文件
        fs::write(&dest, min_content).unwrap();
    }
}
