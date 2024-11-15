fn main() {
    let val1: i32 = 45;
    let val2: i32 = 55;

    let val_sum: i32 = get_sum(val1, val2);

    println!("The sum of val1:{} and val2:{} is {}", val1, val2, val_sum);
}

fn get_sum(val1: i32, val2: i32) -> i32 {
    return val1 + val2;
}
