// Double slash to print line commands

/* This is a block of comments,
it will disable large chuncks of code within
so the interpreter will ignore them */

// Triple '///' slash is to include doc comments. It supports markdown notation. 
// This '//!' is used to include doc comments inside a block of code.
// }

// Testing placeholders for strings in Rust
pub fn ph() {
    println!("My name is {}", "Ricardo");
}

pub fn ph_arg() {
    println!("I am {age} years old, {heigh} meters tall and I live in {city}", age = 32, heigh = 1.80, city = "Lisbon");
}

pub fn ph_mults(){
    println!("{:?}", ("This is a test for multiple placeholders", 2))
}

// We use the 'let' command to assign values to varibales.
// In Rust variables are by default immutable.
pub fn variable() {
    let language = "Rust";
    println!("Language: {}", language);
}

// In order to make tha variable mutable, we need to use the 'mut' command after 'let', like so:
pub fn mut_var() {
    let mut language = "Rust";
    println!("Language: {}", language);
    language = "Java";
    println!("Language: {}", language);
}