#![allow(unused)]
fn main() {
    let mut v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();
    let v1_sum:i32 = v1_iter.sum(); //sum() is a "consumer adaptor" takes ownership of iterator
    // here we can not use v1_iter, sum took ownership

    for elem in v1.iter().map(|x|x*x) {
        println!("Got: {}", elem);
    }

    println!("v1: {:?}", v1);
    println!("Sum: {:?}", v1_sum);
    

    // map() is an "iterator adaptors"
    // changes iterator into other kind of itetarors
    //
    // Important, iterators are lazy, you have to call
    // one of the consuming adaptors to get results e.g. "collect()"
    let v2: Vec<_> = v1.iter().map(|x|format!("E:{}",x)).collect();
    println!("v2: {:?}", v2);
;
}
