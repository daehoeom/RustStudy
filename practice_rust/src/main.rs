/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// references in functinos
// /summary/

// move semantics
fn print_country(country_name: String) -> String {
    println!("My country is {}", country_name);
    country_name
}

fn print_country_ref(country_name: &String) {
    println!("My country is {}", country_name);
}

fn main() {
    let mut country = "대한민국".to_string();
    country = print_country(country);
    country = print_country(country);
    country = print_country(country);
}


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


/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// String Methods
// /summary/
// fn main() {
//     // String
//     // .capacity
//     // .push
//     // .push_str
//     // .pop
//     // with_capacity

//     let mut my_name = "David".to_string();
//     my_name.push('!');  //add char
//     my_name.push_str(" and I live in Seoul");   //add string
//     println!("My name is {my_name}");

//     // allocation or reallocation
//     let mut my_other_name = "David".to_string();
//     println!("Length is {}, Capacity is : {}", my_other_name.len(), my_other_name.capacity());
//     my_other_name.push('!');  //add char
//     println!("Length is {}, Capacity is : {}", my_other_name.len(), my_other_name.capacity());
//     my_other_name.push_str(" and I live in Seoul");   //add string
//     println!("Length is {}, Capacity is : {}", my_other_name.len(), my_other_name.capacity());
//     println!("My name is {my_other_name}");

//     let mut pre_allocate_my_name = String::with_capacity(0);
// }


/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// String and &str
// /summary/
// fn main() {
//     // String = Sized type 
//     // &str = dynamic type

//     //주로 String 아니면 &str (ref str), "string slice"
//     //growable string (길이 조절), owned type
//     let my_name = "David";  //&str
//     let my_name = "David".to_string();
//     let other_name = String::from("David2");
//     //growable + shrinkable
//     let mut my_other_name = "David3".to_string();
//     my_other_name.push('!');
//     println!("{}", my_other_name);
// }


/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// fancy printing
// /summary/
// fn main() {
//     print!("This\n");
//     print!("This");
//     print!(r#"c:\thisdrive\new_drive"#);    //raw text
//     println!("Let me tell you
// 어떤 이야기를   //공간이 있으면 그만큼 띄어쓰기 함
// 봅시다");
    
//     let my_variable = &9;
//     println!("{:p}", my_variable);      //포인터 메모리 주소 출력

//     let my_variable = 9000;
//     println!("{:X}", my_variable);
//     println!("{:b}", my_variable);      

//     let title = "TODAY'S NEWS";
//     println!("{:-^30}", title);     // no variable name, pad with -, put in centre, 30 characters long
//     let bar = "|";
//     println!("{: <15}{: >15}", bar, bar);       // no variable name, pad with space, 15 characters each, one to
//     let a = "SEOUL";
//     let b = "TOKYO";
//     println!("{city1:-<15}{city2:->15}", city1 = a, city2 = b); //variable names city1 and city2
//     //좀 더 자세한 내용은 https://doc.rust-lang.org/std/fmt 사이트 확인할 것
// }


/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// stack, heap, pointer
// /summary/
// fn main() {
//     let my_number = 15;                             // This is an i32
//     let single_reference = &my_number;             // Reference to my_number
//     let double_reference = &single_reference;     // This is a &&i32
//     let five_references = &&&&&my_number;      // This is a &&&&&i32        //굳이 여러 개 사용하는건 의미 없음
// }


/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// mutability and shadowing
// /summary/

// immutable by default  //rust에서는 기본은 불변변수
// shadowing 같은 이름을 다시 쓰는 것
// fn double(input: i32) -> i32 {
//     input * 2
// }

// fn triple(input: i32) -> i32{
//     input * 3
// }
// fn main() {
//     // let mut my_number = 10; // 재할당이 가능한 변수
//     // my_number = 9;

//     // let my_variable = 10;   
//     // let my_variable = "My variable";    // 여기서 my_variable로 덮여씌움
//     // println!("{}", my_variable);
    
//     // let x = 9;
//     // let x = double(x);
//     // let x = triple(x);

//     // println!("{}", x);

//     let my_variable = 9;
//     println!("{my_variable}");              // 9
//     {
//         let my_variable = "Some string";
//         println!("{my_variable}");          //Some string
//     }
//     println!("{my_variable}");              // 9
// }


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