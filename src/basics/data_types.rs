fn main() {
    let num:i32 = 10;
    let float:f32 = 10.2;
    let mut double:f64 = 10.2222;
    println!("{} {} {}", num, float, double);

    double = 10.23232;
    println!("{} {} {}", num, float, double);

    let string:String = String::from("Hola mundo");
    let flag:bool = true;
    let num_2:i32=10_000;
    println!("{} {} {}",string, flag, num_2);

    let char:char= 'a';
    let age: u32 = 1_100_100_000;
    println!("{} {}", char, age);
    
    let counter:usize = 10;
    let counter_:i8 = 10;
    println!("{} {}",counter, counter_);

    println!("{}", '😁');
}
