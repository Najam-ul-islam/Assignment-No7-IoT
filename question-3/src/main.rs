/* Q3. Write a Rust library,
● Make a library package
● Make a module with suitable name in library
● Make a sub module with an other name in first module
● Define a simple function in sub module
● make a binary package
● add your library in dependencies in cargo.toml
● use your library in main.rs
● Call that function from fn main
*/
use question_3_lib;

fn main() {
    let mut lst:[u8;12] = [2,1,10,4,3,5,7,6,9,8,0,15];
    println!("Original Array:{:?}",lst);
    // Function call
     let bubble = question_3_lib::algorithm::sorting::bubble_sort(lst);
    println!("New Array: {:?}",bubble);
}
