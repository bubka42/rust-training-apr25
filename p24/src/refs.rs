// f1 returns first or second element of tuple depending on the flag.
pub fn f1(pair: &mut (u32, u32), flag: bool) -> &mut u32 {
    match flag {
        true => &mut pair.0,
        false => &mut pair.1,
    }
}

// f2 returns n-th element of slice.
pub fn f2(slice: &mut [u32], n: usize) -> &mut u32 {
    &mut slice[n]
}

// f3 returns n-th element of slice from the end.
pub fn f3(slice: &mut [u32], n: usize) -> &mut u32 {
    let len = slice.len();
    &mut slice[len - n - 1]
}

// f4 breaks up a slice into four equal parts and returns all the parts.
pub fn f4(slice: &mut [u32]) -> (&mut [u32], &mut [u32], &mut [u32], &mut [u32]) {
    let len = slice.len();
    let quarter = len / 4;
    let (a, rest) = slice.split_at_mut(quarter);
    let (b, rest) = rest.split_at_mut(quarter);
    let (c, d) = rest.split_at_mut(quarter);
    (a, b, c, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f1() {
        let mut pair = (1, 2);
        assert_eq!(*f1(&mut pair, true), 1);
        *f1(&mut pair, false) = 3;
        assert_eq!(pair, (1, 3));
    }

    #[test]
    fn test_f2() {
        let mut slice = [1, 2, 3, 4];
        assert_eq!(*f2(&mut slice, 2), 3);
        *f2(&mut slice, 1) = 5;
        assert_eq!(slice, [1, 5, 3, 4]);
    }

    #[test]
    fn test_f3() {
        let mut slice = [1, 2, 3, 4];
        assert_eq!(*f3(&mut slice, 1), 3);
        *f3(&mut slice, 0) = 5;
        assert_eq!(slice, [1, 2, 3, 5]);
    }

    #[test]
    fn test_f4() {
        let mut slice = [1, 2, 3, 4, 5, 6, 7, 8];
        let (a, b, c, d) = f4(&mut slice);
        assert_eq!(a.len(), 2);
        assert_eq!(b.len(), 2);
        assert_eq!(c.len(), 2);
        assert_eq!(d.len(), 2);
        a[0] = 9;
        b[0] = 10;
        assert_eq!(a, &mut [9, 2]);
        assert_eq!(slice, [9, 2, 10, 4, 5, 6, 7, 8]);
    }
}
