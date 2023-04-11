#[cfg(test)]
mod tests {
    use std::string;

    #[test]
    fn learn_std_string() {
        let s = "Hello".to_string();

        let s = String::from("world");
        let s: String = "also this".into();
        // U: ~const From<T> 로 into() 함수의 where에 제약한다. ~가 무슨 뜻인가?
        //
    }

    #[test]
    fn learn_std_string_struct() {
        let mut hello = String::from("Hello, ");

        hello.push('w');
        hello.push_str("orld!");

        let s = "hello";
        let third_character = s.chars().nth(2);
        assert_eq!(third_character, Some('l'));

        let s = "💖💖💖💖💖";
        let third_character = s.chars().nth(2);
        assert_eq!(third_character, Some('💖'));

        // The first byte is 104 - the byte value of `'h'`
        let s = "hello";
        assert_eq!(s.as_bytes()[0], 104);
        // or
        assert_eq!(s.as_bytes()[0], b'h');

        // The first byte is 240 which isn't obviously useful
        let s = "💖💖💖💖💖";
        assert_eq!(s.as_bytes()[0], 240);

        // String은 내부에 Vec<u8>을 사용하고 Char와 Chars의
        // 도움을 받아 문자열 처리를 한다.
    }

    #[test]
    fn learn_std_string_retain() {
        let mut s = String::from("f_o_ob_ar");

        s.retain(|c| c != '_');

        assert_eq!(s, "foobar");
    }

    #[test]
    fn learn_std_string_drain() {
        let mut s = String::from("α is alpha, β is beta");
        let beta_offset = s.find('β').unwrap_or(s.len());

        // Remove the range up until the β from the string
        let t: String = s.drain(..beta_offset).collect();
        assert_eq!(t, "α is alpha, ");
        assert_eq!(s, "β is beta");

        // A full range clears the string, like `clear()` does
        s.drain(..);
        assert_eq!(s, "");
    }

    // contains, starts_with, ends_with, split, find
    // split_terminator, match_indices, trim, trim_start
    // trim_left, trim_right, trim_matches, trim_start_matches
    // strip_prefix, strip_suffix, parse
    // is_ascii, make_ascii_lowercase, replace

    #[test]
    fn learn_std_string_replace() {
        let s = "this is old";

        assert_eq!("this is new", s.replace("old", "new"));
        assert_eq!("than an old", s.replace("is", "an"));
    }
}
