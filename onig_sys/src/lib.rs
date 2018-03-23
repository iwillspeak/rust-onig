extern crate libc;

mod constants;
mod onigenc;

pub use self::constants::*;
pub use self::onigenc::*;

use libc::{c_int, c_uint, c_void, c_char, c_uchar};

pub type OnigCodePoint = c_uint;
pub type OnigUChar = c_uchar;
pub type OnigCtype = c_uint;
pub type OnigDistance = c_uint;
pub type OnigCaseFoldType = c_uint;
pub type OnigOptionType = c_uint;
pub type OnigSyntaxOp = c_uint;
pub type OnigSyntaxOp2 = c_uint;
pub type OnigSyntaxBehavior = c_uint;

pub type OnigEncoding = *const OnigEncodingType;
pub type OnigRegex = *const OnigRegexType;
pub type OnigRegexMut = *mut OnigRegexType;

/// Warning Callback
///
/// `void (*func)(char* warning_message)`
pub type OnigWarnFunc = extern "C" fn(*const c_char);

/// Apply All Case Fold Callback, see OnigEncodingType->apply_all_case_fold
pub type OnigApplyAllCaseFoldFunc = extern "C" fn(from: OnigCodePoint,
                                                  to: *const OnigCodePoint,
                                                  to_len: c_int,
                                                  arg: *const c_void)
                                                  -> c_int;


/// Foreach Callback
///
/// This callback will be invoked for each name when calling
/// [`onig_foreach_name`](fn.onig_foreach_name.html). The
/// final argument to that function is passed back to this callback.
pub type OnigForeachNameCallback = extern "C" fn(*const OnigUChar,
                                                 *const OnigUChar,
                                                 c_int,
                                                 *const c_int,
                                                 OnigRegex,
                                                 *mut c_void)
                                                 -> c_int;

/// Capture Tree Callback
///
/// ```c
/// int(*func)(int,int,int,int,int,void*)
/// ```
///
/// This callback will be invoked for each node in the capture tree
/// being traversed. See
/// [`onig_capture_tree_traverse`](fn.onig_capture_tree_traverse.html)
/// for more information about parameters and use.
pub type OnigCaptureTreeTraverseCallback = extern "C" fn(c_int,
                                                         c_int,
                                                         c_int,
                                                         c_int,
                                                         c_int,
                                                         *mut c_void)
                                                         -> c_int;

/// ```c
/// int (*scan_callback)(int, int, OnigRegion*, void*)
/// ```
pub type OnigScanCallback = extern "C" fn(c_int, c_int, *const OnigRegion, *mut c_void) -> c_int;

#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct OnigRegion {
    pub allocated: c_int,
    pub num_regs: c_int,
    pub beg: *const c_int,
    pub end: *const c_int,
    pub history_root: *const OnigCaptureTreeNode,
}

#[repr(C)]
#[derive(Debug)]
pub struct OnigCaptureTreeNode {
    pub group: c_int,
    pub beg: c_int,
    pub end: c_int,
    pub allocated: c_int,
    pub num_childs: c_int,
    pub childs: *const *const OnigCaptureTreeNode,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct OnigSyntaxType {
    pub op: c_uint,
    pub op2: c_uint,
    pub behavior: c_uint,
    pub options: c_uint,
    pub meta_char_table: OnigMetaCharTableType,
}

#[repr(C)]
#[derive(Debug)]
pub struct OnigCompileInfo {
    pub num_of_elements: c_int,
    pub pattern_enc: OnigEncoding,
    pub target_enc: OnigEncoding,
    pub syntax: *const OnigSyntaxType,
    pub option: OnigOptionType,
    pub case_fold_flag: OnigCaseFoldType,
}


#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct OnigMetaCharTableType {
    pub esc: OnigCodePoint,
    pub anychar: OnigCodePoint,
    pub anytime: OnigCodePoint,
    pub zero_or_one_time: OnigCodePoint,
    pub one_or_more_time: OnigCodePoint,
    pub anychar_anytime: OnigCodePoint,
}

#[repr(C)]
pub struct OnigCaseFoldCodeItem {
    pub byte_len: c_int, // argument(original) character(s) byte length
    pub code_len: c_int, // number of code
    pub code: [OnigCodePoint; ONIGENC_MAX_COMP_CASE_FOLD_CODE_LEN as usize],
}

#[repr(C)]
pub struct OnigEncodingType {
    pub mbc_enc_len: extern "C" fn(p: *const OnigUChar) -> c_int,
    pub name: *const c_char,
    pub max_enc_len: c_int,
    pub min_enc_len: c_int,
    pub is_mbc_newline: extern "C" fn(p: *const OnigUChar, end: *const OnigUChar) -> c_int,
    pub mbc_to_code: extern "C" fn(p: *const OnigUChar, end: *const OnigUChar) -> OnigCodePoint,
    pub code_to_mbclen: extern "C" fn(code: OnigCodePoint) -> c_int,
    pub code_to_mbc: extern "C" fn(code: OnigCodePoint, buf: *mut OnigUChar) -> c_int,
    pub mbc_case_fold: extern "C" fn(flag: OnigCaseFoldType,
                                     pp: *const *const OnigUChar,
                                     end: *const OnigUChar,
                                     to: *const OnigUChar)
                                     -> c_int,
    pub apply_all_case_fold: extern "C" fn(flag: OnigCaseFoldType,
                                           f: OnigApplyAllCaseFoldFunc,
                                           arg: *const c_void)
                                           -> c_int,
    pub get_case_fold_codes_by_str: extern "C" fn(flag: OnigCaseFoldType,
                                                  p: *const OnigUChar,
                                                  end: *const OnigUChar /* ... */)
                                                  -> c_int,
    pub property_name_to_ctype: extern "C" fn(enc: OnigEncoding,
                                              p: *const OnigUChar,
                                              end: *const OnigUChar)
                                              -> c_int,
    pub is_code_ctype: extern "C" fn(code: OnigCodePoint, ctype: OnigCtype) -> c_int,
    pub get_ctype_code_range: extern "C" fn(ctype: OnigCtype,
                                            sb_out: *const OnigCodePoint /* ... */)
                                            -> c_int,
    pub left_adjust_char_head: extern "C" fn(start: *const OnigUChar, p: *const OnigUChar)
                                             -> *const OnigUChar,
    pub is_allowed_reverse_match: extern "C" fn(p: *const OnigUChar, end: *const OnigUChar) -> c_int,
    pub init: extern "C" fn() -> c_int,
    pub is_initialised: extern "C" fn() -> c_int,
}

