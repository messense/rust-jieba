extern crate rust_jieba;

fn jieba() -> rust_jieba::Jieba {
    rust_jieba::Jieba::from_dir("cjieba-sys/cppjieba-cabi/cppjieba/dict")
}

#[test]
fn test_jieba() {
    let jieba = jieba();
    let ret = jieba.cut("南京市长江大桥", true);
    assert_eq!(vec!["南京市", "长江大桥"], ret);
}
