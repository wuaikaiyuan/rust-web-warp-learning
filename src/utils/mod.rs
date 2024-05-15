// 将逗号隔开的字符串转成json数组
pub fn comma_separated_to_json_array(
    input: &str,
) -> Result<serde_json::Value, serde_json::Error> {
    // 添加开头的方括号和结尾的方括号，
    // 同时在每个元素前后添加双引号以符合JSON格式要求
    let json_text = format!(
        "[{}]",
        input.replace(',', ",\"\",\"").replace(',', "\",\"")
    );

    // 使用 serde_json::from_str 将格式化后的字符串解析为
    // serde_json::Value，后者可以表示任何有效的JSON值
    serde_json::from_str(&json_text)
}

pub fn parse_tags(tags: &str) -> Option<Vec<String>> {
    let gg = tags
        .trim()
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty()) // 过滤空标签
        .map(|s| s.to_string())
        .collect();
    Some(gg)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_comma_separated_to_json_array() {
        let comma_separated = "apple,banana,orange, 文学, 诗词";

        match comma_separated_to_json_array(comma_separated) {
            Ok(json_array) => {
                println!("Parsed JSON Array: {:?}", json_array)
            }
            Err(e) => println!("Failed to parse: {}", e),
        }
    }

    #[test]
    fn test_parse_tags() {
        let tags = "  ";
        println!("{}", tags.len());
        //let res = parse_tags(tags);
        //println!("{:?}", res);
    }
}
