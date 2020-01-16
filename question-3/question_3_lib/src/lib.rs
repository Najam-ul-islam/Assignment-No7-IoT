pub mod algorithm {
        pub mod sorting{

            pub fn bubble_sort() {
                let mut lst = [2,1,10,4,3,5,7,6,9,8,15];
                println!("========================================================");
                println!("Original Array:{:?}",lst);
                // Bubble Sort Algorithm
                // count total  Iterations
                let mut count = 0;
                for _i in 0..lst.len() {
                    for ele in 0.._i{
                        // if index[0] value is greater index[1]
                        if lst[ele] > lst[ele + 1]{
                            // storing swaped values in "tmp"
                            let tmp = lst[ele];
                            // swap values
                            lst[ele] = lst[ele + 1];
                        //    lst[ele + 1] = lst[ele];
                            // add tmp value back to index
                            lst[ele + 1] = tmp;
                            count = count + 1;
                        }
                    }
                }
                println!("\n");
                println!("Sorted Array: {:?}",lst);
                println!("Total Iterations: {:?}",count);
                println!("========================================================");
            }

        }
    }
