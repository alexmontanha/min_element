fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    let min = min_element(&numbers);
    println!("min: {:?}", min);
}

fn min_element<T: PartialOrd>(list: &[T]) -> Option<&T> {
    let mut min = list.get(0)?;
    for item in list {
        if item < min {
            min = item;
        }
    }
    Some(min)
}
