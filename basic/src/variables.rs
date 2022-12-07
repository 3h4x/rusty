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
    let _x1: u8 = 255; // 0 to 255
    let _x2: i8 = -128; // -128 to 127
    let _x3: u16 = 65535; // 0 to 65535
    let _x4: i16 = -32768; // -32768 to 32767
    let _x5: u32 = 4294967295; // 0 to 4294967295
    let _x6: i32 = -2147483648; // -2147483648 to 2147483647
    let _x7: u64 = 18446744073709551615; // 0 to 18446744073709551615
    let _x8: i64 = -9223372036854775808; // -9223372036854775808 to 9223372036854775807
    let _x9: u128 = 340282366920938463463374607431768211455; // 0 to 340282366920938463463374607431768211455
    let _x10: i128 = -170141183460469231731687303715884105728; // -170141183460469231731687303715884105728 to 170141183460469231731687303715884105727
    let _x11: usize = 18446744073709551615; // 0 to 18446744073709551615
    let _x12: isize = -9223372036854775808; // -9223372036854775808 to 9223372036854775807

    // floats https://doc.rust-lang.org/book/ch03-02-data-types.html#floating-point-types
    let f1: f64 = 2.0; // f64
    let f2 = 2.0f32; // f32

    // let sum1 = f1 + f2; // type mismatch
    let _sum2 = f1 + f2 as f64; // ok
    let _sum3 = 5 + 10;

    // let diff1 = f1 - f2; // type mismatch
    let _diff2 = 5 - 10;

    let _multi1 = 4 * 5;
    let _multi2 = 4.1 * 5.1;

    let _div1 = 4 / 5;
    let _div2 = 4.1 / 5.1;

    let _reminder1 = 4 % 5;
    let _reminder2 = 4.1 % 5.1;

    // bool https://doc.rust-lang.org/book/ch03-02-data-types.html#the-boolean-type
    let _b1 = true;
    let _b2 = false;

    // chars https://doc.rust-lang.org/book/ch03-02-data-types.html#the-character-type
    let _c1 = 'a';
    let _c2 = '😻';
    let _c3 = '🤣';
    let _c4 = '👍';
    let _c5 = '👎';

    // compound types, tuple
    let tuple1: (&str, i32, f64) = ("hello", 5_000_000, 1.2);
    let (_name, _population, _gdp) = tuple1;
    let _uk = tuple1.1;

    // array
    let error_codes = [404, 500, 200];
    let _not_found = error_codes[0];
    // let out_of_bounds = error_codes[4]; // arrays are fixed length

    let _byte = [0; 8];
}
