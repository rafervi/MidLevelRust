use std::ops::{Deref, DerefMut};

struct MySmartPointer<T> {
    value: T
}

impl<T> MySmartPointer<T> {
    fn new(value: T) -> MySmartPointer<T> {
        MySmartPointer { value }
    }
}

impl<T> Deref for MySmartPointer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for MySmartPointer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

fn main() {
    let mut s = MySmartPointer::new(Box::new("Let's get Rusty".to_owned()));
    //let s = &(*s); //the more Deref(*) in (*s) the rigther goes from MySmartPointer
    // &MySmartPointer -> &Box -> &String -> &str //no runtime cost to use Deref Coercion. Deref And DerefMut only use on smart pointers types
    print(&mut s);
}


fn print(s: &str){
    println!("{s}");
}
