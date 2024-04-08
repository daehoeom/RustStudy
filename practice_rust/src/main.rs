/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// More generics
// /summary/

use std::fmt::Display;
use std::cmp::PartialOrd;

//fn compare_and_print<T: Display, U: Display + PartialOrd>(statement: T, num_1: U, num_2: U) {
fn compare_and_print<T, U>(statement: T, num_1: U, num_2: U) 
    where 
        T: Display,
        U: Display + PartialOrd,
{
    println!(
        "{}! Is {} greater than {}? {}",
        statement,
        num_1,
        num_2,
        num_1 > num_2
    );
}

fn main() {
    compare_and_print("Listen up!", 9, 8);
}



/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// Generics
// /summary/

// generics (Custom type) - concreate (ex. i32, String)
// "It's a little generic"

// very concreate
// fn print_and_give_item() -> i32 {
//     let number = 9;
//     println!("The number is: {}", number);
//     9
// }

// // fn give_thing<GenericType>(input: GenericType) -> GenericType {  // 
// //     input
// // }

// struct Book;

// use std::fmt::Display;
// //제약 조건
// fn give_thing<T: Display>(input: T) -> T {  // 
//     println!("{}", input);
//     input
// }

// fn main() {
//     let x = print_and_give_item();
//     let x2 = give_thing(String::from("Take this thing"));
//     let y = give_thing(9);
//     let z = give_thing(Book);
//     println!("{}", x2);
//     println!("{}", y);
// }



/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// Dereferencing and the dot operator
// /summary/

// references and the dot operator

// struct Item {
//     number: u8
// }

// // . dot operator
// impl Item {
//     fn compare_number(&self, other_number: u8) {
//         println!("Are they equal? {}", self.number == other_number);
//     }
// }

// // Deref *
// fn main() {
//     let item = Item {
//         number: 10
//     };

//     let reference_item = &item;   // &u8
//     let other_reference_item = &reference_item;

//     item.compare_number(10);
//     reference_item.compare_number(10);
//     other_reference_item.compare_number(10);

//     //println!("{}", reference == 10);
//     // let my_number = 10;     // i32
//     // let reference = &my_number; //&i32

//     // println!("Are they the same? {}", my_number == *reference);
// }