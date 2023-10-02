use std::io::Write;
use std::iter;

fn explosive<I: IntoIterator<Item = ()> + Clone + 'static>(iter: I) -> impl Iterator<Item = ()> {
    let mut res: Box<dyn Iterator<Item = ()>> = Box::new(iter::once(()));
    for () in iter.clone().into_iter() {
        let iter = iter.clone();
        res = Box::new(res.flat_map(move |()| iter.clone()));
    }
    res
}

fn main() {
    let count: usize = std::env::args().nth(1).unwrap().parse().unwrap();
    let mut lock = std::io::stdout().lock();
    write!(lock, "[").unwrap();
    let mut iter = explosive(iter::repeat(()).take(count)).peekable();
    while let Some(()) = iter.next() {
        write!(lock, "()").unwrap();
        if let Some(()) = iter.peek() {
            write!(lock, ",").unwrap();
        }
    }
    writeln!(lock, "]").unwrap();
}
