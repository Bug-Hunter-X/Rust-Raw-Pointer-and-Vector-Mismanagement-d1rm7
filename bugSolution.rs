fn main() {
    let mut v = vec![1, 2, 3];
    let index = 0;
    unsafe {
        let ptr = v.as_mut_ptr().add(index);
        *ptr = 10;
    }
    println!("v: {:?}", v);
}