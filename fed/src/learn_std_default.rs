#[cfg(test)]
mod tests {

    use std::default::Default;

    #[test]
    fn learn_std_default_module() {
      // #[derive(Default)]를 주로 사용한다.
      #[derive(PartialEq)]
        enum Kind {
            A,
            B,
            C,
        }

        impl Default for Kind {
            fn default() -> Self {
                Kind::A
            }
        }

        let kv = Kind::default();
        assert!(kv == Kind::A); // PartialEq를 필요로 한다.
    }
}
