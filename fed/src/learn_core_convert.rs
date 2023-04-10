#[cfg(test)]
mod tests {

    #[test]
    fn learn_core_convert() {
        // AsRef
        // AsMut
        // From
        // Into
        // TryFrom
        // TryInto
    }

    #[test]
    fn learn_core_convert_asmut() {
        // fn as_mut(&mut self) -> &mut T; of AsMut<T: ?Sized>

        struct Document {
            info: String,
            content: Vec<u8>,
        }

        impl<T: ?Sized> AsMut<T> for Document
        where
            Vec<u8>: AsMut<T>, // Vec<T>가 AsMut<T>를 구현하고 있다.
        {
            fn as_mut(&mut self) -> &mut T {
                self.content.as_mut()
            }
        }

        fn caesar<T: AsMut<[u8]>>(data: &mut T, key: u8) {
            for byte in data.as_mut() {
                *byte = byte.wrapping_add(key);
            }
        }

        fn null_terminate<T: AsMut<Vec<u8>>>(data: &mut T) {
            // Using a non-generic inner function, which contains most of the
            // functionality, helps to minimize monomorphization overhead.
            fn doit(data: &mut Vec<u8>) {
                let len = data.len();
                if len == 0 || data[len - 1] != 0 {
                    data.push(0);
                }
            }
            doit(data.as_mut());
        }

        let mut v: Vec<u8> = vec![1, 2, 3];
        caesar(&mut v, 5);
        assert_eq!(v, [6, 7, 8]);
        null_terminate(&mut v);
        assert_eq!(v, [6, 7, 8, 0]);
        let mut doc = Document {
            info: String::from("Example"),
            content: vec![17, 19, 8],
        };
        // doc의 as_mut가 불린다. 그러면  &mut Vec<u8>이 된다.
        caesar(&mut doc, 1);
        assert_eq!(doc.content, [18, 20, 9]);
        null_terminate(&mut doc);
        assert_eq!(doc.content, [18, 20, 9, 0]);
    }

    #[test]
    fn learn_core_convert_asref() {
        // fn as_ref(&self) -> &T;
        // Borrow는 통으로 빌린다. Hash, Eq, Ord가 borrowed value와 owned value가 같아야 한다.

        // Deref를 구현할 경우의 AsRef구현 참고

        fn is_hello<T: AsRef<str>>(s: T) {
            assert_eq!("hello", s.as_ref());
        }

        let s = "hello";
        is_hello(s);

        let s = "hello".to_string();
        is_hello(s);
    }

    #[test]
    fn learn_core_convert_from() {
        let string = "hello".to_string();
        let other_string = String::from("hello");
        // 내부적으로 slice와 벡터간 변환을 사용한다.

        assert_eq!(string, other_string);
    }

    #[test]
    fn learn_core_convert_tryfrom() {
        // 64비트 정수에서 32비트 정수 변환을 안전하게 할 수 있다. 
        // 이와 같은 점들이 러스트의 장점이다. 

        let big_number = 1_000_000_000_000i64;
        // Silently truncates `big_number`, requires detecting
        // and handling the truncation after the fact.
        let smaller_number = big_number as i32;
        assert_eq!(smaller_number, -727379968);

        // Returns an error because `big_number` is too big to
        // fit in an `i32`.
        let try_smaller_number = i32::try_from(big_number);
        assert!(try_smaller_number.is_err());

        // Returns `Ok(3)`.
        let try_successful_smaller_number = i32::try_from(3);
        assert!(try_successful_smaller_number.is_ok());
    }
}
