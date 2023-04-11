#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::cell::{self};
    use std::cell::{Ref, RefCell, RefMut};
    use std::collections::HashMap;
    use std::marker::PhantomData;
    use std::process::abort;
    use std::ptr::NonNull;
    use std::rc::Rc;

    #[test]
    fn learn_std_cell_module() {
        // Introducing mutability ‘inside’ of something immutable
        // Implementation details of logically-immutable methods.
        // Mutating implementations of Clone.
    }

    #[test]
    fn learn_std_cell_examples() {
        // RefCell은 immutable / mutable borrow count를 트래킹 한다.
        let shared_map = Rc::new(RefCell::new(HashMap::new()));
        // Create a new block to limit the scope of the dynamic borrow
        {
            let mut map = shared_map.borrow_mut();
            map.insert("africa", 92388);
            map.insert("kyoto", 11837);
            map.insert("piccadilly", 11826);
            map.insert("marbles", 38);
        }

        // Note that if we had not let the previous borrow of the cache fall out
        // of scope then the subsequent borrow would cause a dynamic thread panic.
        // This is the major hazard of using `RefCell`.
        let total: i32 = shared_map.borrow().values().sum();
        println!("{total}");
    }

    #[test]
    fn learn_std_cell_hide_mutablility_for_clone() {
        //
        struct Rc<T: ?Sized> {
            ptr: NonNull<RcBox<T>>,
            phantom: PhantomData<RcBox<T>>,
        }

        struct RcBox<T: ?Sized> {
            strong: Cell<usize>,
            refcount: Cell<usize>,
            value: T,
        }

        impl<T: ?Sized> Clone for Rc<T> {
            fn clone(&self) -> Rc<T> {
                self.inc_strong();
                Rc {
                    ptr: self.ptr,
                    phantom: PhantomData,
                }
            }
        }

        trait RcBoxPtr<T: ?Sized> {
            fn inner(&self) -> &RcBox<T>;

            fn strong(&self) -> usize {
                self.inner().strong.get()
            }

            fn inc_strong(&self) {
                self.inner()
                    .strong
                    .set(self.strong().checked_add(1).unwrap_or_else(|| abort()));
            }
        }

        impl<T: ?Sized> RcBoxPtr<T> for Rc<T> {
            fn inner(&self) -> &RcBox<T> {
                unsafe { self.ptr.as_ref() }
            }
        }

        // new() 함수를 구현하지 않아서 테스트 하려면 필요하다.
        // 좋은 연습.
    }

    #[test]
    fn learn_std_cell_struct() {
        struct SomeStruct {
            regular_field: u8,
            special_field: Cell<u8>,
        }

        let my_struct = SomeStruct {
            regular_field: 0,
            special_field: Cell::new(1),
        };

        let new_value = 100;

        // ERROR: `my_struct` is immutable
        // my_struct.regular_field = new_value;

        // WORKS: although `my_struct` is immutable, `special_field` is a `Cell`,
        // which can always be mutated
        my_struct.special_field.set(new_value);
        assert_eq!(my_struct.special_field.get(), new_value);

        // Cell은 ptr을 보던 때와 다르게 상위 코드이다.
        // 이 정도에서도 필요한 것들을 만들 수 있다.
        // 몇 가지 도구가 더 있어야 할 것 같다. Unique<T>와 같은 것?
    }

    #[test]
    fn learn_std_cell_functions() {
        // swap
        let c1 = Cell::new(5i32);
        let c2 = Cell::new(10i32);
        c1.swap(&c2);
        assert_eq!(10, c1.get());
        assert_eq!(5, c2.get());

        // Cell은 UnsafeCell을 통해 unsafe casting으로 메모리를 얻은 후 값을 지정.
        // 이전 값은 drop된다.
        c2.set(3i32);
        assert_eq!(3, c2.get());

        // into_inner
        let c = Cell::new(5);
        let five = c.into_inner(); // move 된다. 어디서? UnsafeCell::into_inner()를 통해

        assert_eq!(five, 5);

        // get
        // update : 함수 버전
        // as_ptr
        // get_mut
        // from_mut
        // take : Default::default() 값을 남겨 둔다.
        //
    }

    #[test]
    fn learn_std_ref_struct() {
        // Ref는 RefCell::borrow()의 반환값
        let c = RefCell::new((5, 'b'));
        let b1: Ref<(u32, char)> = c.borrow();
        let b2: Ref<u32> = Ref::map(b1, |t| &t.0);
        assert_eq!(*b2, 5)
    }

    #[test]
    fn learn_std_ref_functions() {
        // clone
        // map
        // filter_map
        let c = RefCell::new(vec![1, 2, 3]);
        let b1: Ref<Vec<u32>> = c.borrow();
        let b2: Result<Ref<u32>, _> = Ref::filter_map(b1, |v| v.get(1));
        assert_eq!(*b2.unwrap(), 2);

        // map_split
    }

    #[test]
    fn learn_std_refcell_struct() {
        // 동적으로 borrow 체크를 한다.

        let c = RefCell::new(5);
        let m = c.borrow_mut();
        let b = c.borrow(); // this causes a panic
    }

    #[test]
    fn learn_std_refcell_functions() {
        // new
        // into_inner
        // replace
        // replace_with
        // swap
        // borrow
        // try_borrow
        // borrow_mut
        // try_borrow_mut
        // as_ptr
        // get_mut
        let mut c = RefCell::new(5);
        *c.get_mut() += 1;

        assert_eq!(c, RefCell::new(6));
        // try_borrow_unguarded
        // take
        //
    }

    #[test]
    fn learn_std_unsafecell_struct() {}
}
