extern crate rust_jieba;

fn jieba() -> rust_jieba::Jieba {
    rust_jieba::Jieba::from_dir("cjieba-sys/cppjieba-cabi/cppjieba/dict")
}

#[test]
fn test_jieba() {
    let jieba = jieba();
    let ret = jieba.cut("南京市长江大桥", true);
    assert_eq!(vec!["南京市", "长江大桥"], ret);

    let ret = jieba.cut_all("南京市长江大桥");
    assert_eq!(vec!["南京", "南京市", "京市", "市长", "长江", "长江大桥", "大桥"], ret);

    let ret = jieba.cut_for_search("南京市长江大桥", true);
    assert_eq!(vec!["南京", "京市", "南京市", "长江", "大桥", "长江大桥"], ret);

    let ret = jieba.cut_hmm("南京长江大桥");
    assert_eq!(vec!["南京长", "江大桥"], ret);

    let ret = jieba.cut_small("南京长江大桥", 2);
    assert_eq!(vec!["南京", "长江", "大桥"], ret);

    let ret = jieba.tag("南京市长江大桥");
    assert_eq!(2, ret.len());
    assert_eq!("南京市", &ret[0].word);
    assert_eq!("ns", &ret[0].flag);
    assert_eq!("长江大桥", &ret[1].word);
    assert_eq!("ns", &ret[1].flag);

    let ret = jieba.lookup_tag("工作");
    assert_eq!("vn", &ret);
}
