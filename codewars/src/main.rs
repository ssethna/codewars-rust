//use dialoguer::{theme::ColorfulTheme, Select};
use l6kyu::arrays::*;
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
        "square_sum is {}",
        create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0])
    );
    println!("boolean_to_string is {}", boolean_to_string(false));
    println!("solution is {}", solution("abc", "bc"));
    println!("{}", multi_table(5));
}
