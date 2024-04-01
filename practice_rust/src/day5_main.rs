/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// Match statements
// /summary/
// fn main() {
//     let sky = "cloudy";     //&str
//     let temperature = "warm";

//     match (sky, temperature) {
//         ("cloudy", "cold") => println!("It's not very nice today"),
//         ("clear", "warm") => println!("It's a nice day"),
//         ("cloudy", _) => println!("Cloudy and something else"),
//         _ => println!("Not sure what the weather is."),
//     }

//     let children = 5;
//     let married = true;

//     match (children, married) {
//         (children, married) if married == false => println!("Not married with {} children", children),
//         (c, m) if c == 0 && m => println!("Married but with no children"),
//         (_, _) => println!("Some other type of marriage and children combination"),
//     }
// }


/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// Control flow
// /summary/
// fn main() {
//     let my_number = 5;
//     if my_number == 7 {
//         println!("It's seven");
//     } else if my_number == 6 {
//         println!("It's six");
//     } else {
//         println!("It's a different number");
//     }

//     // expression-based laguage
//     // match 
//     let my_number2: u8 = 5;
    
//     let second_number = match my_number2 {  // switch
//         0 => 23, //println!("It's a zero"),
//         1 => 65, //println!("It's a one"),
//         _ => 0, //println!("It's a different number")    // _ 
//     };

//     println!("The second number is: {}", second_number);
// }


/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// Tuples and destructuring
// /summary/
// Vec<(String, i32)>
// Destructuring
// Structure
// fn main() {
//     let my_tuple = (8, "Dave MacLeod", vec![8, 9, 10]);
//     let my_variable = vec![("Hey", 9), ("Hellow there", 12312)];
//     println!("{:?}", my_tuple);

//     // let str_vec = vec!["one", "two", "three"];
//     // let (a, b, c) = str_vec;

//     // let str_tuple = ("one", "two", "trhee");
//     // let (a1, b1, c1) = str_tuple;

//     let str_array = ["one", "two", "three"];

//     let [a, b, c] = str_array;

//     println!("Item a is: {}", a);
// }


/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// From and Into
// /summary/

// trait
// This type implements (trait name)
//
// From, Into
// fn main() {
//     let my_name = String::from("Dave MacLeod");
//     let my_city: String = "Seoul".into();

//     println!("{}", my_city);
// }


/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// Vecs
// /summary/

// Vec<String>
// Vec<u8>
// T = some type
// generics

// fn main() {
//     let my_string = String::new();
//     let name1 = String::from("Windy");
//     let name2 = String::from("Gomesy");

//     let mut my_vec = Vec::new();
//     println!("Space for my_vec : {:?}", my_vec.capacity());     // 0
//     my_vec.push(name1.clone());
//     println!("Space for my_vec : {:?}", my_vec.capacity());     // reallocation
//     my_vec.push(name2.clone());
//     println!("Space for my_vec : {:?}", my_vec.capacity());     // reallocation
//     println!("My cats are {:?}", my_vec);

//     let my_vec_2 = vec![name1.clone(), name2.clone()];
//     println!("My cats are {:?}", my_vec);
// }