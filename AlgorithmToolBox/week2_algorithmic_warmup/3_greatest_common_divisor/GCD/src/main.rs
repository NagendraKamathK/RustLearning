fn main() {
    let mut read_line = String::new();
    std::io::stdin().read_line(&mut read_line).unwrap();

    let mut numbers = read_line.split_whitespace();

    let first_number:usize = numbers.next().unwrap().parse().expect("Please enter a valid number");
    let second_number:usize = numbers.next().unwrap().parse().expect("Please enter a valid number");

    println!("{0}",gcd(first_number,second_number));
}

fn gcd(a:usize, b:usize) -> usize{
    if b == 0 {return a;}
     gcd(b, a % b)
}