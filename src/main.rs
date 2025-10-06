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

/* Integer Type and Range

Bits        Signed Type   Range (iX)                                                Unsigned Type   Range (uX)
---------------------------------------------------------------------------------------------------------------
8-bit       i8            -128 to 127                                              u8              0 to 255
16-bit      i16           -32,768 to 32,767                                        u16             0 to 65,535
32-bit      i32           -2,147,483,648 to 2,147,483,647                          u32             0 to 4,294,967,295
64-bit      i64           -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807  u64             0 to 18,446,744,073,709,551,615
128-bit     i128          -170,141,183,460,469,231,731,687,303,715,884,105,728     u128            0 to 340,282,366,920,938,463,463,374,607,431,768,211,455
Arch-dep.   isize         same as i32 on 32-bit, i64 on 64-bit                     usize           same as u32 on 32-bit, u64 on 64-bit
*/

// default interger use "i32"
// rust support conversion integer type number, like from i8 -> i64
// but, still aware in rust have a  "integer overflow" like if converting integer type from large to small
// or if interger data cannot be acomodateed bt the specified data type
// to convert data type interger use keyword "as"

#[test]
fn conversion_integer_type() {
    let a: i8 = 10;
    println!("{}", a);

    let b: i64 = a as i64;
    println!("{}", b);

    // here example of "integrer overflow"
    let c: i64 = 1000000000;
    println!("{}", c);

    let d: i8 = c as i8; // return 0 value
    println!("{}", d)
}

/* Operator Numeric */
// + > sum
// - > substraction
// / > division
// * > multiply
// % > modulo

#[test]
fn numeric_operator() {
    let a = 20;
    let b = 10;
    let c = a + b;
    println!("{}", c);

    let c = a - b;
    println!("{}", c);

    let c = a / b;
    println!("{}", c);

    let c = a * b;
    println!("{}", c);

    let c = a % b;
    println!("{}", c);
}

#[test]
fn augmented_assigment() {
    // variable must be mutable if use augmanted assigment
    let mut a = 20;

    println!("{}", a);

    a += 100;

    println!("{}", a);
}

/* Booelan data type */
// boolean use keyword "bool"

#[test]
fn boolean_data_type() {
    let a = true;
    let b: bool = false;

    println!("{}", a);
    println!("{}", b);
}

/* Comparation Operator */
// Compraeation operator return booela value
// ">" more than
// "<"  less than
// "==" same value
// ">=" more than equal
// "<=" more than equal

#[test]
fn comparation_operator() {
    let a = 10;
    let b = 20;

    let result = a > b;
    println!("{}", result)
}

/* Logical Operator */
// Logical Operator return a boolean
/*
Operator    Name                Description                                Example (Result)
--------------------------------------------------------------------------------------------
&&          Logical AND         true if both operands are true             true && false => false
||          Logical OR          true if at least one operand is true       true || false => true
!           Logical NOT         inverts a boolean value                    !true => false
*/
#[test]
fn logical_operator() {
    // check if the price in budget checkout
    let budget = 50;
    let price = 30;

    let checkout = price < budget;
    println!("{}", checkout)
}

// Char data type
// Rust have a data type char use single qutoe -> 'A'
#[test]
fn char_data_type() {
    let a = 'A';
    let b = 'b';

    println!("{} {}", a, b)
}

/* COMPUND DATA TYPES */
// tuple fixed collection that can vary, data cannot add or remove
#[test]
fn tuple_data_type() {
    // create tuple of data author(its fake, trust me)
    let a = ("Bodro Adikoro", 30, "Gaming");
    println!("{:?}", a);

    // accesing tuple data, get author name, age, and hobby
    println!("name: {}", a.0);
    println!("age: {}", a.1);
    println!("hobby: {}", a.2)
}

#[test]
fn destructuring_tuple() {
    let a = ("Bodro Adikoro", 30, "Gaming");
    println!("{:?}", a);

    // if data unused use "_"
    let (name, age, hobby) = a;
    println!("{} {} {}", name, age, hobby)
}

#[test]
fn muttable_tuple() {
    // to modify tuple data
    // Default tuple is immutable to make mutable use keyword "mut"
    let mut a = ("Bodro Adikoro", 30, "Gaming");
    println!("{:?}", a);

    // Chage hobby from Gaming to Code
    a.2 = "Code";

    println!("{:?}", a);
}

// in rust if function not return value defult return value is emty tuple only ()
#[test]
fn unit() {
    println!("Heres is function not return anything!")
}

#[test]
fn default_return_value_unit() {
    let result = unit();
    println!("{:?}", result)
}

/* Array data type */
// array is a collection fiexed data and value must be same
#[test]
fn array_data_type() {
    // length is fixed
    let a: [i32; 4] = [10, 20, 30, 40];
    println!("{:?}", a);

    // accesing array use index of array from "0"
    let b = a[2];
    println!("{}", b) // result 30
}

#[test]
fn muttable_array() {
    // to modify data array
    // defult array is immutable use "mut" keyword to make it muttable
    let mut a: [i32; 4] = [10, 20, 30, 40];
    println!("{:?}", a);

    a[0] = 100;
    a[3] = 400;
    println!("{:?}", a);

    // get array length
    let length = a.len();
    println!("{}", length)
}

#[test]
fn dimensional_array() {
    let matrix: [[i32; 2]; 2] = [[10, 20], [40, 60]];
    println!("{:?}", matrix);

    // accesnig array
    let a = matrix[1];
    let b = matrix[1][0];
    println!("{:?}", a);
    println!("{:?}", b);
}
