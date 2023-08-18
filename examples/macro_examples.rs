/// macro_examples.rs
///
/// This is a file containing examples of creatable Rust macroprocesses.

// ---- OUTLINE ----
// # IMPORTS .... (14)
// # ALIASES .... (18)
// # ENUMS ...... (24)
// # STRUCTS .... (32)
// # MACROS ..... (40)
// # FUNCS ...... (47)
// # MAIN ....... (57)

// ---- IMPORTS ----

// use std::io::*;

// ---- ALIASES ----

/// UwU Type Alias
///
/// Alias for u128

// ----  ENUMS  ----

/// MyState Enum
///
/// StateTheFirst
/// StateTheSecond
/// StateTheThird

// ---- STRUCTS ----

/// MyTable Struct
///
/// id: i32 - The Primary Key for MyTable
/// field1: String - The 1st (Non-PK) field of MyTable
/// field2: bool - The 2nd (Non-PK) field of MyTable

// ---- MACROS ----

/// my_first_macro Macro
///
/// # Parameters
/// $expr - An expression to be evaluated, then printed

// ----  FUNCS  ----

/// NO Function
///
/// A function for `drop`-ing (or `NO`-ing) variables.
///
/// # Parameters
/// x: _ - The variable to be NO'd
fn NO(x: _) { }

// ----  MAIN  ----

/// Just the main function.
fn main() {
    NO(0);
}
