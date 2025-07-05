use normalize_path::NormalizePath;
use std::path::Path;

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
