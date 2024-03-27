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