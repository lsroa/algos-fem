use std::fmt::Display;

fn main() {
    let result = search(&[1, 2, 3, 4], 8);

    match result {
        Ok(value) => println!("{}", value),
        Err(error) => println!("{} in:", error),
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
