use std::io;

fn main() {
    let mut read_line = String::new(); 

    io::stdin()
        .read_line(&mut read_line)
        .expect("Failed to read line");

    let n = read_line.trim().parse().expect("Please enter a valid number");
    println!("{0}",fib(n));
}

fn fib(n:usize)->u32{
    let (mut n_2,mut n_1) = (0,1);
    std::iter::repeat(()).take(n).for_each(|_| (n_2,n_1) = (n_1 , (n_2 + n_1)%10));
    n_2
}