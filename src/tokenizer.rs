pub fn string_to_token(str: String) -> Vec<String> {
    str.split(|c: char| c.is_whitespace() || c == '(' || c == ')' || c == ':')
        .filter(|s| !s.is_empty())
        .map(|x| x.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_string_to_token_with_colon() {
        let input = String::from("hello: world");
        let expected = vec![String::from("hello"), String::from("world")];
        assert_eq!(string_to_token(input), expected);
    }

    #[test]
    fn test_string_to_token_with_brackets() {
        let input = String::from("(hello) world");
        let expected = vec![String::from("hello"), String::from("world")];
        assert_eq!(string_to_token(input), expected);
    }

    #[test]
    fn test_string_to_token_empty() {
        let input = String::from("");
        let expected: Vec<String> = Vec::new();
        assert_eq!(string_to_token(input), expected);
    }

    #[test]
    fn test_string_to_token_multiple_spaces() {
        let input = String::from("hello   world");
        let expected = vec![String::from("hello"), String::from("world")];
        assert_eq!(string_to_token(input), expected);
    }

    #[test]
    fn test_string_to_token_newline() {
        let input = String::from("hello\nworld");
        let expected = vec![String::from("hello"), String::from("world")];
        assert_eq!(string_to_token(input), expected);
    }
}
