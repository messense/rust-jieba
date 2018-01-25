
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jieba_t {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jieba_token_t {
    pub offset: u32,
    pub length: usize,
    pub unicode_offset: u32,
    pub unicode_length: u32,
}

pub type jieba_tokenize_mode_t = u32;

pub const JIEBA_TOKENIZE_MODE_DEFAULT: jieba_tokenize_mode_t = 0;
pub const JIEBA_TOKENIZE_MODE_SEARCH: jieba_tokenize_mode_t = 1;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jieba_word_weight_t {
    pub word: *mut ::std::os::raw::c_char,
    pub weight: f64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jieba_words_t {
    pub words: *mut *mut ::std::os::raw::c_char,
    pub length: usize,
}

extern "C" {
    pub fn jieba_new(
        dict_path: *const ::std::os::raw::c_char,
        hmm_path: *const ::std::os::raw::c_char,
        user_dict: *const ::std::os::raw::c_char,
        idf_path: *const ::std::os::raw::c_char,
        stop_word_path: *const ::std::os::raw::c_char,
    ) -> *mut jieba_t;
    pub fn jieba_free(arg1: *mut jieba_t);
    pub fn jieba_words_free(words: *mut jieba_words_t);
    pub fn jieba_str_free(str: *mut ::std::os::raw::c_char);
    pub fn jieba_cut(
        handle: *mut jieba_t,
        sentence: *const ::std::os::raw::c_char,
        is_hmm_used: ::std::os::raw::c_int,
    ) -> *mut jieba_words_t;
    pub fn jieba_cut_all(
        handle: *mut jieba_t,
        sentence: *const ::std::os::raw::c_char,
    ) -> *mut jieba_words_t;
    pub fn jieba_cut_hmm(
        handle: *mut jieba_t,
        sentence: *const ::std::os::raw::c_char,
    ) -> *mut jieba_words_t;
    pub fn jieba_cut_for_search(
        handle: *mut jieba_t,
        sentence: *const ::std::os::raw::c_char,
        is_hmm_used: ::std::os::raw::c_int,
    ) -> *mut jieba_words_t;
    pub fn jieba_cut_small(
        handle: *mut jieba_t,
        sentence: *const ::std::os::raw::c_char,
        max_word_len: usize,
    ) -> *mut jieba_words_t;
    pub fn jieba_tag(
        handle: *mut jieba_t,
        sentence: *const ::std::os::raw::c_char,
    ) -> *mut jieba_words_t;
    pub fn jieba_lookup_tag(
        handle: *mut jieba_t,
        str: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn jieba_add_user_word(handle: *mut jieba_t, word: *const ::std::os::raw::c_char);
    pub fn jieba_add_user_words(
        handle: *mut jieba_t,
        words: *mut *const ::std::os::raw::c_char,
        count: usize,
    );
    pub fn jieba_reset_separators(handle: *mut jieba_t, sep: *const ::std::os::raw::c_char);
    pub fn jieba_tokenize(
        handle: *mut jieba_t,
        sentence: *const ::std::os::raw::c_char,
        mode: jieba_tokenize_mode_t,
        is_hmm_used: ::std::os::raw::c_int,
    ) -> *mut jieba_token_t;
    pub fn jieba_token_free(tokens: *mut jieba_token_t);
    pub fn jieba_extract(
        handle: *mut jieba_t,
        sentence: *const ::std::os::raw::c_char,
        top_k: ::std::os::raw::c_int,
    ) -> *mut jieba_words_t;
    pub fn jieba_extract_with_weight(
        handle: *mut jieba_t,
        sentence: *const ::std::os::raw::c_char,
        top_k: ::std::os::raw::c_int,
    ) -> *mut jieba_word_weight_t;
    pub fn jieba_word_weight_free(wws: *mut jieba_word_weight_t);
}
