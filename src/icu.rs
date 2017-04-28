//! Relevant libicucore bindings

#![allow(non_snake_case,non_camel_case_types)]
#![allow(dead_code)]

extern crate libc;

pub use self::UCharNameChoice::*;
pub use self::UErrorCode::*;

#[repr(C)]
pub enum UCharNameChoice {
    U_UNICODE_CHAR_NAME  = 0,
    U_EXTENDED_CHAR_NAME = 2,
    U_CHAR_NAME_ALIAS    = 3
}

#[repr(C)]
#[derive(PartialEq,Eq,Clone,Copy,PartialOrd,Ord,Debug)]
pub enum UErrorCode {
   U_USING_FALLBACK_WARNING  = -128,
   //U_ERROR_WARNING_START     = -128,
   U_USING_DEFAULT_WARNING   = -127,
   U_SAFECLONE_ALLOCATED_WARNING = -126,
   U_STATE_OLD_WARNING       = -125,
   U_STRING_NOT_TERMINATED_WARNING = -124,
   U_SORT_KEY_TOO_SHORT_WARNING = -123,
   U_AMBIGUOUS_ALIAS_WARNING = -122,
   U_DIFFERENT_UCA_VERSION = -121,
   U_PLUGIN_CHANGED_LEVEL_WARNING = -120,
   U_ERROR_WARNING_LIMIT,
   U_ZERO_ERROR              =  0,
   U_ILLEGAL_ARGUMENT_ERROR  =  1,
   U_MISSING_RESOURCE_ERROR  =  2,
   U_INVALID_FORMAT_ERROR    =  3,
   U_FILE_ACCESS_ERROR       =  4,
   U_INTERNAL_PROGRAM_ERROR  =  5,
   U_MESSAGE_PARSE_ERROR     =  6,
   U_MEMORY_ALLOCATION_ERROR =  7,
   U_INDEX_OUTOFBOUNDS_ERROR =  8,
   U_PARSE_ERROR             =  9,
   U_INVALID_CHAR_FOUND      = 10,
   U_TRUNCATED_CHAR_FOUND    = 11,
   U_ILLEGAL_CHAR_FOUND      = 12,
   U_INVALID_TABLE_FORMAT    = 13,
   U_INVALID_TABLE_FILE      = 14,
   U_BUFFER_OVERFLOW_ERROR   = 15,
   U_UNSUPPORTED_ERROR       = 16,
   U_RESOURCE_TYPE_MISMATCH  = 17,
   U_ILLEGAL_ESCAPE_SEQUENCE = 18,
   U_UNSUPPORTED_ESCAPE_SEQUENCE = 19,
   U_NO_SPACE_AVAILABLE      = 20,
   U_CE_NOT_FOUND_ERROR      = 21,
   U_PRIMARY_TOO_LONG_ERROR  = 22,
   U_STATE_TOO_OLD_ERROR     = 23,
   U_TOO_MANY_ALIASES_ERROR  = 24,
   U_ENUM_OUT_OF_SYNC_ERROR  = 25,
   U_INVARIANT_CONVERSION_ERROR = 26,
   U_INVALID_STATE_ERROR     = 27,
   U_COLLATOR_VERSION_MISMATCH = 28,
   U_USELESS_COLLATOR_ERROR  = 29,
   U_NO_WRITE_PERMISSION     = 30,
   U_STANDARD_ERROR_LIMIT,
   /*
    * the error code range 0x10000 0x10100 are reserved for Transliterator
    */
   U_BAD_VARIABLE_DEFINITION = 0x10000,
   //U_PARSE_ERROR_START = 0x10000,
   U_MALFORMED_RULE,
   U_MALFORMED_SET,
   U_MALFORMED_SYMBOL_REFERENCE,
   U_MALFORMED_UNICODE_ESCAPE,
   U_MALFORMED_VARIABLE_DEFINITION,
   U_MALFORMED_VARIABLE_REFERENCE,
   U_MISMATCHED_SEGMENT_DELIMITERS,
   U_MISPLACED_ANCHOR_START,
   U_MISPLACED_CURSOR_OFFSET,
   U_MISPLACED_QUANTIFIER,
   U_MISSING_OPERATOR,
   U_MISSING_SEGMENT_CLOSE,
   U_MULTIPLE_ANTE_CONTEXTS,
   U_MULTIPLE_CURSORS,
   U_MULTIPLE_POST_CONTEXTS,
   U_TRAILING_BACKSLASH,
   U_UNDEFINED_SEGMENT_REFERENCE,
   U_UNDEFINED_VARIABLE,
   U_UNQUOTED_SPECIAL,
   U_UNTERMINATED_QUOTE,
   U_RULE_MASK_ERROR,
   U_MISPLACED_COMPOUND_FILTER,
   U_MULTIPLE_COMPOUND_FILTERS,
   U_INVALID_RBT_SYNTAX,
   U_INVALID_PROPERTY_PATTERN,
   U_MALFORMED_PRAGMA,
   U_UNCLOSED_SEGMENT,
   U_ILLEGAL_CHAR_IN_SEGMENT,
   U_VARIABLE_RANGE_EXHAUSTED,
   U_VARIABLE_RANGE_OVERLAP,
   U_ILLEGAL_CHARACTER,
   U_INTERNAL_TRANSLITERATOR_ERROR,
   U_INVALID_ID,
   U_INVALID_FUNCTION,
   U_PARSE_ERROR_LIMIT,
   /*
    * the error code range 0x10100 0x10200 are reserved for formatting API parsing error
    */
   U_UNEXPECTED_TOKEN=0x10100,
   //U_FMT_PARSE_ERROR_START=0x10100,
   U_MULTIPLE_DECIMAL_SEPARATORS,
   //U_MULTIPLE_DECIMAL_SEPERATORS = U_MULTIPLE_DECIMAL_SEPARATORS,
   U_MULTIPLE_EXPONENTIAL_SYMBOLS,
   U_MALFORMED_EXPONENTIAL_PATTERN,
   U_MULTIPLE_PERCENT_SYMBOLS,
   U_MULTIPLE_PERMILL_SYMBOLS,
   U_MULTIPLE_PAD_SPECIFIERS,
   U_PATTERN_SYNTAX_ERROR,
   U_ILLEGAL_PAD_POSITION,
   U_UNMATCHED_BRACES,
   U_UNSUPPORTED_PROPERTY,
   U_UNSUPPORTED_ATTRIBUTE,
   U_ARGUMENT_TYPE_MISMATCH,
   U_DUPLICATE_KEYWORD,
   U_UNDEFINED_KEYWORD,
   U_DEFAULT_KEYWORD_MISSING,
   U_DECIMAL_NUMBER_SYNTAX_ERROR,
   U_FORMAT_INEXACT_ERROR,
   U_FMT_PARSE_ERROR_LIMIT,
   /*
    * the error code range 0x10200 0x102ff are reserved for Break Iterator related error
    */
   U_BRK_INTERNAL_ERROR=0x10200,
   //U_BRK_ERROR_START=0x10200,
   U_BRK_HEX_DIGITS_EXPECTED,
   U_BRK_SEMICOLON_EXPECTED,
   U_BRK_RULE_SYNTAX,
   U_BRK_UNCLOSED_SET,
   U_BRK_ASSIGN_ERROR,
   U_BRK_VARIABLE_REDFINITION,
   U_BRK_MISMATCHED_PAREN,
   U_BRK_NEW_LINE_IN_QUOTED_STRING,
   U_BRK_UNDEFINED_VARIABLE,
   U_BRK_INIT_ERROR,
   U_BRK_RULE_EMPTY_SET,
   U_BRK_UNRECOGNIZED_OPTION,
   U_BRK_MALFORMED_RULE_TAG,
   U_BRK_ERROR_LIMIT,
   /*
    * The error codes in the range 0x10300-0x103ff are reserved for regular expression related errrs
    */
   U_REGEX_INTERNAL_ERROR=0x10300,
   //U_REGEX_ERROR_START=0x10300,
   U_REGEX_RULE_SYNTAX,
   U_REGEX_INVALID_STATE,
   U_REGEX_BAD_ESCAPE_SEQUENCE,
   U_REGEX_PROPERTY_SYNTAX,
   U_REGEX_UNIMPLEMENTED,
   U_REGEX_MISMATCHED_PAREN,
   U_REGEX_NUMBER_TOO_BIG,
   U_REGEX_BAD_INTERVAL,
   U_REGEX_MAX_LT_MIN,
   U_REGEX_INVALID_BACK_REF,
   U_REGEX_INVALID_FLAG,
   U_REGEX_LOOK_BEHIND_LIMIT,
   U_REGEX_SET_CONTAINS_STRING,
   U_REGEX_OCTAL_TOO_BIG,
   U_REGEX_MISSING_CLOSE_BRACKET,
   U_REGEX_INVALID_RANGE,
   U_REGEX_STACK_OVERFLOW,
   U_REGEX_TIME_OUT,
   U_REGEX_STOPPED_BY_CALLER,
   U_REGEX_ERROR_LIMIT,
   /*
    * The error code in the range 0x10400-0x104ff are reserved for IDNA related error codes
    */
   U_IDNA_PROHIBITED_ERROR = 0x10400,
   //U_IDNA_ERROR_START = 0x10400,
   U_IDNA_UNASSIGNED_ERROR,
   U_IDNA_CHECK_BIDI_ERROR,
   U_IDNA_STD3_ASCII_RULES_ERROR,
   U_IDNA_ACE_PREFIX_ERROR,
   U_IDNA_VERIFICATION_ERROR,
   U_IDNA_LABEL_TOO_LONG_ERROR,
   U_IDNA_ZERO_LENGTH_LABEL_ERROR,
   U_IDNA_DOMAIN_NAME_TOO_LONG_ERROR,
   U_IDNA_ERROR_LIMIT,
   /*
    * Aliases for StringPrep
    */
   //U_STRINGPREP_PROHIBITED_ERROR = U_IDNA_PROHIBITED_ERROR,
   //U_STRINGPREP_UNASSIGNED_ERROR = U_IDNA_UNASSIGNED_ERROR,
   //U_STRINGPREP_CHECK_BIDI_ERROR = U_IDNA_CHECK_BIDI_ERROR,

