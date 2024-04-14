/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// Result
// /summary/

// Option - Maybe there, maybe not
// Result - May not work

// enum Option<T> { 
//     None,
//     Some(T)
// }

// enum Result<T, E> { 
//     Ok(T),
//     Err(E),
// }

fn check_error(input: i32) -> Result<(), ()> {
    if input % 2 == 0 {
        Ok(())
    } else {
        Err(())
    }
}

// Option
// .is_some()
// .is_none()

// Result
// .is_ok()
// .is_err()
// None.unwrap -> panic
// Err.unwrap -> panic

fn main() {
    //let variable = check_error();
    // if check_error(5).is_ok() {
    //     println!("It's okay, guys!");
    // } else {
    //     println!("It's an error, guys!");
    // }

    // match check_error(5) {
    //     Ok(_) => println!("Okay guys"),
    //     Err(_) => println!("It's an error"),
    // }

    check_error(5).unwrap()
}



/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// More Option
// /summary/

// fn take_fifth(value: Vec<i32>) -> Option<i32> {
//     if value.len() < 5 { 
//        None
//     } else {
//         Some(value[4])
//     }
// }


// fn main() {
//     let new_vec = vec![1, 2];
//     let index = take_fifth(new_vec);    // Option<i32>

//     // .unwrap() - take out what is inside (unsafe)
//     //println!("{}", index.unwrap());

//     // match index {
//     //     Some(number) => println!("I got a number: {}", number),
//     //     None => println!("There was nothing inside"),
//     // }

//     // Some(number)
//     // if index.is_some() {    //bool
//     //     //Option<i32>
//     //     println!("I got a number: {}", index.unwrap());
//     // }

//     // .expect
//     index.expect("Needed at least five items - make sure Vec has at least five");
// }




/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// Option
// /summary/

// pub enum Option<T> {
//     None,
//     Some(T),
// }


// fn take_fifth(value: Vec<i32>) -> i32 {  //문제가 발생할 수 있음
//     value[4]
// }

// fn take_fifth(value: Vec<i32>) -> Option<i32> {
//     if value.len() < 5 { 
//        None
//     } else {
//         Some(value[4])
//     }
// }

// // wrap in an Option

// fn main() {
//     let new_vec = vec![1, 2, 4, 7, 8, 10, 10];
//     let index = take_fifth(new_vec);
//     println!("{:?}", index);
// }
