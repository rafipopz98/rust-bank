fn main(){
    let string_input = "hello, my name is rafi";
    that_prints_string(string_input);
let result = sum(1,2);
println!("{}",result);

}


fn that_prints_string(string_input:&str){
    println!("{}",string_input);
}

fn sum (num1:u8, num2:u8)->u8{
    println!("{}", num1+num2);
    return num1+num2;
}