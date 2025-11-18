/// Given a line and a starting column, returns the start and end of the current word
///
/// A word consists of:
/// 1. A sequence of letters, digits and underscores, OR
/// 2. A sequence of other non-blank characters
///
/// Separated with white space (spaces, tabs).
///
/// Returns None if positioned on whitespace or if the position is invalid.
pub fn word_boundaries(line: &str, col: usize) -> Option<(usize, usize)> {
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();

    if len == 0 || col >= len {
        return None;
    }

    // Return None if on whitespace
    if chars[col].is_whitespace() {
        return None;
    }

    let is_word = is_word_char(chars[col]);
    let mut start = col;
    let mut end = col;

    // Find start of word - continue while same type
    if is_word {
        // Word character: alphanumeric or underscore
        while start > 0 && is_word_char(chars[start - 1]) {
            start -= 1;
        }
        // Find end of word
        while end + 1 < len && is_word_char(chars[end + 1]) {
            end += 1;
        }
    } else {
        // Non-blank, non-word character (punctuation)
        while start > 0 && !chars[start - 1].is_whitespace() && !is_word_char(chars[start - 1]) {
            start -= 1;
        }
        // Find end of word
        while end + 1 < len && !chars[end + 1].is_whitespace() && !is_word_char(chars[end + 1]) {
            end += 1;
        }
    }

    Some((start, end))
}

/// Returns true if the character is a word character (alphanumeric or underscore)
fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_boundaries_forward() {
        let line = "Hello, world! This is a test.";
        assert_eq!(word_boundaries(line, 0), Some((0, 4))); // "Hello"
        assert_eq!(word_boundaries(line, 4), Some((0, 4))); // "Hello"
        assert_eq!(word_boundaries(line, 5), Some((5, 5))); // ","
        assert_eq!(word_boundaries(line, 6), None); // space
        assert_eq!(word_boundaries(line, 7), Some((7, 11))); // "world"
        assert_eq!(word_boundaries(line, 12), Some((12, 12))); // "!"
        assert_eq!(word_boundaries(line, 13), None); // space
        assert_eq!(word_boundaries(line, 14), Some((14, 17))); // "This"
        assert_eq!(word_boundaries(line, 24), Some((24, 27))); // "test"
        assert_eq!(word_boundaries(line, 28), Some((28, 28))); // "."
    }

    #[test]
    fn test_word_boundaries_backward() {
        let line = "Hello, world! This is a test.";
        assert_eq!(word_boundaries(line, 28), Some((28, 28))); // "."
        assert_eq!(word_boundaries(line, 27), Some((24, 27))); // "test"
        assert_eq!(word_boundaries(line, 23), None); // space
        assert_eq!(word_boundaries(line, 22), Some((22, 22))); // "a"
        assert_eq!(word_boundaries(line, 20), Some((19, 20))); // "is"
        assert_eq!(word_boundaries(line, 17), Some((14, 17))); // "This"
        assert_eq!(word_boundaries(line, 12), Some((12, 12))); // "!"
        assert_eq!(word_boundaries(line, 11), Some((7, 11))); // "world"
        assert_eq!(word_boundaries(line, 5), Some((5, 5))); // ","
        assert_eq!(word_boundaries(line, 4), Some((0, 4))); // "Hello"
        assert_eq!(word_boundaries(line, 0), Some((0, 4))); // "Hello"
    }

    #[test]
    fn test_single_letter_word() {
        let line = "A B C";
        assert_eq!(word_boundaries(line, 0), Some((0, 0))); // "A"
        assert_eq!(word_boundaries(line, 1), None); // space
        assert_eq!(word_boundaries(line, 2), Some((2, 2))); // "B"
        assert_eq!(word_boundaries(line, 3), None); // space
        assert_eq!(word_boundaries(line, 4), Some((4, 4))); // "C"
    }

    #[test]
    fn test_punctuation_words() {
        let line = "foo->bar = '*=*';";
        assert_eq!(word_boundaries(line, 0), Some((0, 2))); // "foo"
        assert_eq!(word_boundaries(line, 3), Some((3, 4))); // "->"
        assert_eq!(word_boundaries(line, 5), Some((5, 7))); // "bar"
        assert_eq!(word_boundaries(line, 8), None); // space
        assert_eq!(word_boundaries(line, 9), Some((9, 9))); // "="
        assert_eq!(word_boundaries(line, 10), None); // space
        assert_eq!(word_boundaries(line, 11), Some((11, 16))); // "'*=*'" - all punctuation grouped
        assert_eq!(word_boundaries(line, 17), None); // past end
    }

    #[test]
    fn test_underscore_in_word() {
        let line = "foo_bar baz_qux";
        assert_eq!(word_boundaries(line, 0), Some((0, 6))); // "foo_bar"
        assert_eq!(word_boundaries(line, 3), Some((0, 6))); // "foo_bar" (from underscore)
        assert_eq!(word_boundaries(line, 7), None); // space
        assert_eq!(word_boundaries(line, 8), Some((8, 14))); // "baz_qux"
    }
}
