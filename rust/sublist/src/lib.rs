use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_sublist<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    let slice_length = a.len();

    if slice_length == 0 {
        return true;
    }

    for idx_start in b
        .iter()
        .enumerate()
        .filter_map(|(i, val)| ((i + slice_length) <= b.len() && *val == a[0]).then(|| i))
    {
        if a == &b[idx_start..(idx_start + slice_length)] {
            return true;
        }
    }

    return false;
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    use Comparison::*;

    match a.len().cmp(&b.len()) {
        Ordering::Less if is_sublist(a, b) => Sublist,
        Ordering::Equal if a == b => Equal,
        Ordering::Greater if is_sublist(b, a) => Superlist,
        _ => Unequal,
    }
}
