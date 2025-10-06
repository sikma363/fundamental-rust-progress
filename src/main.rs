fn main() {
    println!("Rust Fundamental")
}

#[test]
fn hello_test() {
    println!("Test, Unit Test!")
}

#[test]
fn variable() {
    // By default, in rust variable is immutable.
    let name = "Bodro Adikror";
    println!("{}", name)
}

#[test]
fn variable_mutable() {
    // to create muttable varibale use keyword "mut"
    let mut name = "Bodro Adikror";
    println!("{}", name);

    name = "Sikma 363";
    println!("{}", name);
}

#[test]
fn shadowing() {
    //  shadowing rust create same variable name with let keyword
    let name = "Bodro Adikror";
    println!("{}", name);

    let name = 10;
    println!("{}", name);
}

// data type in rust in general divide into 2, scalar and compound
// scalar represent a single value (int, float, boolean, char)
// compunet represent a multi value (array, tuple)

/* SCLAR */
// integer
// float decimal
// boolean
// char

/* COMPOUND */
// tuple a collecion data that can vary
// array a collection data must be same

/* RUST is a satic type */
// but rust support explicit type
// to set variable type use "<varname>: <type>"

#[test]
fn variable_explicit_implicit_type() {
    // explicit variable type
    let num_a = 10;
    println!("{}", num_a);

    // implicit variable type
    let num_b: i8 = 10;
    println!("{}", num_b)
}
