// fn main() {
//     println!("Hello world!")
// }

// function with no `;` means return
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");

    // assign a value after some computation
    let something = {
        let x = 100;
        x + 2
    };

    println!("{something}");

    // conditional assignment
    let condition = false;
    let number = if condition { 5 } else { 6 };
    println!("{number}");

    // breaking out of loop with return value
    let mut counter=0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("{result}");


    let a = [1,2,3,4,5];
    for element in a {
        println!("the value is: {element}");
    }

    for element in (1..10).rev() {
        println!("{element}");
    }
}
