

/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// references in functinos
// /summary/

// move semantics
// fn print_country(country_name: String) -> String {
//     println!("My country is {}", country_name);
//     country_name
// }

// fn print_country_ref(country_name: &String) {
//     println!("My country is {}", country_name);
// }

// fn main() {
//     let mut country = "대한민국".to_string();
//     country = print_country(country);
//     country = print_country(country);
//     country = print_country(country);

//     print_country_ref(country);
// }


/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// references and shadowing
// /summary/
// fn main() {
//     //case 1.
//     // let mut number = 10;
//     // let number_ref = &number;
//     // let number_change = &mut number;
//     // *number_change += 10;
//     // println!("{}", number_ref);

//     // case 2.
//     // let mut number = 10;
//     // let number_change = &mut number;
//     // *number_change += 10;
//     // let number_ref = &number;
//     // println!("{}", number_ref);

//     // shadowing
//     let country = "대한민국";
//     let country_ref = &country;
//     let country = 8;    // unsafe
//     println!("{}, {}", country_ref, country);
// }


/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// mutable references
// /summary/

// & immutable reference    / shared reference
// &mut mutable reference   / unique reference
// fn main() {
//     let mut my_number = 9;
//     let num_ref = &mut my_number;

//     //num_ref = 10;       //불가능
//     *num_ref = 10;          // C++ 포인터과 같은 개념
//     println!("Number is now {}", my_number);
// }



/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// returning a reference
// /summary/

// fn return_it() -> &'static String {
//     let country = String::from("대한민국");
//     &country // return &String      //안됨 lifetime이 스코프를 벗어나면 삭제되는데 삭제되는 변수를 반환하려함
// }

// // & = reference
// fn main() {
//     let my_country = return_it();
// }


/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// Const and Static
// /summary/

// 만약 소문자로 사용하고 싶다면 
// #[allow(non_upper_case_globals)]
// const HIGH_SCORE: i32 = 20; //global scope
// static LOW_SCORE: i32 = 0;  //같은 메모리를 공유함
// static mut MUTABLE_VARIABLE: i32 = 0;   // unsafe

// // lifetime


// fn print_high_score() {
//     println!("The high score is {}", HIGH_SCORE);
// }
// fn main() {
//     print_high_score();
//     let x = 8;  // 'let' binding: i32
//     unsafe { LOW_SCORE = 1; }
//     let my_name = "David";      //&'static str
// }