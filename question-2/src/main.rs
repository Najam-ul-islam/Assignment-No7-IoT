/* Q2. Write a Rust Program,
 ● Make a library (lib.rs) alongwith main.rs
 ● Make a module with suitable name in library
 ● Make a sub module with an other name in first module
 ● Define a simple function in sub module
 ● use that library in main.rs
 ● Call that function from fn main
 */
//mod lib;
mod lib;
fn main() {
    lib::truth_table::xor::simple_function();

}
