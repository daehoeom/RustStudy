/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// impl blocks
// /summary/
//for number in numbers

#[derive(Debug)]
struct Animal {
    age: u8,
    animal_type : AnimalType
}

#[derive(Debug)]
enum AnimalType {
    Cat,
    Dog,
}

impl Animal {
    fn new(age: u8) -> Self {  // Self = Animal, function signature

        Self {
            age,
            animal_type: AnimalType::Cat,
        }
    }
}

fn main() {
    // let my_animal = Animal {
    //     age: 10,
    //     animal_type: AnimalType::Cat,
    // };
    let my_animal = Animal::new(10);  // associated function

    println!("I made a: {:?}", my_animal);
}


/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// More Loop
// /summary/
//for number in numbers
// fn main() {
//     // let mut counter = 0;

//     // while counter != 5 {
//     //     counter += 1;
//     //     println!("The counter is now: {}", counter);
//     // }

//     for number in 0..3 {    // execlusive Range
//         println!("The number is {}", number);
//     }

//     for number in 0..=3 {   // inclusive Range
//         println!("The number is {}", number);
//     }

//     for _ in 0..=3 {
//         println!("I don't need a number");
//     }

//     let mut counter = 5;
//     let my_number = loop {
//         counter += 1;
//         if counter % 53 == 3 {
//             break counter;
//         }
//     };

//     println!("my_number is now: {}", my_number);
// }



/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// Loop
// /summary/

// loops

//for number in numbers
// fn main() {

//     let mut counter = 0;
//     let mut counter2 = 0;

//     'first_loop: loop {
//         counter += 1;
//         println!("The counter is now: {}", counter);
//         if counter > 9 {
//             println!("Now entering the second loop");

//             'second_loop: loop {
//                 println!("The second counter is: {}", counter2);
//                 counter2 += 1;
//                 if counter2 == 3 {
//                     break 'first_loop;
//                 }
//             }
//         }
//     }
// }