#[repr(C)]
#[derive(Debug)]
pub struct OnigErrorInfo {
    pub enc: OnigEncoding,
    pub par: *const OnigUChar,
    pub par_end: *const OnigUChar,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct OnigRepeatRange {
    pub lower: c_int,
    pub upper: c_int,
}

#[repr(C)]
pub struct OnigRegexType {
    // common members of BBuf(bytes-buffer)
    pub p: *const OnigUChar, // compiled pattern
    pub used: c_uint, // used space for p
    pub alloc: c_uint, // allocated space for p

    pub num_mem: c_int, // used memory(...) num counted from 1
    pub num_repeat: c_int, // OP_REPEAT/OP_REPEAT_NG id-counter
    pub num_null_check: c_int, // OP_NULL_CHECK_START/END id counter
    pub num_comb_exp_check: c_int, // combination explosion check
    pub num_call: c_int, // number of subexp call
    pub capture_history: c_uint, // (?@...) flag (1-31)
    pub bt_mem_start: c_uint, // need backtrack flag
    pub bt_mem_end: c_uint, // need backtrack flag
    pub stack_pop_level: c_int,
    pub repeat_range_alloc: c_int,
    pub repeat_range: *const OnigRepeatRange,

    pub enc: OnigEncoding,
    pub options: OnigOptionType,
    pub syntax: *const OnigSyntaxType,
    pub case_fold_flag: OnigCaseFoldType,
    pub name_table: *const c_void,

    // optimization info (string search, char-map and anchors)
    pub optimize: c_int, // optimize flag
    pub threshold_len: c_int, // search str-length for apply optimize
    pub anchor: c_int, // BEGIN_BUF, BEGIN_POS, (SEMI_)END_BUF
    pub anchor_dmin: OnigDistance, // (SEMI_)END_BUF anchor distance
    pub anchor_dmax: OnigDistance, // (SEMI_)END_BUF anchor distance
    pub sub_anchor: c_int, // start-anchor for exact or map
    pub exact: *const OnigUChar,
    pub exact_end: *const OnigUChar,
    pub map: [OnigUChar; ONIG_CHAR_TABLE_SIZE as usize], // used as BM skip or char-map
    pub int_map: *const c_int, // BM skip for exact_len > 255
    pub int_map_backward: *const c_int, // BM skip for backward search
    pub dmin: OnigDistance, // min-distance of exact or map
    pub dmax: OnigDistance, // max-distance of exact or map

