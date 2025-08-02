// using clones

fn main(){
let s1 = String::from("this is another big string");
let len = get_len(s1.clone());
println!("the length of {} is {}",s1,len);
}

fn get_len(item:String)-> usize{
    return item.len();
}






// avoiding ownerships using tuples

#[allow(dead_code)]

fn yep_one_more_function (){
    let s1 = String::from(" this is avery ong text");
    let (s2, len)= gets_length(s1);

    println!("the length of {} is {}",s2,len);
}
fn gets_length (str:String)->(String,usize){
    let length = str.len();
    return (str,length)

}








#[allow(dead_code)]
fn one_more_function_again (){
    let s1:String = String::from("hello");
    println!("s1 has {}",s1);

    let s2 = get_s2_value();

    let s3 = return_got_string(s2);


    println!("s3 has {}",s3);

}


fn get_s2_value()->String{
    let s2 = String::from("world");
    return s2;
}

fn return_got_string(item:String)->String{
    return item;
}





#[allow(dead_code)]
fn one_more_function (){
    let mut new_str = String::from("hello");
    println!("{} in main function",new_str);

    new_str = another_func(new_str);

    println!("{} after all shit",new_str);

}
#[allow(dead_code)]
fn another_func(new_str: String)->String{
    println!("{} in new function",new_str);
    return new_str;
}



#[allow(dead_code)]
fn another_ex(){
    let a = 89;

    println!("a in main = {}",a);

    new_function(a);
    println!("a in main= {}",a);
}


#[allow(dead_code)]
fn new_function (a:u8){
    println!(" a in another function {}",a);
}




#[allow(dead_code)]
fn ownership(){
    // let a =7;
    // let b = a;

    // println!("a = {}",a);
    // println!("b = {}",b);

    // this will work



    // let str1 = "hello";
    // let str2 = str1;

    // println!("str1 = {}",str1);
    // println!("str2 = {}",str2);


    // this will also work




    // let str1:String = String::from("hello");
    // let str2:String = str1;

    // println!("str1 = {}",str1);
    // println!("str2 = {}",str2);



    // wont work cz HEAP MEMORY


    let str1:String = String::from("hello"); // -> str1 is the owner of hello
    let str2:String = str1; // -> transfer of ownership -> str2 is the new owner of hello

    // println!("str1 = {}",str1);
    println!("str2 = {}",str2);

}





#[allow(dead_code)]
fn scope(){
let mut out_side  = 100;

println!("The value of out_side is: {}",out_side);

{
    let in_side = 10;
    println!("The value of in_side is: {}",in_side);
    out_side +=100;
    println!("the value of out_side is {}" , out_side) 
}
// println!("The value of out_side is: {}",in_side);
println!("The value of out_side is: {}",out_side);

}