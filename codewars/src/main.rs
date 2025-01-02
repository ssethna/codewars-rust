//use dialoguer::{theme::ColorfulTheme, Select};
use l8kyu::arrays::*;
use l8kyu::fundamentals::*;
use l8kyu::math::*;

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
}
