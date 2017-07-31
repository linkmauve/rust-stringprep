//! Character Tables
use unicode_bidi::{bidi_class, BidiClass};
use std::cmp::Ordering;

use super::rfc3454;

/// A.1 Unassigned code points in Unicode 3.2
pub fn unassigned_code_point(c: char) -> bool {
    match table_lookup(rfc3454::A_1, c) {
        Some(_) => true,
        None => false,
    }
}

/// B.1 Commonly mapped to nothing
pub fn commonly_mapped_to_nothing(c: char) -> bool {
    match c {
        '\u{00AD}' | '\u{034F}' | '\u{1806}' | '\u{180B}' | '\u{180C}' | '\u{180D}' |
        '\u{200B}' | '\u{200C}' | '\u{200D}' | '\u{2060}' | '\u{FE00}' | '\u{FE01}' |
        '\u{FE02}' | '\u{FE03}' | '\u{FE04}' | '\u{FE05}' | '\u{FE06}' | '\u{FE07}' |
        '\u{FE08}' | '\u{FE09}' | '\u{FE0A}' | '\u{FE0B}' | '\u{FE0C}' | '\u{FE0D}' |
        '\u{FE0E}' | '\u{FE0F}' | '\u{FEFF}' => true,
        _ => false,
    }
}

/// B.2 Mapping for case-folding used with NFKC.
pub fn case_fold(s: &str) -> String {
    let mut result = String::new();

    // Each character either maps to a sequence of replacement characters,
    // or is passed through as-is.
    for c in s.chars() {
        if let Some(entry) = table_lookup(rfc3454::B_2, c) {
            let (_, _, replace) = entry;
            if let Some(replace) = replace {
                result.push_str(replace);
                continue;
            }
        }
        result.push(c);
    }

    result
}

/// C.1.1 ASCII space characters
pub fn ascii_space_character(c: char) -> bool {
    c == ' '
}

/// C.1.2 Non-ASCII space characters
pub fn non_ascii_space_character(c: char) -> bool {
    match c {
        '\u{00A0}' | '\u{1680}' | '\u{2000}' | '\u{2001}' | '\u{2002}' | '\u{2003}' |
        '\u{2004}' | '\u{2005}' | '\u{2006}' | '\u{2007}' | '\u{2008}' | '\u{2009}' |
        '\u{200A}' | '\u{200B}' | '\u{202F}' | '\u{205F}' | '\u{3000}' => true,
        _ => false,
    }
}

/// C.2.1 ASCII control characters
pub fn ascii_control_character(c: char) -> bool {
    match c {
        '\u{0000}'...'\u{001F}' |
        '\u{007F}' => true,
        _ => false,
    }
}

/// C.2.2 Non-ASCII control characters
pub fn non_ascii_control_character(c: char) -> bool {
    match c {
        '\u{0080}'...'\u{009F}' |
        '\u{06DD}' |
        '\u{070F}' |
        '\u{180E}' |
        '\u{200C}' |
        '\u{200D}' |
        '\u{2028}' |
        '\u{2029}' |
        '\u{2060}' |
        '\u{2061}' |
        '\u{2062}' |
        '\u{2063}' |
        '\u{206A}'...'\u{206F}' |
        '\u{FEFF}' |
        '\u{FFF9}'...'\u{FFFC}' |
        '\u{1D173}'...'\u{1D17A}' => true,
        _ => false,
    }
}

/// C.3 Private use
pub fn private_use(c: char) -> bool {
    match c {
        '\u{E000}'...'\u{F8FF}' |
        '\u{F0000}'...'\u{FFFFD}' |
        '\u{100000}'...'\u{10FFFD}' => true,
        _ => false,
    }
}

/// C.4 Non-character code points
pub fn non_character_code_point(c: char) -> bool {
    match c {
        '\u{FDD0}'...'\u{FDEF}' |
        '\u{FFFE}'...'\u{FFFF}' |
        '\u{1FFFE}'...'\u{1FFFF}' |
        '\u{2FFFE}'...'\u{2FFFF}' |
        '\u{3FFFE}'...'\u{3FFFF}' |
        '\u{4FFFE}'...'\u{4FFFF}' |
        '\u{5FFFE}'...'\u{5FFFF}' |
        '\u{6FFFE}'...'\u{6FFFF}' |
        '\u{7FFFE}'...'\u{7FFFF}' |
        '\u{8FFFE}'...'\u{8FFFF}' |
        '\u{9FFFE}'...'\u{9FFFF}' |
        '\u{AFFFE}'...'\u{AFFFF}' |
        '\u{BFFFE}'...'\u{BFFFF}' |
        '\u{CFFFE}'...'\u{CFFFF}' |
        '\u{DFFFE}'...'\u{DFFFF}' |
        '\u{EFFFE}'...'\u{EFFFF}' |
        '\u{FFFFE}'...'\u{FFFFF}' |
        '\u{10FFFE}'...'\u{10FFFF}' => true,
        _ => false,
    }
}

/// C.5 Surrogate codes
pub fn surrogate_code(c: char) -> bool {
    match c {
        // forbidden by rust
        /*'\u{D800}'...'\u{DFFF}' => true,*/
        _ => false,
    }
}

/// C.6 Inappropriate for plain text
pub fn inappropriate_for_plain_text(c: char) -> bool {
    match c {
        '\u{FFF9}' | '\u{FFFA}' | '\u{FFFB}' | '\u{FFFC}' | '\u{FFFD}' => true,
        _ => false,
    }
}

/// C.7 Inappropriate for canonical representation
pub fn inappropriate_for_canonical_representation(c: char) -> bool {
    match c {
        '\u{2FF0}'...'\u{2FFB}' => true,
        _ => false,
    }
}

/// C.8 Change display properties or are deprecated
pub fn change_display_properties_or_deprecated(c: char) -> bool {
    match c {
        '\u{0340}' | '\u{0341}' | '\u{200E}' | '\u{200F}' | '\u{202A}' | '\u{202B}' |
        '\u{202C}' | '\u{202D}' | '\u{202E}' | '\u{206A}' | '\u{206B}' | '\u{206C}' |
        '\u{206D}' | '\u{206E}' | '\u{206F}' => true,
        _ => false,
    }
}

/// C.9 Tagging characters
pub fn tagging_character(c: char) -> bool {
    match c {
        '\u{E0001}' |
        '\u{E0020}'...'\u{E007F}' => true,
        _ => false,
    }
}

/// D.1 Characters with bidirectional property "R" or "AL"
pub fn bidi_r_or_al(c: char) -> bool {
    match bidi_class(c) {
        BidiClass::R | BidiClass::AL => true,
        _ => false,
    }
}

/// D.2 Characters with bidirectional property "L"
pub fn bidi_l(c: char) -> bool {
    match bidi_class(c) {
        BidiClass::L => true,
        _ => false,
    }
}

// Each row of a lookup table contains:
// - A start, or only, character.
// - An optional end character, defining an inclusive range.
// - An optional replacement string.
type TableEntry<'a> = (char, Option<char>, Option<&'a str>);

// Try to find a character in a lookup table.
fn table_lookup<'a>(table: &'a [TableEntry<'a>], c: char) -> Option<TableEntry<'a>> {
    table
        .binary_search_by(|&(start, end, _)| match start.cmp(&c) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
            Ordering::Less => {
                match end {
                    Some(end) if c <= end => Ordering::Equal,
                    _ => Ordering::Less,
                }
            }
        })
        .ok()
        .map(|i| table[i])
}
