extern "C" {
    fn abs(input: i32) -> i32;
}

static HELLO_WORLD: &str = "Hello, world!";

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

#![allow(unused)]
fn main() {
    use std::slice;

    let mut num = 5;

    let r1: &i32 = &num as *const i32;
    let r2: &i32 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
    
        assert!(mid <= len);
    
        unsafe {
        (slice::from_raw_parts_mut(ptr, mut),
         slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
        }
    }

    let address = 0x012345usize;
    let r = address as *mut i32;

    let slice = unsafe {
        slice::from_raw_parts_mut(r, 10000)
    }

    unsafe {
        // -3の絶対値は、Cによると{}
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // 名前は: {}
    println!("name is: {}", HELLO_WORLD);

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    unsafe trait Foo {
        // methods go here
        // メソッドがここに来る
    }

    unsafe impl Foo for i32 {
        // methods implemented go here
        // メソッドの実装がここに来る
    }
}

