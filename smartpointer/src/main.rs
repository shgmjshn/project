use std::rc::{Rc, Weak};
use std::ops::Deref;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value:i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

use List::{Cons, Nil};

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        // CustomSmartPointerをデータ`{}`とともにドロップするよ
        println!("Dropping CustomSmartPointer!");
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let leaf = Rc::new(Node {
        value:3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        // leafのstrong_count = {}, weak/count = {}
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        // leafの親 = {:?}
        let branch = Rc::new(Node {
            value:5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            // branchのstrong_count = {}, weak_count = {}
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    let value = Rc::new(RefCell::new(5));

    let b = Box::new(5);
    println!("b = {}", b);

    let _c = CustomSmartPointer { data: String::from("some data") }; // 俺のもの
    println!("CustomSmartPointers created."); // CustomSmartPointerが作成された
    drop(_c);
    // CustomSmartPointerはmainが終わる前にドロップされた
    println!("CustomSmartPointer droped before the end of main.");

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    
    // aの最初のアカウント参照 = {}
    println!("a initial rc count = {}", Rc::strong_count(&a));
    // aの次の要素は = {:?}
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    // b作成後のaの参照カウント = {}
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    // bの最初のアカウント参照 = {}
    println!("b initial rc count = {}", Rc::strong_count(&b));
    // aの次の要素は = {:?}
    println!("a next item = {:?}", a.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    
    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);

     // aを変更後のbの参照カウント = {}
     println!("b rc count after changing a = {}", Rc::strong_count(&b));
     // aを変更後のaの参照カウント = {}
     println!("a rc count after changing a = {}", Rc::strong_count(&a));
 
     // Uncomment the next line to see that we have a cycle;
     // it will overflow the stack
     // 次の行のコメントを外して循環していると確認してください; スタックオーバーフローします
     println!("a next item = {:?}", a.tail());        // aの次の要素 = {:?}

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
