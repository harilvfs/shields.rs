//! 字符宽度表消费器，等价于 JS 版 CharWidthTableConsumer。
//! 支持从 JSON 文件加载字体宽度表，查找表查找字符宽度，字符串宽度计算，接口风格与 JS 保持一致。

use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::io::{self};

/// 字符宽度表消费器，等价于 JS 版 CharWidthTableConsumer。
pub struct CharWidthMeasurer {
    /// 查找表：char_code -> width
    hash_map: HashMap<u32, f64>,
    /// 'm' 字符的宽度
    pub em_width: f64,
}

impl CharWidthMeasurer {
    /// 判断是否为控制字符（ASCII 0-31 或 127）
    pub fn is_control_char(char_code: u32) -> bool {
        char_code <= 31 || char_code == 127
    }

    /// 从二维数组数据创建实例
    pub fn from_data(data: Vec<(u32, u32, f64)>) -> Self {
        // 构建查找表：将所有区间展开为 char_code -> width
        let mut hash_map = HashMap::new();
        for &(lower, upper, width) in &data {
            for code in lower..=upper {
                hash_map.insert(code, width);
            }
        }
        // emWidth 取 'm' 字符宽度
        let mut consumer = CharWidthMeasurer {
            hash_map,
            em_width: 0.0,
        };
        consumer.em_width = consumer.width_of("m", true);
        consumer
    }

    /// 从 JSON 文件加载（同步）
    pub fn load_sync(path: &str) -> io::Result<Self> {
        let json_str = fs::read_to_string(path)?;
        let value: Value = serde_json::from_str(&json_str)?;
        let arr = value
            .as_array()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "JSON 不是数组"))?;
        let mut data = Vec::with_capacity(arr.len());
        for item in arr {
            let triple = item
                .as_array()
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "子项不是数组"))?;
            if triple.len() != 3 {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "子项长度不是3"));
            }
            let lower = triple[0]
                .as_u64()
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "lower 不是整数"))?
                as u32;
            let upper = triple[1]
                .as_u64()
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "upper 不是整数"))?
                as u32;
            let width = triple[2]
                .as_f64()
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "width 不是浮点数"))?;
            data.push((lower, upper, width));
        }
        Ok(CharWidthMeasurer::from_data(data))
    }

    /// 查找单个字符码点的宽度，等价于 JS widthOfCharCode
    /// 控制字符宽度为 0，未命中返回 None
    pub fn width_of_char_code(&self, char_code: u32) -> Option<f64> {
        if Self::is_control_char(char_code) {
            return Some(0.0);
        }
        // 直接用哈希表查找字符宽度
        // 查找表在初始化时已将所有区间展开为 char_code -> width
        self.hash_map.get(&char_code).copied()
    }

    /// 计算字符串宽度，guess=true 时遇未知字符用 emWidth 替代，否则报错
    pub fn width_of(&self, text: &str, guess: bool) -> f64 {
        let mut total = 0.0;
        for ch in text.chars() {
            let code = ch as u32;
            match self.width_of_char_code(code) {
                Some(width) => total += width,
                None => {
                    if guess {
                        total += self.em_width;
                    } else {
                        panic!("No width available for character code {}", code);
                    }
                }
            }
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_control_chars() {
        assert!(CharWidthMeasurer::is_control_char(0));
        assert!(CharWidthMeasurer::is_control_char(31));
        assert!(CharWidthMeasurer::is_control_char(127));
        assert!(!CharWidthMeasurer::is_control_char(32));
        assert!(!CharWidthMeasurer::is_control_char(128));
    }

    #[test]
    fn test_from_data() {
        let data = vec![(65, 90, 10.0), (97, 122, 8.0)]; // A-Z 10宽度, a-z 8宽度
        let measurer = CharWidthMeasurer::from_data(data);

        assert_eq!(measurer.width_of_char_code(65), Some(10.0)); // 'A'
        assert_eq!(measurer.width_of_char_code(90), Some(10.0)); // 'Z'
        assert_eq!(measurer.width_of_char_code(97), Some(8.0)); // 'a'
        assert_eq!(measurer.width_of_char_code(122), Some(8.0)); // 'z'
        assert_eq!(measurer.width_of_char_code(64), None); // '@'
    }

    #[test]
    fn test_width_of() {
        let data = vec![
            (65, 90, 10.0),   // A-Z 10宽度
            (97, 122, 8.0),   // a-z 8宽度
            (109, 109, 16.0), // 特别设置 'm' 宽度为16，以便测试
        ];
        let measurer = CharWidthMeasurer::from_data(data);

        // 检查 em_width 是否正确设置
        assert_eq!(measurer.em_width, 16.0);

        // 测试字符串宽度计算
        assert_eq!(measurer.width_of("ABC", true), 30.0);
        assert_eq!(measurer.width_of("abc", true), 24.0);
        assert_eq!(measurer.width_of("Am", true), 26.0);

        // 测试猜测模式
        assert_eq!(measurer.width_of("A測", true), 10.0 + 16.0); // '測'未知，用em_width
    }

    #[test]
    #[should_panic(expected = "No width available for character code")]
    fn test_width_of_no_guess() {
        let data = vec![(65, 90, 10.0)];
        let measurer = CharWidthMeasurer::from_data(data);
        measurer.width_of("A測", false); // 应该为'測'panic
    }
}
