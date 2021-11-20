pub fn vars() {
    let language = "Rust"; // assignemnt of variable. In Rust variables are by default immutable
    println!("{}", language);
}

pub fn mut_vars() {
    let mut languages = "Rust";
    println!("{}", languages);
    languages = "Python";
    println!("{}", languages);
}

pub fn multi_vars() {
    let (course, category) = ("Rust", "beginner");
    println!("Learning {}, level: {}", course, category);
}

// There are local and global variables. In Rust, if you declare a variable inside '{}' then this variable is only accessible within that block of code.
// On the other hand, if you declare a variable ouside any '{}' then this variable is accessible globaly by the program.