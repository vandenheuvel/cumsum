use std::ops::{AddAssign, Add};

pub fn cumsum<T>(x: &[T]) -> Vec<T>
where
    T: Clone,
    for<'r> &'r T: Add<&'r T, Output=T>,
{
    let mut y = Vec::with_capacity(x.len());

    if !x.is_empty() {
        y.push(x[0].clone());

        for i in 1..x.len() {
            y.push(&y[i - 1] + &x[i]);
        }
    }

    y
}

pub fn cumsum_owned<T>(mut x: Vec<T>) -> Vec<T>
where
    for<'r> T: AddAssign<&'r T>,
{
    for i in 1..x.len() {
        let (read, write) = x.split_at_mut(i);
        write[0] += &read[i - 1];
    }

    x
}

pub fn cumsum_array<T, const N: usize>(x: &[T; N]) -> [T; N]
where
    T: Add<Output=T> + Default + Copy,
{
    let mut y = [T::default(); N];

    if N > 0 {
        y[0] = x[0];

        for i in 1..x.len() {
            y[i] = y[i - 1] + x[i];
        }
    }

    y
}

pub fn cumsum_array_owned<T, const N: usize>(mut x: [T; N]) -> [T; N]
where
    for<'r> T: AddAssign<&'r T>,
{
    for i in 1..N {
        let (read, write) = x.split_at_mut(i);
        write[0] += &read[i - 1];
    }
    x
}

#[cfg(test)]
mod tests {
    use crate::{cumsum, cumsum_owned, cumsum_array, cumsum_array_owned};

    #[test]
    fn test() {
        assert_eq!(cumsum::<i32>(&[]), []);
        assert_eq!(cumsum(&[1]), [1]);
        assert_eq!(cumsum(&[1, 2, 3]), [1, 3, 6]);

        assert_eq!(cumsum_owned::<i32>(vec![]), vec![]);
        assert_eq!(cumsum_owned(vec![1]), vec![1]);
        assert_eq!(cumsum_owned(vec![1, 2, 3]), vec![1, 3, 6]);

        assert_eq!(cumsum_array::<i32, 0>(&[]), []);
        assert_eq!(cumsum_array(&[1]), [1]);
        assert_eq!(cumsum_array(&[1, 2, 3]), [1, 3, 6]);

        assert_eq!(cumsum_array_owned::<i32, 0>([]), []);
        assert_eq!(cumsum_array_owned([1]), [1]);
        assert_eq!(cumsum_array_owned([1, 2, 3]), [1, 3, 6]);
    }
}
