use std::io;

fn nth_of_fib(nth: usize) -> i32 {
	let mut acc: i32 = 0;
	let mut nums = [0, 1];
	if nth < 3 {
		return nums[nth - 1]
	}
	for _i in 2..nth {
		acc = nums[0] + nums[1];
		nums[0] = nums[1];
		nums[1] = acc;
	}
	return acc
}

fn main() {
    println!("Fibonacci!");
    println!("What number in the Fibonacci sequence do you want to see?");

    let mut nth = String::new();

    io::stdin()
    	.read_line(&mut nth)
    	.expect("Failed to read line");

    let nth: usize = match nth.trim().parse() {
    	Ok(num) => num,
    	Err(_) => 0,
    };

    println!("The {} Fibonacci number is {}", nth, nth_of_fib(nth));
}