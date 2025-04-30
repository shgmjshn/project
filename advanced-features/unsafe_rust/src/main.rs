#![allow(unused)]

use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn outline_print<T: fmt::Display>(item: &T) {
    let output = format!("{}", item);
    let len = output.len();
    println!("{}", "*".repeat(len + 4));
    println!("* {} *", output);
    println!("{}", "*".repeat(len + 4));
}

unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

static HELLO_WORLD: &str = "Hello, world!";

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    use std::slice;

    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

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
            let ptr = slice.as_mut_ptr();
            (slice::from_raw_parts_mut(ptr, mid),
             slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
        }
    }

    let address = 0x012345usize;
    let r = address as *mut i32;

    let slice = unsafe {
        slice::from_raw_parts_mut(r, 10000)
    };

    unsafe {
        // -3の絶対値は、Cによると{}
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // 名前は: {}
    println!("name is: {}", HELLO_WORLD);

    add_to_count(3);

    unsafe {
        let counter = COUNTER;
        println!("COUNTER: {}", counter);
    }

    unsafe trait Foo {
        // methods go here
        // メソッドがここに来る
    }

    unsafe impl Foo for i32 {
        // methods implemented go here
        // メソッドの実装がここに来る
    }

    let point = Point { x: 3, y: 4 };
    outline_print(&point);
}

