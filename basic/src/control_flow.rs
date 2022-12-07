pub fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    loop {
        println!("loop!");
        break;
    }

    let mut counter = 0;
    let result = loop {
        println!("loop!");
        counter += 1;
        if counter == 10 {
            // break; // do not return anything
            break counter; // return counter from loop
        }
    };
    println!("Result: {}", result);

    let mut counter = 0;
    while counter == 2 {
        println!("{}!", number);
        counter += 1;
    }

    let array = [10, 20, 30, 40, 50];
    for element in array.iter() {
        println!("The value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
}
