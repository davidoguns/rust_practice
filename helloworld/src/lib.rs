#[warn(clippy::pedantic)]

pub mod foo;
pub mod point;

use foo::do_foo_sep;

pub fn do_bar() {
    println!("Doing bar...");
    foo::do_foo();
    do_foo_sep();
}

pub fn find_largest_num1<T>(v: &Vec<T>) -> Option<&T>
where
    T: PartialOrd,
{
    let mut largest_num = Option::<&T>::None;
    v.iter()
        .for_each(|n| largest_num = Some(largest_num.map_or(n, |m| if m > n { m } else { n })));
    largest_num
}

pub fn find_largest_num2<T>(v: &Vec<T>) -> Option<&T>
where
    T: PartialOrd,
{
    let mut largest_num = Option::<&T>::None;
    v.iter().for_each(|n| {
        if let Some(largest_n) = largest_num {
            if *largest_n < *n {
                largest_num = Some(n);
            }
        } else {
            largest_num = Some(n);
        }
    });
    largest_num
}

pub fn find_largest_in_slice<T: PartialOrd>(v: &[T]) -> Option<&T> {
    if v.len() == 0 {
        return None
    }
    let mut largest = &v[0];

    for item in v {
        if item > largest {
            largest = item;
        }
    }
    Some(largest)
}

#[cfg(test)]
pub mod test {
    use crate::find_largest_in_slice;
    use crate::find_largest_num1;
    use crate::find_largest_num2;
    use crate::point::Point;
    use rand::RngExt;

    #[test]
    pub fn point_stuff() {
        let p1 = Point::new(&1, &1);
        assert!(p1.is_some(), "Point should be some!");
        let p2 = Point::new(&i32::default(), &i32::default());
        assert!(p2.is_none(), "Point should be none!");
        println!("i32::default = {}", i32::default());
    }

    #[test]
    pub fn test_find_largest_slice() {
        let arr = [23, 19, 0, 199, 15];
        let slice = &arr[..];
        let largest_in_empty = find_largest_in_slice(slice);
        assert_eq!(largest_in_empty, Some(&199));
    }

    #[test]
    pub fn test_find_largest_compare() {
        println!("i32 max: {}", i32::MAX);
        const VECSIZE_1: usize = 16384;
        let mut v1 = Vec::<i32>::with_capacity(VECSIZE_1);
        let mut rng = rand::rng();
        for _ in 0..VECSIZE_1 {
            v1.push(rng.random());
        }
        println!("v1 size: {}", v1.len());
        let max_1 = find_largest_num1(&v1);
        let max_2 = find_largest_num2(&v1);
        println!("Max 1: {}", max_1.unwrap());
        println!("Max 2: {}", max_2.unwrap());
        assert_eq!(max_1.unwrap(), max_2.unwrap());

        const VECSIZE_2: usize = 8192;
        let mut v2 = Vec::<i32>::with_capacity(VECSIZE_2);
        for _ in 0..VECSIZE_2 {
            v2.push(rng.random());
        }
        println!("v2 size: {}", v2.len());
        let max_1 = find_largest_num1(&v2);
        let max_2 = find_largest_num2(&v2);
        println!("Max 1: {}", max_1.unwrap());
        println!("Max 2: {}", max_2.unwrap());
        assert_eq!(max_1.unwrap(), max_2.unwrap());

        const VECSIZE_3: usize = 141072;
        let mut v3 = Vec::<i32>::with_capacity(VECSIZE_3);
        for _ in 0..VECSIZE_3 {
            v3.push(rng.random());
        }
        println!("v3 size: {}", v3.len());
        let max_1 = find_largest_num1(&v3);
        let max_2 = find_largest_num2(&v3);
        println!("Max 1: {}", max_1.unwrap());
        println!("Max 2: {}", max_2.unwrap());
        assert_eq!(max_1.unwrap(), max_2.unwrap());
    }
}
