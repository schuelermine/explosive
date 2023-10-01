use std::iter;
use std::io::Write;

fn explosive<I: IntoIterator<Item = ()> + Clone + 'static>(iter: I) -> impl Iterator<Item = ()> {
    let mut res: Box<dyn Iterator<Item = ()>> = Box::new(iter::once(()));
    for () in iter.clone().into_iter() {
        let iter = iter.clone();
        res = Box::new(res.flat_map(move |()| iter.clone()));
    }
    res
}

fn main() {
    let count: usize = std::env::args().skip(1).next().unwrap().parse().unwrap();
    print!("[");
    let mut iter = explosive(iter::repeat(()).take(count)).peekable();
    let mut lock = std::io::stdout().lock();
    while let Some(()) = iter.next() {
        write!(lock, "()").unwrap();
        if let Some(()) = iter.peek() {
            write!(lock, ",").unwrap();
        }
    }
    println!("]");
}
