extern crate cjieba_sys;

use std::slice;
use std::path::Path;
use std::ffi::{CString, CStr};
use std::borrow::Cow;

use cjieba_sys::*;

#[derive(Debug, Clone)]
pub struct Jieba {
    inner: *mut jieba_t,
}

#[derive(Debug, Clone)]
pub struct Tag {
    word: String,
    flag: String,
}

#[derive(Debug, Clone)]
pub struct WordWeight {
    word: String,
    weight: f64,
}

impl Jieba {
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

    pub fn lookup_tag(&self, word: &str) -> Cow<str> {
        let c_word = CString::new(word).unwrap();
        unsafe {
            let ret = jieba_lookup_tag(self.inner, c_word.as_ptr());
            CStr::from_ptr(ret).to_string_lossy()
        }
    }

    pub fn add_user_word(&mut self, word: &str) {
        let c_word = CString::new(word).unwrap();
        unsafe {
            jieba_add_user_word(self.inner, c_word.as_ptr());
        }
    }

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
