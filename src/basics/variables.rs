use std::f64::consts::PI;

fn main(){
    const _PI:f64 = PI;
    println!("{}", _PI);

    // by default variables defined with "let" are inmutable (read-only)
    let salary:f32 = 8_789.23;
    println!("My salary is {}", salary);

    // add "mut" in the definition to perform mutations on the value
    let mut debts:f32 = 102_233.23;

    println!("I will pay some part of {}", debts);
    
    // with "mut" debts, is mutable
    debts = 100_232_1.1;
    println!("I have {} of debt",debts);
}