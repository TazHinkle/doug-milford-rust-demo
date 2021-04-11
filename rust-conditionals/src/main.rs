#[allow(unused_variables)]
fn main() {
    let some_bool = true;
    let some_int = 30;
    let some_int2 = 50;

    if some_bool == true || some_int > 100 && some_int2 == 200 {
        println!("Hit If branch");
    } else if some_int == 30 {
        println!("Hit Else If branch");
    } else {
        println!("Hit Else branch");
    }

    match some_bool {
        true => {
            println!("Hit true branch");
        }
        false => {
            println!("Hit false branch");
        }
    }

    match some_int {
        0 => println!("Hit 0 branch"),
        1..=100 => println!("between 1 and 100 branch"),
        _ => println!("else branch"),
    }
}
