/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// More destructuring
// /summary/

struct Person {
    name: String,
    real_name: String,
    height: u8,
    happiness: bool,
}

#[derive(Debug)]
struct Person2 {
    name: String,
    height: u8,
}

impl Person2 {
    fn from_person(input: Person) -> Self {
        let Person { name, height , .. } = input;

        Self {
            name, 
            height,
        }
    }
}

fn main() {
    let papa_doc = Person {
        name: "Papa Doc".to_string(),
        real_name: "Clarence".to_string(),
        height: 170,
        happiness: false,
    };

    //copy
    // let Person { name, real_name, height, happiness } = papa_doc;
    // println!("They call him {} but his real name is {}. He is {} cm tall and is he happy? {}",
    //     name, real_name, height, happiness);

    let person2 = Person2::from_person(papa_doc);
    println!("Person2 type is :{:?}", person2);
}


/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// enum impl blocks
// /summary/

// #[derive(Debug)]
// struct Animal {
//     age: u8,
//     animal_type : AnimalType
// }

// #[derive(Debug)]
// enum AnimalType {
//     Cat(String),
//     Dog(String),
// }

// impl AnimalType {
//     fn print_name(&self) {
//         match self {
//             AnimalType::Cat(name) => println!("Animal type is cat and name is: {}", name),
//             AnimalType::Dog(name) => println!("Animal tpye is dog and name is: {}", name),
//         }
//     }

//     // fn check_type(&self) {
//     //     use AnimalType::*;
//     //     match self {
//     //         Cat => println!("Animal type is cat"),
//     //         Dog => println!("Animal tpye is dog"),
//     //     }
//     // }
// }

// impl Animal {
//     fn new(age: u8, animalType: AnimalType) -> Self {  // Self = Animal, function signature

//         Self {
//             age,
//             animal_type: animalType,
//         }
//     }

//     // fn change_to_dog(&mut self) {
//     //     self.animal_type = AnimalType::Dog;
//     //     println!("Changed to dog! No I am: {:?}", self);
//     // }

//     // fn change_to_cat(&mut self) {
//     //     self.animal_type = AnimalType::Cat;
//     //     println!("Changed to cat! No I am: {:?}", self);
//     // }
// }

// fn main() {
//     // use AnimalType::*;
//     // let my_cat = Animal::new(10, Cat);
//     // let my_dog = Animal::new(10, Dog);

//     // my_cat.animal_type.check_type();
//     let my_cat = Animal::new(10, AnimalType::Cat("Windy".to_string()));
//     my_cat.animal_type.print_name();
// }


/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// More impl blocks
// /summary/

// #[derive(Debug)]
// struct Animal {
//     age: u8,
//     animal_type : AnimalType
// }

// #[derive(Debug)]
// enum AnimalType {
//     Cat,
//     Dog,
// }

// // C# Partial와 같은 기능
// impl Animal {
//     fn new_old_cat() -> Self {
//         Self {
//             age: 15,
//             animal_type: AnimalType::Cat,
//         }
//     }
// }

// impl Animal {
//     fn new_cat(age: u8) -> Self {  // Self = Animal, function signature

//         Self {
//             age,
//             animal_type: AnimalType::Cat,
//         }
//     }

//     fn new_dog(age: u8) -> Self {
//         Self {
//             age,
//             animal_type: AnimalType::Dog,
//         }
//     }

//     fn print(&self) {
//         println!("I am a: {:?}", self);
//     }

//     fn change_to_dog(&mut self) {
//         self.animal_type = AnimalType::Dog;
//         println!("Changed to dog! No I am: {:?}", self);
//     }

//     fn change_to_cat(&mut self) {
//         self.animal_type = AnimalType::Cat;
//         println!("Changed to cat! No I am: {:?}", self);
//     }
// }

// fn main() {
//     let mut my_animal = Animal::new_dog(10);  
//     my_animal.print();                  // dot operator, syntactic sugar
//     //Animal::print(&my_animal);    // 
//     my_animal.change_to_cat();
//     my_animal.change_to_dog();

//     let my_old_cat = Animal::new_old_cat();
// }