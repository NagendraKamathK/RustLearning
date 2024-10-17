fn main() {
    let mut buff = String::new();
    ::std::io::stdin().read_line(&mut buff);
    let mut words = buff.split_whitespace();
    let a: usize = words.next().unwrap().parse().unwrap();

    println!("{0}",fib(a));
}
fn fib(n:usize)->u32{
    let (mut n_2,mut n_1) = (0,1);
    std::iter::repeat(()).take(n).for_each(|_| (n_2,n_1) = (n_1 , n_2 + n_1));
    n_2
}