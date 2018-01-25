extern crate rust_jieba;

use rust_jieba::{Jieba, TokenizeMode, Token};

fn jieba() -> Jieba {
    Jieba::from_dir("cjieba-sys/cppjieba-cabi/cppjieba/dict")
}

#[test]
fn test_jieba() {
    let mut jieba = jieba();
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

    jieba.add_user_word("WTF");

    let tokens = jieba.tokenize("南京市长江大桥", TokenizeMode::Default, true);
    assert_eq!(2, tokens.len());
    assert_eq!(Token("南京市".to_string(), 0, 3), tokens[0]);
    assert_eq!(Token("长江大桥".to_string(), 3, 7), tokens[1]);

    let tokens = jieba.tokenize("南京市长江大桥", TokenizeMode::Search, true);
    assert_eq!(6, tokens.len());
    assert_eq!(Token("南京".to_string(), 0, 2), tokens[0]);
    assert_eq!(Token("京市".to_string(), 1, 3), tokens[1]);
    assert_eq!(Token("南京市".to_string(), 0, 3), tokens[2]);
    assert_eq!(Token("长江".to_string(), 3, 5), tokens[3]);
    assert_eq!(Token("大桥".to_string(), 5, 7), tokens[4]);
    assert_eq!(Token("长江大桥".to_string(), 3, 7), tokens[5]);

    let ret = jieba.extract("南京市长江大桥", 20);
    assert_eq!(vec!["长江大桥", "南京市"], ret);

    let ret = jieba.extract("南京市长江大桥", 1);
    assert_eq!(vec!["长江大桥"], ret);
}
