//! [cppjieba](https://github.com/yanyiwu/cppjieba) Rust binding
//!
//! ## Installation
//!
//! Add it to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! rust-jieba = "0.1"
//! ```
//!
//! ## Example
//!
//! ```rust
//! extern crate rust_jieba;
//!
//! use rust_jieba::Jieba;
//!
//! fn main() {
//!     let jieba = Jieba::from_dir("cjieba-sys/cppjieba-cabi/cppjieba/dict");
//!     let words = jieba.cut("南京市长江大桥", true);
//!     assert_eq!(vec!["南京市", "长江大桥"], words);
//! }
//! ```
//!
extern crate cjieba_sys;

use std::slice;
use std::path::Path;
use std::ffi::{CString, CStr};

use cjieba_sys::*;

#[derive(Debug, Clone)]
pub struct Jieba {
    inner: *mut jieba_t,
}

/// `Jieba::tag` API return type
#[derive(Debug, Clone, PartialEq)]
pub struct Tag {
    /// Word
    pub word: String,
    /// Flag
    pub flag: String,
}

/// Word with weight
#[derive(Debug, Clone, PartialEq)]
pub struct WordWeight {
    /// Word
    pub word: String,
    /// Weight
    pub weight: f64,
}

/// Tokenize mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenizeMode {
    /// Default mode
    Default,
    /// Search mode
    Search,
}

/// Token
#[derive(Debug, Clone, PartialEq)]
pub struct Token(pub String, pub usize, pub usize);

impl Token {
    /// Word of the token
    pub fn word(&self) -> &str {
        &self.0
    }

    /// Unicode start position of the token
    pub fn start(&self) -> usize {
        self.1
    }

    /// Unicode end position of the token
    pub fn end(&self) -> usize {
        self.2
    }
}

impl Jieba {
    /// Create a new instance
    pub fn new(dict_path: &str, hmm_path: &str, user_dict_path: &str, idf_path: &str, stop_words_path: &str)
        -> Self
    {
        let c_dict_path = CString::new(dict_path).unwrap();
        let c_hmm_path = CString::new(hmm_path).unwrap();
        let c_user_dict_path = CString::new(user_dict_path).unwrap();
        let c_idf_path = CString::new(idf_path).unwrap();
        let c_stop_words_path = CString::new(stop_words_path).unwrap();
        unsafe {
            Self {
                inner: jieba_new(
                    c_dict_path.as_ptr(),
                    c_hmm_path.as_ptr(),
                    c_user_dict_path.as_ptr(),
                    c_idf_path.as_ptr(),
                    c_stop_words_path.as_ptr()
                )
            }
        }
    }

    /// Create a new instance from dict data  directory
    pub fn from_dir(data_dir: &str) -> Self {
        let data_path = Path::new(data_dir);
        let dict_path = data_path.join("jieba.dict.utf8");
        let hmm_path = data_path.join("hmm_model.utf8");
        let user_dict_path = data_path.join("user.dict.utf8");
        let idf_path = data_path.join("idf.utf8");
        let stop_words_path = data_path.join("stop_words.utf8");
        Self::new(
            dict_path.to_str().unwrap(),
            hmm_path.to_str().unwrap(),
            user_dict_path.to_str().unwrap(),
            idf_path.to_str().unwrap(),
            stop_words_path.to_str().unwrap(),
        )
    }

    /// Cut the input text
    ///
    /// ## Params
    ///
    /// `text`: input text
    ///
    /// `hmm`: enable HMM or not
    pub fn cut(&self, text: &str, hmm: bool) -> Vec<String> {
        let c_text = CString::new(text).unwrap();
        let is_hmm = if hmm { 1 } else { 0 };
        unsafe {
            let ret = jieba_cut(self.inner, c_text.as_ptr(), is_hmm);
            let c_words = slice::from_raw_parts((*ret).words, (*ret).length);
            let words = c_words.into_iter().map(|s| {
                let word = CStr::from_ptr(*s);
                word.to_string_lossy().into_owned()
            }).collect();
            jieba_words_free(ret);
            words
        }
    }

    /// Cut the input text, return all possible words
    ///
    /// ## Params
    ///
    /// `text`: input text
    pub fn cut_all(&self, text: &str) -> Vec<String> {
        let c_text = CString::new(text).unwrap();
        unsafe {
            let ret = jieba_cut_all(self.inner, c_text.as_ptr());
            let c_words = slice::from_raw_parts((*ret).words, (*ret).length);
            let words = c_words.into_iter().map(|s| {
                let word = CStr::from_ptr(*s);
                word.to_string_lossy().into_owned()
            }).collect();
            jieba_words_free(ret);
            words
        }
    }

    /// Cut the input text in search mode
    ///
    /// ## Params
    ///
    /// `text`: input text
    ///
    /// `hmm`: enable HMM or not
    pub fn cut_for_search(&self, text: &str, hmm: bool) -> Vec<String> {
        let c_text = CString::new(text).unwrap();
        let is_hmm = if hmm { 1 } else { 0 };
        unsafe {
            let ret = jieba_cut_for_search(self.inner, c_text.as_ptr(), is_hmm);
            let c_words = slice::from_raw_parts((*ret).words, (*ret).length);
            let words = c_words.into_iter().map(|s| {
                let word = CStr::from_ptr(*s);
                word.to_string_lossy().into_owned()
            }).collect();
            jieba_words_free(ret);
            words
        }
    }

