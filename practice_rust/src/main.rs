/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// Struct size
// /summary/

use std::mem::size_of_val;

struct Numbers { 
    one: u8,
    two: u8,
    three: u8,
    four: u8
}

#[derive(Debug)]
struct Country {
    population: u32,
    capital: String,
    leader_name: String,
    all_populations: [u32; 5500]
}
fn main() {
    let population = 35_000_000;
    let capital = "Ottawa".to_string();
    let leader_name = "Justin Trudeau".to_string();

    //Clippy
    let my_country = Country {
        population,
        capital,
        leader_name,
        all_populations: [500; 5500]
    };

    println!("The country is: {:#?}", my_country);
    println!("Country is {} bytes in size", size_of_val(&my_country));

    let numbers = Numbers {
        one: 8,
        two: 19,
        three: 20,
        four: 30
    };

    println!("Size is: {}", size_of_val(&numbers));
}



/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// Structs
// /summary/

//struct - and
//enum = or

// unit struct
// struct FileDirectory;

// // trait
// // tuple struct
// #[derive(Debug)]   // attribute
// struct Colour(u8, u8, u8);

// // named struct
// #[derive(Debug)]
// struct Country {
//     population: u32,
//     capital: String,
//     leader_name: String,
// }

// fn takes_file_directory(input: FileDirectory) {
//     println!("I got a file directory");
// }
// fn main() {
//     //let x = FileDirectory;
//     //takes_file_directory(x);
//     //std::mem::size_of_val(&x);

//     // let my_colour = Colour(20, 50, 100);
//     // println!("The second colour is {:?}", my_colour.1);

//     let canada = Country {
//         population: 35_000_000,
//         capital: "Ottawa".to_string(),
//         leader_name: "Justin Trudeau".to_string(),  
//     };

//     //println!("The population is: {:?}\nThe capital is: {:?}", canada.population, canada.capital);
//     println!("The country is: {:#?}", canada);
// }


/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// More match statements
// /summary/

//rgb

// fn match_number(input: i32) {
//     match input {
//         number => println!("It's the number {}", number),
//         number @ 0..= 10 => println!("It's between 0 and 10. It's the number {}", number),
//         _ => println!("It's greater than ten"),
//     }
// }

// fn match_colours(rgb: (u32, u32, u32)) {
//     match rgb {
//         (r, _, _) if r < 10 => println!("Not much red"),
//         (_, g, _) if g < 10 => println!("Not much green"),
//         (_, _, b) if b < 10 => println!("Not much blue"),
//         _ => println!("Every colour has at least 10"),
//     }
// }
// fn main() {
//     let first = (200, 0 , 0);
//     let second = (50, 50, 50);
//     let third = (200, 50, 0);

//     match_colours(first);
//     match_colours(second);
//     match_colours(third);

//     // let my_number = 10;
//     // let some_variable = match my_number {
//     //     10 => 8,
//     //     _ => "Not ten", //다른 데이터 타입은 안됨
//     // };

//     // let other_variable = if my_number == 10 { 8 } else { "Something else" }; //다른 데이터 타입은 안됨

//     match_number(10);
//     match_number(100);
// }