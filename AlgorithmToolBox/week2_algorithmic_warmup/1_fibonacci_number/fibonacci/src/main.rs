fn main() {
    println!("{0}",fib(3));
}

fn fib(n:i32)->i32{
    if n <=1 {return n}        
    fib(n-2)+fib(n-1)
}