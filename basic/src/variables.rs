
/// .
pub fn main() {
    // mutable variable without type
    let mut x = 5;
    println!("{}", x);
    x = 7;
    println!("{}", x);
    // shadow variable x with string type
    let mut x: &str = "asdf";
    println!("{}", x);
    x = "qwer";
    println!("{}", x);

    // const require type and should be UPPER_CASE
    // rust allows underscores in numbers for readability
    const Y_COUNT: u32 = 100_000;
    println!("{}", Y_COUNT);

    // data types
    // integers https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-types
    let x1: u8 = 255; // 0 to 255
    let x2: i8 = -128; // -128 to 127
    let x3: u16 = 65535; // 0 to 65535
    let x4: i16 = -32768; // -32768 to 32767
    let x5: u32 = 4294967295; // 0 to 4294967295
    let x6: i32 = -2147483648; // -2147483648 to 2147483647
    let x7: u64 = 18446744073709551615; // 0 to 18446744073709551615
    let x8: i64 = -9223372036854775808; // -9223372036854775808 to 9223372036854775807
    let x9: u128 = 340282366920938463463374607431768211455; // 0 to 340282366920938463463374607431768211455
    let x10: i128 = -170141183460469231731687303715884105728; // -170141183460469231731687303715884105728 to 170141183460469231731687303715884105727
    let x11: usize = 18446744073709551615; // 0 to 18446744073709551615
    let x12: isize = -9223372036854775808; // -9223372036854775808 to 9223372036854775807

    // floats https://doc.rust-lang.org/book/ch03-02-data-types.html#floating-point-types
    let f1: f64 = 2.0; // f64
    let f2 = 2.0f32; // f32

    // let sum1 = f1 + f2; // type mismatch
    let sum2 = f1 + f2 as f64; // ok
    let sum3 = 5 + 10;

    // let diff1 = f1 - f2; // type mismatch
    let diff2 = 5 - 10;

    let multi1 = 4 * 5;
    let multi2 = 4.1 * 5.1;

    let div1 = 4 / 5;
    let div2 = 4.1 / 5.1;

    let reminder1 = 4 % 5;
    let reminder2 = 4.1 % 5.1;

    // bool https://doc.rust-lang.org/book/ch03-02-data-types.html#the-boolean-type
    let b1 = true;
    let b2 = false;

    // chars https://doc.rust-lang.org/book/ch03-02-data-types.html#the-character-type
    let c1 = 'a';
    let c2 = '😻';
    let c3 = '🤣';
    let c4 = '👍';
    let c5 = '👎';

    // compound types, tuple
    let tuple1: (&str, i32, f64) = ("hello", 5_000_000, 1.2);
    let (name, population, gdp) = tuple1;
    let uk = tuple1.1;

    // array 
    let error_codes = [404, 500, 200];
    let not_found = error_codes[0];
    // let out_of_bounds = error_codes[4]; // arrays are fixed length

    let byte = [0; 8];
}