//use std::io;

fn main() {
    let mut read_line = String::new(); 

    io::stdin()
        .read_line(&mut read_line)
        .expect("Failed to read line");

    let mut numbers = read_line.split_whitespace();
    let n = numbers.next().unwrap().parse().expect("Please enter a valid number");

    let mod_number = numbers.next().unwrap().parse().expect("Please enter a valid number");
    
    println!("{0}",fib_matrix(n, mod_number));
    //println!("{0}",fib(n, mod_number));
}

fn fib(n:usize,mod_number:usize)->usize{
    let (mut n_2,mut n_1) = (0,1);
    std::iter::repeat(()).take(n).for_each(|_| (n_2,n_1) = (n_1 , (n_2 + n_1)%mod_number));
    n_2
}

fn matrix_mult(a:[[usize;2];2],b:[[usize;2];2],mod_number:usize)->[[usize;2];2]
{
    [
        [
            ((a[0][0]*b[0][0])+(a[0][1]*b[1][0]))%mod_number,
            ((a[0][0]*b[0][1])+(a[0][1]*b[1][1]))%mod_number
        ],
        [
            ((a[1][0]*b[0][0])+(a[1][1]*b[1][0]))%mod_number,
            ((a[1][0]*b[0][1])+(a[1][1]*b[1][1]))%mod_number
        ]
    ]
}

fn fib_matrix(n:usize,mod_number:usize)->usize
{
    if n<=1 {return 1;}
    let mut exp = n;
    let mut result = [[1, 0], [0, 1]];
    let mut base = [[1, 1], [1, 0]];
    while exp>0
    {
        if exp % 2 == 1 {
            result = matrix_mult(result, base, mod_number);
        }
        base = matrix_mult(base, base, mod_number);
        exp /= 2;
    }

    return result[1][0];

}