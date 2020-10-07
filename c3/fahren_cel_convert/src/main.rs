fn to_fahrenheit(c: &f64) -> f64 {
	(c * 9.0 / 5.0) + 32.0 
}

fn to_celsius(f: &f64) -> f64 {
	(f - 32.0) * 5.0 / 9.0
}

fn main() {
	let fahren: f64 = 32.0;
	println!("{}°F equals {}°C", fahren, to_celsius(&fahren));
	let cel: f64 = 32.0;
	println!("{}°C equals {}°F", cel, to_fahrenheit(&cel));
}