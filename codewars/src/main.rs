//use dialoguer::{theme::ColorfulTheme, Select};
use l5kyu::fundamentals::*;
use l6kyu::algorithms::*;
use l6kyu::arrays::*;
use l7kyu::fundamentals::*;
use l7kyu::strings::*;
use l8kyu::arrays::*;
use l8kyu::fundamentals::*;
use l8kyu::math::*;
use l8kyu::strings::*;

fn main() {
    // let options = vec!["Option 1", "Option 2", "Option 3", "Option 4"];
    // let selection = Select::with_theme(&ColorfulTheme::default())
    //     .with_prompt("Select an option")
    //     .default(0)
    //     .items(&options)
    //     .interact()
    //     .unwrap();
    // println!("You selected: {}", options[selection]);

    println!("isLove {}", is_love(5, 7));
    println!("the summation is {}", summation(8));
    println!("square_sum is {}", square_sum(vec![5, 3, 4]));
    println!(
        "create_phone_number is {}",
        create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0])
    );
    println!("boolean_to_string is {}", boolean_to_string(false));
    println!("solution is {}", solution("abc", "bc"));
    println!("{}", multi_table(5));
    println!("Encipher 'ABCD' is '{}'", encipher("ABCD"));
    println!("Decipher 'GBCE' is '{}'", decipher("GBCE"));
    println!(
        "Encipher_digital is '{:?}'",
        digital_encipher("Scout".to_string(), 1939)
    );
    println!(
        "crack is {:?}",
        crack("86aa400b65433b608a9db30070ec60cd".to_string())
    );
    println!("Derive is {}", derive(7, 8));
    println!("{}", code("Alan Turing"));
    println!(
        "Decode is {}",
        decode("\x0bi A\n\x0bnTl\n\x0bgua\n\x0b\x0brn")
    );
    println!(
        "Alternate case for 'Hello World' is '{}'",
        to_alternating_case("Hello World")
    );
    println!(
        "No spaces is {}",
        no_space(String::from("8 j 8   mBliB8g  imjB8B8  jl  B"))
    );
    println!("Even is multiplied by 8 so 4 is {}", simple_multiplication(4));
    println!("Odd is multiplied by 9 so 5 is {}", simple_multiplication(5));
}
