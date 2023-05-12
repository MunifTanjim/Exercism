use std::cmp::Ordering::{Equal, Greater, Less};

pub fn find<T: Ord, A: AsRef<[T]>>(array: A, key: T) -> Option<usize> {
    let slice = array.as_ref();
    let mi = slice.len() / 2;
    match key.cmp(slice.get(mi)?) {
        Equal => Some(mi),
        Less => find(&slice[..mi], key),
        Greater => find(&slice[mi + 1..], key).map(|i| i + mi + 1),
    }
}
