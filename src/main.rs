fn main() {
}

fn is_even(n: u32) -> bool {
    if n % 2 == 0 {
        //return true;	// Return with the return keyword
	true
    } else {
        return false;
    }
}
fn digit_sum(n: i32) -> i32 {
	let mut n = n;
	let mut sum = 0;
	while n > 0 {
 	let digit = n % 10;
	sum += digit;
	n /= 10;
	}
	sum
}
