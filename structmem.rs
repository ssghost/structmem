use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
struct Seq {
    id: i32,
    vals: Rc<RefCell<Vec<i32>>>,
}

#[no_mangle]
#[inline(never)]
fn upped_id(mut seq: Seq) -> Seq {
    seq.id += 1;
    seq
}

#[no_mangle]
#[inline(never)]
fn report(seq: &Seq) {
    println!("{} has {}", seq.id, seq.vals.borrow().len());
}

#[no_mangle]
#[inline(never)]
fn report_changes(a: Seq) {
    let b = upped_id(a.clone());
    a.vals.borrow_mut().push(0);
    report(&a);
    report(&b);
}

fn main() {
    let a = Seq {
        id: 5,
        vals: Rc::new(RefCell::new(vec![6, 7])),
    };
    report_changes(a.clone());
}
