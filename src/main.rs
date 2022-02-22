use std::rc::Rc;
fn main() {
    let r = Rc::new(1);
    let b = r.as_ref();
    println!("{:?}", b);
}
