pub fn cycle<T: PartialEq + Copy>(val: &mut T, items: &[T], forward: bool) {
    if let Some(pos) = items.iter().position(|x| x == val) {
        let n = items.len();
        *val = items[if forward { pos + 1 } else { pos + n - 1 } % n];
    }
}
