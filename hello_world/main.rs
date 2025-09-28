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
        x+2
    };

    println!("{something}");
}