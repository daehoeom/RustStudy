// /////////////////////////////////////////////////////////////////////////////////////
// // /summary/
// // Enum 2
// // /summary/

enum Star {
    BrownDwarf = 10,
    RedDwarf = 50,
    YellowStar = 100,
    RedGiant = 1000,
    DeadStar,
}

enum Number {
    U32(u32),
    I32(i32),
}

fn get_number(input: i32) -> Number {
    let number = match input.is_positive() {
        true => Number::U32(input as u32),
        false => Number::I32(input)
    };

    number
}

fn main() {
    // use Star::*;
    // let starvec = vec![BrownDwarf, RedDwarf, YellowStar, RedGiant, DeadStar];

    // for star in starvec {
    //     match star as u32 {
    //         size if size <= 80 => println!("Not the biggest star: {}", size),
    //         size if size >= 80 => println!("Pretty big star: {}", size),
    //         _ => println!("Some other star")
    //     }
    // }

    // println!("What about DeadStar? It is: {}", DeadStar as u32);

    let my_vec = vec![get_number(-800), get_number(8)];     // Vec<Number>

    for item in my_vec {
        match item {
            Number::U32(number) => println!("It's a u32 with val"),
            Number::I32(number) => println!("It's a i32 with val"),
        }
    }
}



// /////////////////////////////////////////////////////////////////////////////////////
// // /summary/
// // Enum 2
// // /summary/

// enum Mood {
//     Happy,
//     Sleepy,
//     NotBad,
//     Angry
// }

// enum Season {
//     Spring,
//     Summer,
//     Autumn,
//     Winter,
// }

// fn match_mood(mood: &Mood) -> i32 {
//     use Mood::*;
//     // let happiness_level = match mood {
//     //     // Mood::Happy => 10,
//     //     // Mood::Sleepy => 6,
//     //     // Mood::NotBad => 7,
//     //     // Mood::Angry => 2
//     //     Happy => 10,
//     //     Sleepy => 6,
//     //     NotBad => 7,
//     //     Angry => 2
//     // };

//     // happiness_level

//     match mood {
//         // Mood::Happy => 10,
//         // Mood::Sleepy => 6,
//         // Mood::NotBad => 7,
//         // Mood::Angry => 2
//         Happy => 10,
//         Sleepy => 6,
//         NotBad => 7,
//         Angry => 2
//     }
// }

// fn main() {
//     // let my_mood = Mood::NotBad;
//     // let happiness_level = match_mood(&my_mood);
//     // println!("Out of 1 to 10, happiness is {}", happiness_level);
//     use Season::*;
//     let four_seasons = vec![Spring, Summer, Autumn, Winter]; //Vec<Season>
//     for season in four_seasons {
//         println!("The number is: {}", season as u32);
//     }
// }


/////////////////////////////////////////////////////////////////////////////////////
// /summary/
// Enum
// /summary/

// struct = and
// enum(enumeration) = or

// enum ThingsInTheSky {
//     Sun,    // 0
//     Stars,  // 1
//     Other  // 2
// }

// fn create_skystate(time: i32) -> ThingsInTheSky {
//     match time {
//         6..=18 => ThingsInTheSky::Sun,
//         _ => ThingsInTheSky::Stars,
//     }
// }

// fn check_skystate(state: &ThingsInTheSky) {
//     match state {
//         ThingsInTheSky::Sun => println!("I can see the sun"),
//         ThingsInTheSky::Stars => println!("I can see the stars"),
//         _ => println!("I can't see anything"),
//     }
// }

// fn main() {
//     // let time = 8;
//     // let sky_state = create_skystate(time);
//     // check_skystate(&sky_state);

//     check_skystate(&create_skystate(20));
// }