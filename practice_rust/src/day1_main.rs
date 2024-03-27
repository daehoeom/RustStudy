/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// functions, code blocks
// /summary/

// fn print_number(one: i32, two: i32) -> i32 {
//     let multiplied = one * two;
//     println!("{multiplied}");

//     let multiplied_by_ten = {
//         let first_number = 10;
//         first_number * one * two
//     };

//     multiplied_by_ten
// }

// fn main() -> () {
//     let my_number = print_number(9, 8);
//     println!("{my_number}");
// }

/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// semicolon, unit type
// /summary/

// // () - empty tuple, unit type (void)
// // expression-based language
// fn empty_tuple() -> () {
// }
// //main 함수는 반드시 void로만 return
// // Display {}
// // Debug print
// fn main() -> () {
//     let tuple = empty_tuple();
//     //println!("{}", tuple);  //tuple은 void 형태이기 때문에 에러
//     println!("{:?}", tuple);  //debug print
// }


/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// println!-2
// /summary/
// fn main() {
//     let my_city = "Seoul";
//     let year = 2002;
//     let population = 9_987_987;
//     println!("The city of {} in {} had a population of {}", my_city, year, population);
//     println!("The city of {my_city} in {year} had a population of {population}");   //string interpolation
//     println!("The city of {city} in {year} had a population of {population}", 
//         city = my_city,
//         year = year,
//         population = population);
//     println!("The city of {0} in {1} had a population of {2}. I love {0}!", my_city, year, population);
//     //println!("The city of {my_city} in {year + 1} had a population of {population}");   //expression    //아직 지원하지 않음
// }


/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// println! - 1
// /summary/
//macro = function that writes code

// fn give_age() -> i32{   // int32 정수 반환 메서드
//     return 42;  //또는 그냥 42로 넣어도 된다.
// }

// fn give_other_age() -> i32 { 42 }

// fn main() {
//     let my_name = "David";
//     let my_age = 42;
//     println!("My name is {}");  // ! 에러 변수명 넣는 곳에 데이터가 없음
//     println!("My name is {} and my age is {}", my_name, my_age);
// }


/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// float 데이터 타입
// /summary/
// fn main() {
//     // type inference
//     let my_number : u8 = 9; // same as let my_number = 9_u8;
//     let other_number = 100_000_000u64;
//     let my_float = 9.; //f32(4byte) / f64 (8 byte) 기본형 float는 f64

//     println!("{}", my_float as i32 + other_number);   //연산 시 소수점 자름
// }