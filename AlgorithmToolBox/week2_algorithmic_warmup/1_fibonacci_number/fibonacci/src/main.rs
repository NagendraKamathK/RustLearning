fn main() {
    let mut buff = String::new();
    ::std::io::stdin().read_line(&mut buff);
    let mut words = buff.split_whitespace();
    let a: u32 = words.next().unwrap().parse().unwrap();

    println!("{0}",fib(a));
}

fn fib(n:u32)->u32{
    let mut n_2:u32 = 0;
    let mut n_1:u32 = 1;
  for i in 1..n
  {
      let temp = n_2;
      n_2 = n_1;
      n_1 = temp+n_2;
  }
    n_1
}