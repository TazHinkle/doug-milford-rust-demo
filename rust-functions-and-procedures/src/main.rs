#[allow(unused_variables)]

fn main() {
    let returned_data = some_function(2.2, 50);
    println!("returned_data is {}", returned_data);
    some_procedure(2.3, 1);
    some_str_procedure("a string");

    let string_slice_var: &str = "Howdy";
    some_str_procedure(string_slice_var);

    let string_var = String::from("I'm a REAL String");
    some_str_procedure(&string_var);

    some_string_procedure(string_var);
    // using the string_var a second time runs into "borrowing" problems
}

fn some_function(param_a: f32, param_b: i128) -> f32 {
    println!("I'm in some_function!");
    if param_a < 100. {
        // This could be problematic math, but it will compile (i128 is much)
        // bigger than f32.
        let return_var = 10.1 * param_a + param_b as f32;
        // any line without a semi-colon is assumed to be a return value
        return_var
    } else {
        -1.
    }
}

fn some_procedure(param_a: f32, param_b: i8) {
    println!("I'm in some_procedure with a {} b {}", param_a, param_b);
}

fn some_str_procedure(param: &str) {
    println!("I'm in some_str_procedure with param {}", param);
}

fn some_string_procedure(param: String) {
    println!("I'm in some_string_procedure with param {}", param);
}
