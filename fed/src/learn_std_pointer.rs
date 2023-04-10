#[cfg(test)]
mod tests {

    #[test]
    fn learn_std_pointer() {
        {
            let my_num: i32 = 10;
            let my_num_ptr: *const i32 = &my_num;
            let mut my_speed: i32 = 88;
            let my_speed_ptr: *mut i32 = &mut my_speed;
        }

        {
            let my_num: Box<i32> = Box::new(10);
            let my_num_ptr: *const i32 = &*my_num;
            let mut my_speed: Box<i32> = Box::new(88);
            let my_speed_ptr: *mut i32 = &mut *my_speed;

            let my_num = unsafe { *my_speed_ptr }; // my_speed가 drop되면 my_speed_ptr은 invalid
        }

        {
            let my_speed: Box<i32> = Box::new(88);
            let my_speed: *mut i32 = Box::into_raw(my_speed);

            // By taking ownership of the original `Box<T>` though
            // we are obligated to put it together later to be destroyed.
            unsafe {
                // 아래 코드가 없으면 누수(leak)이다.
                drop(Box::from_raw(my_speed));
            }
        }

        {
            #[derive(Debug, Default, Copy, Clone)]
            #[repr(C, packed)]
            struct S {
                aligned: u8,
                unaligned: u32, // packed면 바이트 정렬이라 alignment가 u32에 맞지 않는다.
            }
            let s = S::default();
            let p = std::ptr::addr_of!(s.unaligned); // not allowed with coercion.
        }
    }

    // fn is_null(self) -> bool
    // const fn cast<U>(self) -> *const U
    // const fn cast_mut(self) -> *mut T
    // unsafe fn as_ref<'a>(self) -> Option<&'a T>

    #[test]
    fn learn_std_pointer_as_ref() {
        let ptr: *const u8 = &10u8 as *const u8;

        unsafe {
            if let Some(val_back) = ptr.as_ref() {
                println!("We got back the value: {val_back}!");
            }
        }
    }

    // const unsafe fn offset(self, count: isize) -> *const T 
    // const fn wrapping_offset(self, count: isize) -> *const T
    // const unsafe fn offset_from(self, origin: *const T) -> isize 
    // const unsafe fn add(self, count: usize) -> *const T
    // const unsafe fn sub(self, count: usize) -> *const T
    // const fn wrapping_add(self, count: usize) -> *const T
    // const fn wrapping_sub(self, count: usize) -> *const T
    // unsafe fn read(self) -> T
    // unsafe fn read_volatile(self) -> T
    // unsafe fn read_unaligned(self) -> T
    // const unsafe fn copy_to(self, dest: *mut T, count: usize)
    // const unsafe fn copy_to_nonoverlapping(self, dest: *mut T, count: usize)
    // fn align_offset(self, align: usize) -> usize
 
    // 많은 함수들이 있고 impl<T: ?Sized> *const T, impl<T: ?Sized> *mut T에 대해 
    // 각 const_ptr.rs, mut_ptr.rs에서 유사하거나 같은 함수들을 정의한다. 

}