    /// Cut the input text using HMM
    ///
    /// ## Params
    ///
    /// `text`: input text
    pub fn cut_hmm(&self, text: &str) -> Vec<String> {
        let c_text = CString::new(text).unwrap();
        unsafe {
            let ret = jieba_cut_hmm(self.inner, c_text.as_ptr());
            let c_words = slice::from_raw_parts((*ret).words, (*ret).length);
            let words = c_words.into_iter().map(|s| {
                let word = CStr::from_ptr(*s);
                word.to_string_lossy().into_owned()
            }).collect();
            jieba_words_free(ret);
            words
        }
    }

    /// Cut the input text but limit max word length
    ///
    /// ## Params
    ///
    /// `text`: input text
    ///
    /// `max_word_len`: max word length
    pub fn cut_small(&self, text: &str, max_word_len: usize) -> Vec<String> {
        let c_text = CString::new(text).unwrap();
        unsafe {
            let ret = jieba_cut_small(self.inner, c_text.as_ptr(), max_word_len);
            let c_words = slice::from_raw_parts((*ret).words, (*ret).length);
            let words = c_words.into_iter().map(|s| {
                let word = CStr::from_ptr(*s);
                word.to_string_lossy().into_owned()
            }).collect();
            jieba_words_free(ret);
            words
        }
    }

    /// Tag the input text
    ///
    /// ## Params
    ///
    /// `text`: input text
    pub fn tag(&self, text: &str) -> Vec<Tag> {
        let c_text = CString::new(text).unwrap();
        unsafe {
            let ret = jieba_tag(self.inner, c_text.as_ptr());
            let c_words = slice::from_raw_parts((*ret).words, (*ret).length);
            let tags = c_words.into_iter().map(|s| {
                let word = CStr::from_ptr(*s).to_string_lossy();
                let mut parts = word.splitn(2, '/');
                Tag {
                    word: parts.next().unwrap().to_string(),
                    flag: parts.next().unwrap().to_string(),
                }
            }).collect();
            jieba_words_free(ret);
            tags
        }
    }

    /// Look up an single word's tag
    pub fn lookup_tag(&self, word: &str) -> String {
        let c_word = CString::new(word).unwrap();
        unsafe {
            let ret = jieba_lookup_tag(self.inner, c_word.as_ptr());
            let tag = CStr::from_ptr(ret).to_string_lossy().into_owned();
            jieba_str_free(ret);
            tag
        }
    }

    /// Add user defined word
    pub fn add_user_word(&mut self, word: &str) {
        let c_word = CString::new(word).unwrap();
        unsafe {
            jieba_add_user_word(self.inner, c_word.as_ptr());
        }
    }

    /// Tokenize
    ///
    /// ## Params
    ///
    /// `text`: input text
    ///
    /// `mode`: tokenize mode
    ///
    /// `hmm`: enable HMM or not
    pub fn tokenize(&self, text: &str, mode: TokenizeMode, hmm: bool) -> Vec<Token> {
        let c_text = CString::new(text).unwrap();
        let c_mode = match mode {
            TokenizeMode::Default => JIEBA_TOKENIZE_MODE_DEFAULT,
            TokenizeMode::Search => JIEBA_TOKENIZE_MODE_SEARCH,
        };
        let is_hmm = if hmm { 1 } else { 0 };
        let mut tokens = Vec::new();
        unsafe {
            let ret = jieba_tokenize(self.inner, c_text.as_ptr(), c_mode, is_hmm);
            let mut index = 0;
            let mut c_token = ret.offset(index);
            while !c_token.is_null() && (*c_token).length > 0 {
                let start = (*c_token).offset as usize;
                let end = start + (*c_token).length as usize;
                let word = text[start..end].to_string();
                let unicode_start = (*c_token).unicode_offset as usize;
                let unicode_end = unicode_start + (*c_token).unicode_length as usize;
                tokens.push(Token(word, unicode_start, unicode_end));
                index += 1;
                c_token = ret.offset(index);
            }
            jieba_token_free(ret);
        }
        tokens
    }

    /// Extract keywords
    ///
    /// ## Params
    ///
    /// `text`: input text
    ///
    /// `top_k`: limit return keywords count
    pub fn extract(&self, text: &str, top_k: usize) -> Vec<String> {
        let c_text = CString::new(text).unwrap();
        unsafe {
            let ret = jieba_extract(self.inner, c_text.as_ptr(), top_k as i32);
            let c_words = slice::from_raw_parts((*ret).words, (*ret).length);
            let words = c_words.into_iter().map(|s| {
                let word = CStr::from_ptr(*s);
                word.to_string_lossy().into_owned()
            }).collect();
            jieba_words_free(ret);
            words
        }
    }

    /// Extract keywords with weight
    ///
    /// ## Params
    ///
    /// `text`: input text
    ///
    /// `top_k`: limit return keywords count
    pub fn extract_with_weight(&self, text: &str, top_k: usize) -> Vec<WordWeight> {
        let c_text = CString::new(text).unwrap();
        let mut words = Vec::new();
        unsafe {
            let ret = jieba_extract_with_weight(self.inner, c_text.as_ptr(), top_k as i32);
            let mut index = 0;
            let mut c_word = ret.offset(index);
            while !c_word.is_null() && !(*c_word).word.is_null() {
                let word = CStr::from_ptr((*c_word).word).to_string_lossy().into_owned();
                words.push(WordWeight {
                    word: word,
                    weight: (*c_word).weight
                });
                index += 1;
                c_word = ret.offset(index);
            }
            jieba_word_weight_free(ret);
        }
        words
    }
}

impl Drop for Jieba {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe { jieba_free(self.inner) };
        }
    }
}

unsafe impl Send for Jieba {}
unsafe impl Sync for Jieba {}
