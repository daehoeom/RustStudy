/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// slices
// /summary/

// Slices
// Vecs (Vector)
// &str

// dynamically sized type
fn main() {
    let seasons = ["봄", "여름", "가을", "겨울", "봄", "여름", "가을", "겨울"];
    println!("{:?}", &seasons[0..2]);   // up to not including
    println!("{:?}", &seasons[0..=2]);   // up to and including
    println!("{:?}", &seasons[3..]);   // up to and including
    println!("{:?}", &seasons[..=4]);   // up to and including
}


/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// array
// /summary/

// Collection types
// Array

// &str
// fn main() {
//     let array = ["One", "Two"];     // [&str; 2]
//     //let array2 = ["One", "Two", "Five"];    // [&str; 3]
//     let array2 = ["One", "Twoo"];     // [&str; 2]
//     println!("Is array the same as array2? {}", array == array2);   
    
//     let intArray = [0; 640];
//     println!("{:?}", intArray);

//     let weekArray = ["1월", "2월"]; //indexing
//     println!("{:?}", array.get(3));   //first
//     /// Some None options
// }


/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// uninitilized variables and for loops
// /summary/

// uninitialized variable
// control flow

// fn loop_then_return(mut counter: i32) -> i32 {
//     loop {
//         counter += 1;
//         if counter % 50 == 0 {      // 102 / 50
//             break;
//         }
//     }

//     counter
// }

// fn main() {
//     // let my_number: u8;

//     // {
//     //     my_number = 9;
//     // }

//     // let my_number = {
//     //     //복잡한 것들
//     //     let x = 9;

//     //     x + 9
//     // };

//     let my_number: i32;
//     {
//         let x = loop_then_return(43);
//         my_number = x;
//     }
//     println!("{}", my_number);  // 사용 불가능, possibly uninitilized = maybe doesn't have a value yet
// }


/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// copy and clone
// /summary/

// It's trivial to copy the bytes

// Ownership and copy types
// fn prints_number(number: i32) {
//     println!("{}", number);
// }

// fn prints_string(input: String) {
//     println!("{}", input);
// }

// // copy - copy types
// // clone = non-copy types
// fn main() {
//     let my_number = 8;
//     prints_number(my_number);
//     prints_number(my_number);

//     let my_country = "Austria".to_string();
//     prints_string(my_country.clone());  //소유권을 넘기지 않기에 오류가 발생하지 않음 나쁘지 않지만 굳이?
//     prints_string(my_country);
// }


/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// mutable references and mut in functions
// /summary/

// fn add_is_great(country_name: &mut String) {
//     country_name.push_str(" is great!");
//     println!("Now it says: {}", country_name);
// }

// fn add_is_great_not_ref(mut country_name: String) -> String { //take by value, declare as mutable
//     country_name.push_str(" is great!");
//     println!("Now it says: {}", country_name);
//     country_name
// }

// fn main() {
//     let mut my_country = "Canada".to_string();
    
//     //add_is_great(my_country);     // by value
//     //add_is_great(&mut my_country);     // by mutable reference    
//     add_is_great(&mut my_country);
//     add_is_great(&mut my_country);

//     let my_country_not_ref = "Korea".to_string();
//     add_is_great(my_country);
// }