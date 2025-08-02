fn main() {
    let mut number:u8 = 12;
    println!("the number is {}", number);
    const IS_NUMBER:i8=-91;
    println!("the number is {}", IS_NUMBER);


    // mutable and immutable

    number = 90;

    println!("the number is {}", number);


    // strings

    // &str nd String

    // let str_is:String = "hello";

    let str_is:&str = "hello";

    println!("the string is {}", str_is);


    let mut new_string:String = String::from("this is a string of type Sting");

    println!("new type of string is {}", new_string);


    // the difference between &str and String


    new_string.push_str(" wowowwowowowowwo");

    // str_is.push_str(" wowowwowowowowwo");

    println!("new type of string is {}", new_string);

    // println!("new type of string is {}", str_is);


// also String is stored in heap and we can have dynamic length
// &str not stored in heap, stored in stack or some special memory read only has fixed length



// tuples

    // like objects maybe 
    let user_info:(&str, u8) = ("rafi", 90);

    println!("first tuple value is {}, {}", user_info.0, user_info.1);

    // we can destructure tuples

    let (name, age) = user_info;

    println!("first tuple value is {}, {}", name, age)



}
