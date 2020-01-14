/* Q1. Write a Rust Program,
 ● Make a module with suitable name in main.rs
 ● Make a sub module with an other name in first module
 ● Define a simple function in sub module
 ● Call that function from fn main
*/
// Make a module with suitable name in main.rs
mod  introduction {
	// Make a sub module with an other name in first module
	pub mod my_intro{
		// Define a simple function in sub module
		pub fn simple(){
			println!("Muhammad Najam Ul Islam\nPIAIC IoT Batch-3 Morning");
			
		}
	}


}
fn main() {
	// function call
    introduction::my_intro::simple();

}
