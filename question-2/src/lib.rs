pub mod truth_table{
	pub mod xor{
		pub fn simple_function(){
			let input = [[0,0],
					 	 [0,1],
					 	 [1,0],
					 	 [1,1]];

			let output = [0,1,1,0];
			println!("============");
			println!(" A  B     C");
			println!("============");
			println!(" {:?}  {:?} \t  {:?}",input[0][0],input[0][1],output[0]);
			println!(" {:?}  {:?} \t  {:?}",input[1][0],input[1][1],output[1]);
			println!(" {:?}  {:?} \t  {:?}",input[2][0],input[2][1],output[1]);
			println!(" {:?}  {:?} \t  {:?}",input[3][0],input[3][1],output[0]);
            println!("============");
		} 
	} 
}