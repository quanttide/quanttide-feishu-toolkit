//! 包骨架冒烟：领域名与版本常量。

use quanttide_feishu::{DOMAIN, VERSION};

/// 领域名固定，与 Python 包同源。
#[test]
fn 领域名固定() {
    assert_eq!(DOMAIN, "feishu-platform");
}

/// 版本取自 `Cargo.toml`，编译期注入，不该为空。
#[test]
fn 版本非空() {
    assert!(!VERSION.is_empty());
}
