fn main() {
    // This is a part in rust about numbers
    let x_i8: i8 = 5;
    let x_i32: i32 = 243;
    let x_u32: u32 = 32324;
    let y_f32: f32 = 100.23;
    let y_f64: f64 = 1000.32;

    println!(
        "This is i8 variable, which contains numbers from -2**7 to 2**7-1: {}",
        x_i8
    );
    println!(
        "This is i32 variable, its default and can contain numbers from -2**31 to 2**31-1: {}",
        x_i32
    );
    println!(
        "This is u32 variable, it only contains +ve values from 0 to 2**32 -1: {}",
        x_u32
    );
    println!(
        "This is f32 variable, it is a floating point number and contains -ve and +ve values: {}",
        y_f32
    );
    println!("This is f64 variable, it is the default one: {}", y_f64);
    println!("The numbers can be i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32 and f64.");

    //This part is about booleans.

    let is_male: bool = true;
    let is_above_18: bool = true;

    if is_male {
        println!("You are a male.");
    } else {
        println!("You are not a male.");
    }

    if is_male && is_above_18 {
        println!("You are a legal male.");
    }
}
