use std::fmt::Display;

fn main() {
    let result = search(&[1, 2, 3, 4], 8);
    let result_2 = binary_search(&[1, 2, 3, 4, 7, 10, 14, 15], 14);

    match result {
        Ok(value) => println!("{}", value),
        Err(error) => println!("{}", error),
    }

    match result_2 {
        Ok(value) => println!("{}", value),
        Err(error) => println!("value {} not found", error),
    }
}

fn search<T: PartialEq + Display>(arr: &[T], value: T) -> Result<usize, String> {
    for (index, element) in arr.iter().enumerate() {
        if *element == value {
            return Ok(index);
        }
    }

    return Err(format!("the value {} not found ", value));
}

fn binary_search(arr: &[usize], value: usize) -> Result<usize, String> {
    let mut lo = 0;
    let mut hi = arr.len();

    loop {
        let m = lo + (hi - lo) / 2;
        let v = arr[m];

        if v == value {
            return Ok(m);
        } else if value < v {
            hi = m;
        } else {
            lo = m + 1;
        }

        if hi < lo {
            break;
        };
    }

    return Err(format!("{}", value));
}
