use alloc::vec::Vec;

fn normalize_latvian(c: char) -> char {
    match c {
        'ā' | 'a' => 'a',
        'č' | 'c' => 'c',
        'ē' | 'e' => 'e',
        'ģ' | 'g' => 'g',
        'ķ' | 'k' => 'k',
        'ļ' | 'l' => 'l',
        'ņ' | 'n' => 'n',
        'ō' | 'o' => 'o',
        'ŗ' | 'r' => 'r',
        'ū' | 'u' => 'u',
        'š' | 's' => 's',
        'ž' | 'z' => 'z',
        other => other,
    }
}

fn normalize_char(c: char, case_sensitive: bool) -> char {
    let c = if case_sensitive {
        c
    } else {
        c.to_lowercase().next().unwrap_or(c)
    };

    normalize_latvian(c)
}

fn normalize_string(s: &str, case_sensitive: bool) -> Vec<char> {
    s.chars()
        .map(|c| normalize_char(c, case_sensitive))
        .collect()
}

pub fn find_substr(str: &str, substr: &str, diacritic: bool, case_sensitive: bool) -> Vec<u32> {
    let mut positions = Vec::new();

    if substr.is_empty() {
        return positions;
    }

    let text: Vec<char> = if diacritic {
        normalize_string(str, case_sensitive)
    } else {
        str.chars()
            .map(|c| {
                if case_sensitive {
                    c
                } else {
                    c.to_lowercase().next().unwrap_or(c)
                }
            })
            .collect()
    };

    let pattern: Vec<char> = if diacritic {
        normalize_string(substr, case_sensitive)
    } else {
        substr
            .chars()
            .map(|c| {
                if case_sensitive {
                    c
                } else {
                    c.to_lowercase().next().unwrap_or(c)
                }
            })
            .collect()
    };

    let n = text.len();
    let m = pattern.len();

    if m > n {
        return positions;
    }

    for i in 0..=(n - m) {
        if text[i..i + m] == pattern[..] {
            positions.push(i as u32);
        }
    }

    positions
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    // BASIC

    #[test]
    fn single_match() {
        let result = find_substr("hello world", "world", false, false);
        assert_eq!(result, vec![6]);
    }

    #[test]
    fn multiple_matches() {
        let result = find_substr("aaaaa", "aa", false, false);
        assert_eq!(result, vec![0, 1, 2, 3]);
    }

    #[test]
    fn no_match() {
        let result = find_substr("hello", "xyz", false, false);
        let expected: Vec<u32> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn empty_substring() {
        let result = find_substr("hello", "", false, false);
        let expected: Vec<u32> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn full_match() {
        let result = find_substr("abc", "abc", false, false);
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn overlapping_matches() {
        let result = find_substr("aaaa", "aa", false, false);
        assert_eq!(result, vec![0, 1, 2]);
    }

    // CASE SENSITIVITY TESTS

    #[test]
    fn case_sensitive_match() {
        let result = find_substr("Hello", "hello", false, true);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn case_insensitive_match() {
        let result = find_substr("Hello", "hello", false, false);
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn mixed_case_multiple() {
        let result = find_substr("HeLLo HeLLo", "hello", false, false);
        assert_eq!(result, vec![0, 6]);
    }

    // DIACRITICS

    #[test]
    fn diacritic_equal_basic() {
        let result = find_substr("āa", "aa", true, false);
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn diacritic_not_equal_when_disabled() {
        let result = find_substr("āa", "aa", false, false);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn ch_equals_c() {
        let result = find_substr("čau", "cau", true, false);
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn sh_equals_s_multiple() {
        let result = find_substr("ššs", "sss", true, false);
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn uu_equals_u() {
        let result = find_substr("sūna", "suna", true, false);
        assert_eq!(result, vec![0]);
    }

    // OTHER

    #[test]
    fn case_and_diacritic_combined() {
        let result = find_substr("ŠaU", "sau", true, false);
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn strict_case_and_diacritic_fail() {
        let result = find_substr("ŠaU", "sau", false, true);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn repeated_pattern_with_diacritics() {
        let result = find_substr("āāāā", "aa", true, false);
        assert_eq!(result, vec![0, 1, 2]);
    }

    #[test]
    fn mix() {
        let text = "Ģēģa ķēķis ļaunā ņemšana";
        let pattern = "gega kekis launa nemsana";

        let result = find_substr(text, pattern, true, false);
        assert_eq!(result, vec![0]);
    }
}
