#![allow(unused)]

use normalize_path::NormalizePath;
use std::path::Path;

pub fn encode_uri(uri: &str) -> String {
    uri.chars()
        .map(|it| {
            match it {
                // a-zA-Z0-9
                _ if it.is_ascii_alphanumeric() => it.to_string(),
                '/' | '+' | '-' | '_' | '.' | '~' => it.to_string(),
                _ => {
                    let mut char = [0; 3];
                    it.encode_utf8(&mut char);
                    let str = char[..it.len_utf8()].iter().map(|it| format!("%{:X}", it));
                    str.collect::<Vec<String>>().join("")
                }
            }
        })
        .collect::<Vec<String>>()
        .join("")
}

#[test]
fn test_encode_uri() {
    assert_eq!(encode_uri("你好"), "%E4%BD%A0%E5%A5%BD");
    assert_eq!(encode_uri("abc"), "abc");
    assert_eq!(
        encode_uri("你 好/abc你好a+b%c"),
        "%E4%BD%A0%20%E5%A5%BD/abc%E4%BD%A0%E5%A5%BDa+b%25c"
    );
}

pub fn decode_uri(uri: &str) -> Option<String> {
    let mut result = String::with_capacity(uri.len());
    let mut chars = uri.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '%' {
            let mut encoded_bytes = Vec::new();

            loop {
                let a = chars.next()?;
                let b = chars.next()?;
                let byte = u8::from_str_radix(&format!("{}{}", a, b), 16).ok()?;
                encoded_bytes.push(byte);

                // 检查下一个字符是否是%，如果是则继续解码
                match chars.peek() {
                    Some('%') => {
                        chars.next(); // 消耗掉%
                    }
                    _ => break,
                }
            }

            result.push_str(&String::from_utf8(encoded_bytes).ok()?);
        } else {
            result.push(c);
        }
    }

    Some(result)
}

#[test]
fn test_decode_uri() {
    assert_eq!("你+好", decode_uri("%E4%BD%A0+%E5%A5%BD").unwrap());
    assert_eq!("abc", decode_uri("abc").unwrap());
    assert_eq!(
        "你 好/abc你好a+b%c",
        decode_uri("%E4%BD%A0%20%E5%A5%BD/abc%E4%BD%A0%E5%A5%BDa+b%25c").unwrap()
    );
    assert_eq!(
        decode_uri("Hello%20World%21"),
        Some("Hello World!".to_string())
    );
    assert_eq!(decode_uri("%E4%B8%AD%E6%96%87"), Some("中文".to_string()));
    assert_eq!(decode_uri("%%"), None); // 无效编码
    assert_eq!(decode_uri("%2"), None); // 不完整编码
}

pub fn normalize_path(path: &str) -> String {
    let path = Path::new(path);
    path.normalize().display().to_string().replace("\\", "/")
}

#[test]
fn test_normalize_path() {
    // 相对路径
    assert_eq!(normalize_path("a/../b"), "b");
    assert_eq!(normalize_path("./a/b/../c"), "a/c");
    assert_eq!(normalize_path("./a/b/..//c"), "a/c");
    assert_eq!(normalize_path("./a/b/..//c/.."), "a");
    assert_eq!(normalize_path("./a/b/..//c/../.."), "");
    // 绝对路径
    assert_eq!(normalize_path("/a/../b"), "/b");
    assert_eq!(normalize_path("/a/..//b"), "/b");
    assert_eq!(normalize_path("/a/..//b//../.."), "/");
}