    pub chain: *const OnigRegexType,
}

/// Match Parameters Struct
///
/// This can be passed to `onig_search_with_param` and
/// `onig_match_with_param` to control their behavior. Instances can
/// be created with `onig_new_match_param`. The contents of the struct
/// should not be modified directly.
#[repr(C)]
pub struct OnigMatchParam {
    /// External Data
    _external: c_int
}

extern "C" {
    pub static OnigEncodingASCII: OnigEncodingType;
    pub static OnigEncodingISO_8859_1: OnigEncodingType;
    pub static OnigEncodingISO_8859_2: OnigEncodingType;
    pub static OnigEncodingISO_8859_3: OnigEncodingType;
    pub static OnigEncodingISO_8859_4: OnigEncodingType;
    pub static OnigEncodingISO_8859_5: OnigEncodingType;
    pub static OnigEncodingISO_8859_6: OnigEncodingType;
    pub static OnigEncodingISO_8859_7: OnigEncodingType;
    pub static OnigEncodingISO_8859_8: OnigEncodingType;
    pub static OnigEncodingISO_8859_9: OnigEncodingType;
    pub static OnigEncodingISO_8859_10: OnigEncodingType;
    pub static OnigEncodingISO_8859_11: OnigEncodingType;
    pub static OnigEncodingISO_8859_13: OnigEncodingType;
    pub static OnigEncodingISO_8859_14: OnigEncodingType;
    pub static OnigEncodingISO_8859_15: OnigEncodingType;
    pub static OnigEncodingISO_8859_16: OnigEncodingType;
    pub static OnigEncodingUTF8: OnigEncodingType;
    pub static OnigEncodingUTF16_BE: OnigEncodingType;
    pub static OnigEncodingUTF16_LE: OnigEncodingType;
    pub static OnigEncodingUTF32_BE: OnigEncodingType;
    pub static OnigEncodingUTF32_LE: OnigEncodingType;
    pub static OnigEncodingEUC_JP: OnigEncodingType;
    pub static OnigEncodingEUC_TW: OnigEncodingType;
    pub static OnigEncodingEUC_KR: OnigEncodingType;
    pub static OnigEncodingEUC_CN: OnigEncodingType;
    pub static OnigEncodingSJIS: OnigEncodingType;
    pub static OnigEncodingKOI8: OnigEncodingType;
    pub static OnigEncodingKOI8_R: OnigEncodingType;
    pub static OnigEncodingCP1251: OnigEncodingType;
    pub static OnigEncodingBIG5: OnigEncodingType;
    pub static OnigEncodingGB18030: OnigEncodingType;

    pub static OnigSyntaxASIS: OnigSyntaxType;
    pub static OnigSyntaxPosixBasic: OnigSyntaxType;
    pub static OnigSyntaxPosixExtended: OnigSyntaxType;
    pub static OnigSyntaxEmacs: OnigSyntaxType;
    pub static OnigSyntaxGrep: OnigSyntaxType;
    pub static OnigSyntaxGnuRegex: OnigSyntaxType;
    pub static OnigSyntaxJava: OnigSyntaxType;
    pub static OnigSyntaxPerl: OnigSyntaxType;
    pub static OnigSyntaxPerl_NG: OnigSyntaxType;
    pub static OnigSyntaxRuby: OnigSyntaxType;
    pub static OnigSyntaxOniguruma: OnigSyntaxType;

    pub static mut OnigDefaultSyntax: *const OnigSyntaxType;

    // Oniguruma API  Version 5.9.2  2008/02/19

    /// Initialise Library
    ///
    /// This is the replacement for `onig_init`.
    pub fn onig_initialize(encodings: *const OnigEncoding, n: c_int) -> c_int;

    ///   Initialize library.
    ///
    ///  `int onig_init(void)`
    ///
    ///   You don't have to call it explicitly, because it is called in onig_new().
    pub fn onig_init() -> c_int;

    ///   Get error message string.
    ///   If this function is used for onig_new(),
    ///   don't call this after the pattern argument of onig_new() is freed.
    ///
    ///   `int onig_error_code_to_str(UChar* err_buf, int err_code, ...)`
    ///
    ///   normal return: error message string length
    ///
    /// # Arguments
    ///
    ///   1. err_buf:              error message string buffer.
    ///                           (required size: ONIG_MAX_ERROR_MESSAGE_LEN)
    ///   2. err_code:             error code returned by other API functions.
    ///   3. err_info (optional):  error info returned by onig_new().
    pub fn onig_error_code_to_str(err_buff: *mut OnigUChar, err_code: c_int, ...) -> c_int;

    ///   Set warning function.
    ///
    ///  `void onig_set_warn_func(OnigWarnFunc func)`
    ///
    ///   WARNING:
    ///     '[', '-', ']' in character class without escape.
    ///     ']' in pattern without escape.
    ///
    ///   arguments
    ///   1 func:     function pointer.    void (*func)(char* warning_message)
    pub fn onig_set_warn_func(func: OnigWarnFunc);

    ///   Set verbose warning function.
    ///
    ///  `void onig_set_verb_warn_func(OnigWarnFunc func)`
    ///
    ///   WARNING:
    ///     redundant nested repeat operator.
    ///
    ///   arguments
    ///   1 func:     function pointer.    void (*func)(char* warning_message)
    pub fn onig_set_verb_warn_func(func: OnigWarnFunc);

    /// Create a regex object.
    ///
    /// ```c
    /// int onig_new(regex_t** reg, const UChar* pattern, const UChar* pattern_end,
    ///             OnigOptionType option, OnigEncoding enc, OnigSyntaxType* syntax,
    ///             OnigErrorInfo* err_info)
    /// ```
    ///
    /// normal return: ONIG_NORMAL
    ///
    /// # Arguments
    ///
    /// 1. `reg`:         return regex object's address.
    /// 2. `pattern`:     regex pattern string.
    /// 3. `pattern_end`: terminate address of pattern. (pattern + pattern length)
    /// 4. `option`:      compile time options.
    ///  *  ONIG_OPTION_NONE               no option
    ///  *  ONIG_OPTION_SINGLELINE         `^` -> `\A`, `$` -> `\Z`
    ///  *  ONIG_OPTION_MULTILINE          `.` match with newline
    ///  *  ONIG_OPTION_IGNORECASE         ambiguity match on
    ///  *  ONIG_OPTION_EXTEND             extended pattern form
    ///  *  ONIG_OPTION_FIND_LONGEST       find longest match
    ///  *  ONIG_OPTION_FIND_NOT_EMPTY     ignore empty match
    ///  *  ONIG_OPTION_NEGATE_SINGLELINE
    ///       clear ONIG_OPTION_SINGLELINE which is enabled on
    ///       ONIG_SYNTAX_POSIX_BASIC, ONIG_SYNTAX_POSIX_EXTENDED,
    ///       ONIG_SYNTAX_PERL, ONIG_SYNTAX_PERL_NG, ONIG_SYNTAX_JAVA
    ///  *  ONIG_OPTION_DONT_CAPTURE_GROUP only named group captured.
    ///  *  ONIG_OPTION_CAPTURE_GROUP      named and no-named group captured.
    ///
    /// 5. `enc`:        character encoding.
    ///  * ONIG_ENCODING_ASCII         ASCII
    ///  * ONIG_ENCODING_ISO_8859_1    ISO 8859-1
    ///  * ONIG_ENCODING_ISO_8859_2    ISO 8859-2
    ///  * ONIG_ENCODING_ISO_8859_3    ISO 8859-3
    ///  * ONIG_ENCODING_ISO_8859_4    ISO 8859-4
    ///  * ONIG_ENCODING_ISO_8859_5    ISO 8859-5
    ///  * ONIG_ENCODING_ISO_8859_6    ISO 8859-6
    ///  * ONIG_ENCODING_ISO_8859_7    ISO 8859-7
    ///  * ONIG_ENCODING_ISO_8859_8    ISO 8859-8
    ///  * ONIG_ENCODING_ISO_8859_9    ISO 8859-9
    ///  * ONIG_ENCODING_ISO_8859_10   ISO 8859-10
    ///  * ONIG_ENCODING_ISO_8859_11   ISO 8859-11
    ///  * ONIG_ENCODING_ISO_8859_13   ISO 8859-13
    ///  * ONIG_ENCODING_ISO_8859_14   ISO 8859-14
    ///  * ONIG_ENCODING_ISO_8859_15   ISO 8859-15
    ///  * ONIG_ENCODING_ISO_8859_16   ISO 8859-16
    ///  * ONIG_ENCODING_UTF8          UTF-8
    ///  * ONIG_ENCODING_UTF16_BE      UTF-16BE
    ///  * ONIG_ENCODING_UTF16_LE      UTF-16LE
    ///  * ONIG_ENCODING_UTF32_BE      UTF-32BE
    ///  * ONIG_ENCODING_UTF32_LE      UTF-32LE
    ///  * ONIG_ENCODING_EUC_JP        EUC-JP
    ///  * ONIG_ENCODING_EUC_TW        EUC-TW
    ///  * ONIG_ENCODING_EUC_KR        EUC-KR
    ///  * ONIG_ENCODING_EUC_CN        EUC-CN
    ///  * ONIG_ENCODING_SJIS          Shift_JIS
    ///  * ONIG_ENCODING_KOI8_R        KOI8-R
    ///  * ONIG_ENCODING_CP1251        CP1251
    ///  * ONIG_ENCODING_BIG5          Big5
    ///  * ONIG_ENCODING_GB18030       GB18030
    ///  
    ///  or any OnigEncoding data address defined by user.
    ///
    /// 6. `syntax`:     address of pattern syntax definition.
    ///  * ONIG_SYNTAX_ASIS              plain text
    ///  * ONIG_SYNTAX_POSIX_BASIC       POSIX Basic RE
    ///  * ONIG_SYNTAX_POSIX_EXTENDED    POSIX Extended RE
    ///  * ONIG_SYNTAX_EMACS             Emacs
    ///  * ONIG_SYNTAX_GREP              grep
    ///  * ONIG_SYNTAX_GNU_REGEX         GNU regex
    ///  * ONIG_SYNTAX_JAVA              Java (Sun java.util.regex)
    ///  * ONIG_SYNTAX_PERL              Perl
    ///  * ONIG_SYNTAX_PERL_NG           Perl + named group
    ///  * ONIG_SYNTAX_RUBY              Ruby
    ///  * ONIG_SYNTAX_ONIGURUMA         Oniguruma syntax (Ruby based)
    ///  * ONIG_SYNTAX_DEFAULT           default (== Oniguruma)
    ///                                  onig_set_default_syntax()
    ///
    ///  or any OnigSyntaxType data address defined by user.
    ///
    /// 7. `err_info`: address for return optional error info.
    ///  Use this value as 3rd argument of onig_error_code_to_str().
    ///
    pub fn onig_new(reg: *mut OnigRegexMut,
                    pattern: *const OnigUChar,
                    pattern_end: *const OnigUChar,
                    option: OnigOptionType,
                    enc: OnigEncoding,
                    syntax: *const OnigSyntaxType,
                    err_info: *mut OnigErrorInfo)
                    -> c_int;

    /// Onig Reg Init
    ///
    /// ```c
    /// int  onig_reg_init(regex_t* reg,
    ///                    OnigOptionType option,
    ///                    OnigCaseFoldType case_fold_flag,
    ///                    OnigEncoding enc,
    ///                    OnigSyntaxType* syntax);
    /// ```
    pub fn onig_reg_init(reg: OnigRegexMut,
                         option: OnigOptionType,
                         case_fold_flag: OnigCaseFoldType,
                         enc: OnigEncoding,
                         syntax: *const OnigSyntaxType)
                         -> c_int;

    ///   Create a regex object.
    ///   reg object area is not allocated in this function.
    ///
    ///  `int onig_new_without_alloc(regex_t* reg, const UChar* pattern,
    ///             const UChar* pattern_end,
    ///             OnigOptionType option, OnigEncoding enc, OnigSyntaxType* syntax,
    ///             OnigErrorInfo* err_info)`
    ///
    ///   normal return: ONIG_NORMAL
    pub fn onig_new_without_alloc(reg: OnigRegexMut,
                                  pattern: *const OnigUChar,
                                  pattern_end: *const OnigUChar,
                                  option: OnigOptionType,
                                  enc: OnigEncoding,
                                  syntax: *const OnigSyntaxType,
                                  err_info: *mut OnigErrorInfo)
                                  -> c_int;

    ///   Create a regex object.
    ///   This function is deluxe version of onig_new().
    ///
    ///  `int onig_new_deluxe(regex_t** reg, const UChar* pattern, const UChar* pattern_end,
    ///                       OnigCompileInfo* ci, OnigErrorInfo* einfo)`
    ///
    ///   normal return: ONIG_NORMAL
    ///
    ///   arguments
    ///   1 reg:         return address of regex object.
    ///   2 pattern:     regex pattern string.
    ///   3 pattern_end: terminate address of pattern. (pattern + pattern length)
    ///   4 ci:          compile time info.
    ///
    ///     ci->num_of_elements: number of elements in ci. (current version: 5)
    ///     ci->pattern_enc:     pattern string character encoding.
    ///     ci->target_enc:      target string character encoding.
    ///     ci->syntax:          address of pattern syntax definition.
    ///     ci->option:          compile time option.
    ///     ci->case_fold_flag:  character matching case fold bit flag for
    ///                          ONIG_OPTION_IGNORECASE mode.
    ///
    ///     ONIGENC_CASE_FOLD_MIN:           minimum
    ///     ONIGENC_CASE_FOLD_DEFAULT:       minimum (onig_set_default_case_fold_flag())
    ///
    ///   5 err_info:    address for return optional error info.
    ///                  Use this value as 3rd argument of onig_error_code_to_str().
    ///
    ///
    ///   Different character encoding combination is allowed for
    ///   the following cases only.
    ///
    ///     pattern_enc: ASCII, ISO_8859_1
    ///     target_enc:  UTF16_BE, UTF16_LE, UTF32_BE, UTF32_LE
    ///
    ///     pattern_enc: UTF16_BE/LE
    ///     target_enc:  UTF16_LE/BE
    ///
    ///     pattern_enc: UTF32_BE/LE
    ///     target_enc:  UTF32_LE/BE
    pub fn onig_new_deluxe(reg: *mut OnigRegexMut,
                           pattern: *const OnigUChar,
                           pattern_end: *const OnigUChar,
                           ci: *const OnigCompileInfo,
                           einfo: *mut OnigErrorInfo)
                           -> c_int;

    ///   Free memory used by regex object.
    ///
    ///   `void onig_free(regex_t* reg)`
    ///
    /// # Arguments
    ///
    ///   1. `reg`: regex object.
    pub fn onig_free(reg: OnigRegexMut);

    ///   Free memory used by regex object. (Except reg oneself.)
    ///
    ///  `void onig_free_body(regex_t* reg)`
    ///
    ///   arguments
    ///   1 reg: regex object.
    pub fn onig_free_body(reg: OnigRegexMut);


    /// Allocate a OnigMatchParam object and initialize the contents by
    /// onig_initialize_match_param().
    pub fn onig_new_match_param() -> *mut OnigMatchParam;

    /// Free memory used by a OnigMatchParam object.
    ///
    /// # Arguments
    ///
    /// 1 mp: OnigMatchParam object
    pub fn onig_free_match_param(mp: *mut OnigMatchParam);

    /// Set match-param fields to default values.
    /// Match-param is used in onig_match_with_param() and onig_search_with_param().
    ///
    /// # Arguments
    ///
    /// 1 mp: match-param pointer
    pub fn onig_initialize_match_param(mp: *mut OnigMatchParam);

    /// Set a maximum number of match-stack depth.
    /// 0 means unlimited.
    ///
    /// # Arguments
    ///
    /// 1 mp: match-param pointer
    /// 2 limit: number of limit
    ///
    /// normal return: ONIG_NORMAL
    pub fn onig_set_match_stack_limit_size_of_match_param(mp: *mut OnigMatchParam, limit: c_uint) -> c_int;

    /// Set a retry limit count of a match process.
    ///
    /// # Arguments
    ///
    /// 1 mp: match-param pointer
    /// 2 limit: number of limit
    ///
    /// normal return: ONIG_NORMAL
    pub fn onig_set_retry_limit_in_match_of_match_param(mp: *mut OnigMatchParam, limit: c_uint) -> c_int;

    ///   Search string and return search result and matching region.
    ///
    ///   `int onig_search(regex_t* reg, const UChar* str, const UChar* end, const UChar* start,
    ///                    const UChar* range, OnigRegion* region, OnigOptionType option)`
    ///
    /// # Returns
    ///
    ///   normal return: match position offset (i.e.  p - str >= 0)
    ///   not found:     ONIG_MISMATCH (< 0)
    ///
    /// # Arguments
    ///
    ///   1. `reg`:    regex object
    ///   2. `str`:    target string
    ///   3. `end`:    terminate address of target string
    ///   4. `start`:  search start address of target string
    ///   5. `range`:  search terminate address of target string
    ///     in forward search  (start <= searched string < range)
    ///     in backward search (range <= searched string <= start)
    ///   6. `region`: address for return group match range info (NULL is allowed)
    ///   7. `option`: search time option
    ///
    ///    * ONIG_OPTION_NOTBOL        string head(str) isn't considered as begin of line
    ///    * ONIG_OPTION_NOTEOL        string end (end) isn't considered as end of line
    ///    * ONIG_OPTION_POSIX_REGION  region argument is regmatch_t[] of POSIX API.
    pub fn onig_search(reg: OnigRegex,
                       str: *const OnigUChar,
                       end: *const OnigUChar,
                       start: *const OnigUChar,
                       range: *const OnigUChar,
                       region: *mut OnigRegion,
                       option: OnigOptionType)
                       -> c_int;

    ///   Search string and return search result and matching region.
    ///
    ///   `int onig_search_with_param(regex_t* reg, const UChar* str, const UChar* end,
    ///                               const UChar* start, const UChar* range, OnigRegion* region,
    ///                               OnigOptionType option, OnigMatchParam* mp)`
    ///
    /// # Returns
    ///
    ///   normal return: match position offset (i.e.  p - str >= 0)
    ///   not found:     ONIG_MISMATCH (< 0)
    ///
    /// # Arguments
    ///
    ///   1-7:  same as onig_search()
    ///   8. `mp`: match parameters
    pub fn onig_search_with_param(reg: OnigRegex,
                                  str: *const OnigUChar,
                                  end: *const OnigUChar,
                                  start: *const OnigUChar,
                                  range: *const OnigUChar,
                                  region: *mut OnigRegion,
                                  option: OnigOptionType,
                                  mp: *const OnigMatchParam)
                                  -> c_int;

    ///   Match string and return result and matching region.
    ///
    ///   `int onig_match(regex_t* reg, const UChar* str, const UChar* end, const UChar* at,
    ///                   OnigRegion* region, OnigOptionType option)`
    ///
    /// # Returns
    ///
    ///   normal return: match length  (>= 0)
    ///   not match:     ONIG_MISMATCH ( < 0)
    ///
    /// # Arguments
    ///
    ///   1. `reg`:    regex object
    ///   2. `str`:    target string
    ///   3. `end`:    terminate address of target string
    ///   4. `at`:     match address of target string
    ///   5. `region`: address for return group match range info (NULL is allowed)
    ///   6. `option`: search time option
    ///
    ///    * ONIG_OPTION_NOTBOL       string head(str) isn't considered as begin of line
    ///    * ONIG_OPTION_NOTEOL       string end (end) isn't considered as end of line
    ///    * ONIG_OPTION_POSIX_REGION region argument is regmatch_t[] type of POSIX API.
    pub fn onig_match(reg: OnigRegex,
                      str: *const OnigUChar,
                      end: *const OnigUChar,
                      at: *const OnigUChar,
                      region: *mut OnigRegion,
                      option: OnigOptionType)
                      -> c_int;

    ///   Match string and return result and matching region.
    ///
    ///   `int onig_match_with_param(regex_t* reg, const UChar* str, const UChar* end,
    ///                              const UChar* at, OnigRegion* region,
    ///                              OnigOptionType option, OnigMatchParam* mp)`
    ///
    /// # Returns
    ///
    ///   normal return: match length  (>= 0)
    ///   not match:     ONIG_MISMATCH ( < 0)
    ///
    /// # Arguments
    ///
    ///   1-6:  same as onig_match()
    ///   7. `mp`: match parameters
    pub fn onig_match_with_param(reg: OnigRegex,
                      str: *const OnigUChar,
                      end: *const OnigUChar,
                      at: *const OnigUChar,
                      region: *mut OnigRegion,
                      option: OnigOptionType,
                      mp: *const OnigMatchParam)
                      -> c_int;

    ///   Scan string and callback with matching region.
    ///
    ///   normal return: number of matching times
    ///   error:         error code
    ///   interruption:  return value of callback function (!= 0)
    ///
    ///   arguments
    ///   1 reg:    regex object
    ///   2 str:    target string
    ///   3 end:    terminate address of target string
    ///   4 region: address for return group match range info (NULL is allowed)
    ///   5 option: search time option
    ///   6 scan_callback: callback function (defined by user)
    ///   7 callback_arg:  optional argument passed to callback
    pub fn onig_scan(reg: OnigRegex,
                     str: *const OnigUChar,
                     end: *const OnigUChar,
                     region: *mut OnigRegion,
                     options: OnigOptionType,
                     scan_callback: OnigScanCallback,
                     callback_arg: *mut c_void)
                     -> c_int;

    ///   Create a region.
    ///
    ///   `OnigRegion* onig_region_new(void)`
    pub fn onig_region_new() -> *mut OnigRegion;

    ///   Free memory used by region.
    ///
    ///  `void onig_region_free(OnigRegion* region, int free_self)`
    ///
    /// # Arguments
    ///
    ///   1. `region`:    target region
    ///   2. `free_self`: [1: free all, 0: free memory used in region
    ///   but not self]
    ///
    pub fn onig_region_free(region: *mut OnigRegion, free_self: c_int);

    ///   Copy contents of region.
    ///
    ///   `void onig_region_copy(OnigRegion* to, OnigRegion* from)`
    ///
    /// # Arguments
    ///
    ///   1. `to`:   target region
    ///   2. `from`: source region
    pub fn onig_region_copy(to: *mut OnigRegion, from: *const OnigRegion);

    ///   Clear contents of region.
    ///
    ///   `void onig_region_clear(OnigRegion* region)`
    ///
    /// # Arguments
    ///
    ///   1. `region`: target region
    pub fn onig_region_clear(region: *mut OnigRegion);

    ///   Resize group range area of region.
    ///
    ///   `int onig_region_resize(OnigRegion* region, int n)`
    ///
    /// # Returns
    ///
    ///   normal return: ONIG_NORMAL
    ///
    /// # Arguments
    ///
    ///   1. `region`: target region
    ///   2. `n`:      new size
    pub fn onig_region_resize(region: *mut OnigRegion, n: c_int) -> c_int;

    ///   Return the group number list of the name.
    ///   Named subexp is defined by (?<name>....).
    ///
    ///  `int onig_name_to_group_numbers(regex_t* reg, const UChar* name, const UChar* name_end,
    ///                                   int** num_list)`
    ///
    ///   normal return:  number of groups for the name.
    ///                   (ex. /(?<x>..)(?<x>..)/  ==>  2)
    ///   name not found: -1
    ///
    ///   arguments
    ///   1 reg:       regex object.
    ///   2 name:      group name.
    ///   3 name_end:  terminate address of group name.
    ///   4 num_list:  return list of group number.
    pub fn onig_name_to_group_numbers(reg: OnigRegex,
                                      name: *const OnigUChar,
                                      name_end: *const OnigUChar,
                                      num_list: *mut *const c_int)
                                      -> c_int;

    ///   Return the group number corresponding to the named backref (\k<name>).
    ///   If two or more regions for the groups of the name are effective,
    ///   the greatest number in it is obtained.
    ///
    ///  `int onig_name_to_backref_number(regex_t* reg, const UChar* name, const UChar* name_end,
    ///                                   OnigRegion *region)`
    ///
    ///   normal return: group number.
    ///
    ///   arguments
    ///   1 reg:      regex object.
    ///   2 name:     group name.
    ///   3 name_end: terminate address of group name.
    ///   4 region:   search/match result region.
    pub fn onig_name_to_backref_number(reg: OnigRegex,
                                       name: *const OnigUChar,
                                       name_end: *const OnigUChar,
                                       region: *const OnigRegion)
                                       -> c_int;

    ///   Iterate function call for all names.
    ///
    ///  `int onig_foreach_name(regex_t* reg,
    ///                         int (*func)(const UChar*, const UChar*, int,int*,regex_t*,void*),
    ///                         void* arg)`
    ///
    /// # Returns
    ///
    ///  * normal return: 0
    ///  * error:         func's return value.
    ///
    /// # Arguments
    ///
    ///   1. reg:     regex object.
    ///   2. func:    callback function.
    ///   
    ///   ```c
    ///   func(name, name_end, <number of groups>, <group number's list>,
    ///        reg, arg);
    ///   ```
    ///   
    ///   if func does not return 0, then iteration is stopped.
    ///
    ///   3. arg:     argument for func.
    pub fn onig_foreach_name(reg: OnigRegex,
                             func: OnigForeachNameCallback,
                             arg: *const c_void)
                             -> c_int;

    ///   Return the number of names defined in the pattern.
    ///   Multiple definitions of one name is counted as one.
    ///
    ///   `int onig_number_of_names(regex_t* reg)`
    ///
    /// # Arguments
    ///
    ///   1. `reg`:     regex object.
    pub fn onig_number_of_names(reg: OnigRegex) -> c_int;

    /// `OnigEncoding     onig_get_encoding(regex_t* reg)`
    pub fn onig_get_encoding(reg: OnigRegex) -> OnigEncoding;

    /// `OnigOptionType   onig_get_options(regex_t* reg)`
    pub fn onig_get_options(reg: OnigRegex) -> OnigOptionType;

    /// `OnigCaseFoldType onig_get_case_fold_flag(regex_t* reg)`
    pub fn onig_get_case_fold_flag(reg: OnigRegex) -> OnigCaseFoldType;

    /// `OnigSyntaxType*  onig_get_syntax(regex_t* reg)`
    pub fn onig_get_syntax(reg: OnigRegex) -> *const OnigSyntaxType;

    ///   Return the number of capture group in the pattern.
    ///
    ///   `int onig_number_of_captures(regex_t* reg)`
    ///
    /// # Arguments
    ///
    ///   1. `reg`:     regex object.
    pub fn onig_number_of_captures(reg: OnigRegex) -> c_int;

    ///   Return the number of capture history defined in the pattern.
    ///
    ///   `int onig_number_of_capture_histories(regex_t* reg)`
    ///
    ///   You can't use capture history if ONIG_SYN_OP2_ATMARK_CAPTURE_HISTORY
    ///   is disabled in the pattern syntax.(disabled in the default syntax)
    ///
    /// # arguments
    ///
    ///   1. `reg`:     regex object.
    pub fn onig_number_of_capture_histories(reg: OnigRegex) -> c_int;

    ///   Return the root node of capture history data tree.
    ///
    ///   `OnigCaptureTreeNode* onig_get_capture_tree(OnigRegion* region)`
    ///
    ///   This value is undefined if matching has faild.
    ///
    /// # Arguments
    ///
    ///   1. `region`: matching result.
    pub fn onig_get_capture_tree(region: *const OnigRegion) -> *const OnigCaptureTreeNode;

    ///  Traverse and callback in capture history data tree.
    ///
    /// `int onig_capture_tree_traverse(OnigRegion* region, int at,
    ///                   int(*func)(int,int,int,int,int,void*), void* arg)`
    ///
    /// # Returns
    ///
    ///   normal return: 0
    ///   error:         callback func's return value.
    ///
    /// # Arguments
    ///
    ///   1. region:  match region data.
    ///   2. at:      callback position.
    ///
    ///    * ONIG_TRAVERSE_CALLBACK_AT_FIRST: callback first, then traverse childs.
    ///    * ONIG_TRAVERSE_CALLBACK_AT_LAST:  traverse childs first, then callback.
    ///    * ONIG_TRAVERSE_CALLBACK_AT_BOTH:  callback first, then traverse childs,
    ///                                      and at last callback again.
    ///
    ///   3. func:  callback function.
    ///             if func does not return 0, then traverse is stopped.
    ///
    ///      ```c
    ///      int func(int group, int beg, int end, int level, int at,
    ///               void* arg)
    ///      ```
    ///
    ///      *  group: group number
    ///      *  beg:   capture start position
    ///      *  end:   capture end position
    ///      *  level: nest level (from 0)
    ///      *  at:    callback position
    ///          *     ONIG_TRAVERSE_CALLBACK_AT_FIRST
    ///          *     ONIG_TRAVERSE_CALLBACK_AT_LAST
    ///      *  arg:   optional callback argument
    ///
    ///   4. arg;     optional callback argument.
    pub fn onig_capture_tree_traverse(region: *const OnigRegion,
                                      at: c_int,
                                      func: OnigCaptureTreeTraverseCallback,
                                      arg: *mut c_void)
                                      -> c_int;


    ///   Return noname group capture activity.
    ///
    ///  `int onig_noname_group_capture_is_active(regex_t* reg)`
    ///
    ///   active:   1
    ///   inactive: 0
    ///
    /// # Arguments
    ///
    ///   1. reg:     regex object.
    ///
    ///  > if option ONIG_OPTION_DONT_CAPTURE_GROUP == ON
    ///  >   --> inactive
    ///  >
    ///  > if the regex pattern have named group
    ///  >    and syntax ONIG_SYN_CAPTURE_ONLY_NAMED_GROUP == ON
    ///  >    and option ONIG_OPTION_CAPTURE_GROUP == OFF
    ///  >   --> inactive
    ///  >
    ///  > else --> active
    pub fn onig_noname_group_capture_is_active(reg: OnigRegex) -> c_int;

    ///   Set default syntax.
    ///
    ///  `int onig_set_default_syntax(OnigSyntaxType* syntax)`
    ///
    ///   arguments
    ///   1 syntax: address of pattern syntax definition.
    pub fn onig_set_default_syntax(syntax: *const OnigSyntaxType) -> c_int;

    ///   Copy syntax.
    ///
    ///   `void onig_copy_syntax(OnigSyntaxType* to, OnigSyntaxType* from)`
    ///
    /// # Arguments
    ///
    ///   1. `to`:   destination address.
    ///   2. `from`: source address.
    pub fn onig_copy_syntax(to: *mut OnigSyntaxType, from: *const OnigSyntaxType);

    /// `unsigned int onig_get_syntax_op(OnigSyntaxType* syntax)`
    pub fn onig_get_syntax_op(syntax: *const OnigSyntaxType) -> OnigSyntaxOp;

    /// `unsigned int onig_get_syntax_op2(OnigSyntaxType* syntax)`
    pub fn onig_get_syntax_op2(syntax: *const OnigSyntaxType) -> OnigSyntaxOp2;

    /// `unsigned int onig_get_syntax_behavior(OnigSyntaxType* syntax)`
    pub fn onig_get_syntax_behavior(syntax: *const OnigSyntaxType) -> OnigSyntaxBehavior;

    /// `OnigOptionType onig_get_syntax_options(OnigSyntaxType* syntax)`
    pub fn onig_get_syntax_options(syntax: *const OnigSyntaxType) -> OnigOptionType;

    /// `void onig_set_syntax_op(OnigSyntaxType* syntax, unsigned int op)`
    pub fn onig_set_syntax_op(syntax: *mut OnigSyntaxType, op: OnigSyntaxOp);

    /// `void onig_set_syntax_op2(OnigSyntaxType* syntax, unsigned int op2)`
    pub fn onig_set_syntax_op2(syntax: *mut OnigSyntaxType, op2: OnigSyntaxOp2);

    /// `void onig_set_syntax_behavior(OnigSyntaxType* syntax, unsigned int behavior)`
    pub fn onig_set_syntax_behavior(syntax: *mut OnigSyntaxType, behavior: OnigSyntaxBehavior);

    /// `void onig_set_syntax_options(OnigSyntaxType* syntax, OnigOptionType options)`
    pub fn onig_set_syntax_options(syntax: *mut OnigSyntaxType, options: OnigOptionType);

    ///   Copy encoding.
    ///
    ///   `void onig_copy_encoding(OnigEncoding to, OnigOnigEncoding from)`
    ///
    ///   arguments
    ///   1 to:   destination address.
    ///   2 from: source address.
    pub fn onig_copy_encoding(to: *mut OnigEncodingType, from: OnigEncoding);

    /// Set a variable meta character to the code point value.
    ///
    /// Except for an escape character, this meta characters
    /// specification only works, if
    /// ONIG_SYN_OP_VARIABLE_META_CHARACTERS is enabled by the
    /// syntax. (Build-in syntaxes are not affected.)
    ///
    /// ```c
    /// int onig_set_meta_char(OnigSyntaxType* syntax, unsigned int what,
    ///                        OnigCodePoint code)
    /// ```
    ///
    /// normal return: ONIG_NORMAL
    ///
    /// # Arguments
    ///
    /// 1. syntax: target syntax
    /// 2. what:   specifies which meta character it is.
    ///
    /// ```c
    /// ONIG_META_CHAR_ESCAPE
    /// ONIG_META_CHAR_ANYCHAR
    /// ONIG_META_CHAR_ANYTIME
    /// ONIG_META_CHAR_ZERO_OR_ONE_TIME
    /// ONIG_META_CHAR_ONE_OR_MORE_TIME
    /// ONIG_META_CHAR_ANYCHAR_ANYTIME
    /// ```
    ///
    /// 3. code: meta character or `ONIG_INEFFECTIVE_META_CHAR`.
    pub fn onig_set_meta_char(syntax: *mut OnigSyntaxType,
                              what: c_uint,
                              code: OnigCodePoint)
                              -> c_int;

    ///   Get default case fold flag.
    ///
    ///   `OnigCaseFoldType onig_get_default_case_fold_flag()`
    pub fn onig_get_default_case_fold_flag() -> OnigCaseFoldType;

    ///   Set default case fold flag.
    ///
    ///   `int onig_set_default_case_fold_flag(OnigCaseFoldType case_fold_flag)`
    ///
    ///   1 case_fold_flag: case fold flag
    pub fn onig_set_default_case_fold_flag(case_fold_flag: OnigCaseFoldType) -> c_int;

    ///   Return the maximum number of stack size.
    ///   (default: 0 == unlimited)
    ///
    ///   `unsigned int onig_get_match_stack_limit_size(void)`
    pub fn onig_get_match_stack_limit_size() -> c_uint;

    ///   Set the maximum number of stack size.
    ///
    ///   `int onig_set_match_stack_limit_size(unsigned int size)`
    ///
    ///   (size = 0: unlimited)
    ///   normal return: ONIG_NORMAL
    pub fn onig_set_match_stack_limit_size(size: c_uint) -> c_int;

    /// Define User Unicode Property
    ///
    /// ```c
    /// int onig_unicode_define_user_property(const char* name,
    ///                                       OnigCodePoint* ranges);
    /// ```
    pub fn onig_unicode_define_user_property(name: *const c_char,
                                             ranges: *const OnigCodePoint)
                                             -> c_int;

    /// Se the maximum number of captures
    ///
    /// Default capture number limit (32767) is not changed. Call
    /// onig_set_capture_num_limit(0) to be unlimited.
    ///
    pub fn onig_set_capture_num_limit(limit: c_int) -> c_int;

    ///   The use of this library is finished.
    ///
    ///   `int onig_end(void)`
    ///
    ///   normal return: ONIG_NORMAL
    ///
    ///   It is not allowed to use regex objects which created
    ///   before onig_end() call.
    pub fn onig_end() -> c_int;

    ///   Return version string.  (ex. "5.0.3")
    ///
    ///   `const char* onig_version(void)`
    pub fn onig_version() -> *const c_char;

    /// Return Copyright String
    ///
    /// ```c
    /// const char* onig_copyright(void);
    /// ```
    pub fn onig_copyright() -> *const c_char;

}
