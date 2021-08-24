fn main() {
	/**
	let r;
	{
		let x=5;
		r=&x;
	}
	println!("the r value:{}",r);
	 */
	let s1 = String::from("abc");
	let s2 = String::from("abcd");
	let s3 = longest_str(&s1, &s2);
	println!("the logest str is:{}", s3);

	let result;
	let longStr = String::from("long long str");
	{
		let t = String::from("dcd1");
		result = longest_str(&longStr, &t);
	}
	println!("the logest str is:{}", result);
}
fn longest_str<'a>(s1: &'a str, s2: &'a str) -> &'a str {
	if s1.len() > s1.len() {
		&s1
	} else {
		&s2
	}
}