   /*
    * The error code in the range 0x10500-0x105ff are reserved for Plugin related error codes
    */
   //U_PLUGIN_ERROR_START = 0x10500,
   U_PLUGIN_TOO_HIGH = 0x10500,
   U_PLUGIN_DIDNT_SET_LEVEL,
   U_PLUGIN_ERROR_LIMIT,
   //U_ERROR_LIMIT = U_PLUGIN_ERROR_LIMIT
}

#[inline]
pub fn U_SUCCESS(err: UErrorCode) -> bool {
    err as isize <= U_ZERO_ERROR as isize
}

#[inline]
pub fn U_FAILURE(err: UErrorCode) -> bool {
    err as isize > U_ZERO_ERROR as isize
}

/// Converts the indicated codepoint into a string, or returns an error.
/// If the codepoint doesn't exist, Ok("") will be returned.
///
/// The codepoint must be 0 <= code <= 0x10FFFF. If an out-of-range codepoint is passed,
/// Err(U_INVALID_CHAR_FOUND) will be returned. This is non-standard; libicucore's
/// u_charName appears to return "" instead.
pub fn u_charName(code: u32, nameChoice: UCharNameChoice) -> Result<String, UErrorCode> {
    if code > 0x10FFFF {
        return Err(U_INVALID_CHAR_FOUND);
    }

    const BUFFER_SIZE: usize = 128;
    let mut buffer = [0u8; BUFFER_SIZE];

    let mut err = U_ZERO_ERROR;
    let len = unsafe {
        raw::u_charName(code as raw::UChar32, nameChoice, buffer.as_mut_ptr() as *mut libc::c_char,
                        BUFFER_SIZE as libc::int32_t, &mut err)
    };

    if U_SUCCESS(err) {
        assert!(len >= 0);
        let name = &buffer[..len as usize];
        Ok(String::from_utf8_lossy(name).into_owned())
    } else {
        assert!(err != U_BUFFER_OVERFLOW_ERROR, "u_charName buffer is too small");
        Err(err)
    }
}

mod raw {
    extern crate libc;

    use super::{UCharNameChoice, UErrorCode};

    pub type UChar32 = libc::int32_t;

    #[link(name="icucore")]
    extern {
        pub fn u_charName(code: UChar32, nameChoice: UCharNameChoice,
                          buffer: *mut libc::c_char, bufferLength: libc::int32_t,
                          pErrorCode: *mut UErrorCode) -> libc::int32_t;
    }
}
