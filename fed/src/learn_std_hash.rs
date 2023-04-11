#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    #[test]
    fn learn_std_hash_module() {
      // 포함된 데이터에 기반해서 해시를 만든다. 
      // 러스트의 매크로는 강력하다. 이런 코드도 생성 가능하다. 
        #[derive(Hash)]
        struct Person {
            id: u32,
            name: String,
            phone: u64,
        }

        let person1 = Person {
            id: 5,
            name: "Janet".to_string(),
            phone: 555_666_7777,
        };

        let person2 = Person {
            id: 5,
            name: "Bob".to_string(),
            phone: 555_666_7777,
        };

        assert!(calculate_hash(&person1) != calculate_hash(&person2));

        fn calculate_hash<T: Hash>(t: &T) -> u64 {
            let mut s = DefaultHasher::new();
            t.hash(&mut s);
            s.finish()
        }
    }
}
