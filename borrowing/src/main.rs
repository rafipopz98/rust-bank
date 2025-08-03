fn main(){
    let mut s1 = String::from("hello");
    // let length = get_len(&s1);
    // println!("length of {} is {}", s1,length);
   update_string(&mut s1);
    println!("{}",s1);
}

// fn get_len (s1:&String)->usize{
//     println!("{}",s1);
//     return s1.len();
// }

fn update_string (s1:&mut String){
     s1.push_str(" world");
}