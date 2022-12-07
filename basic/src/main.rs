mod control_flow;
mod variables;

fn main() {
    variables::main();
    control_flow::main();

    function(1);
    function_that_returns_value(2);
}

fn function(x: u128) {
    println!("{}", x);
}

fn function_that_returns_value(x: u128) -> u128 {
    println!("{}", x);

    let y = x * 128;
    return y;
    // y
    // x * 128
}
