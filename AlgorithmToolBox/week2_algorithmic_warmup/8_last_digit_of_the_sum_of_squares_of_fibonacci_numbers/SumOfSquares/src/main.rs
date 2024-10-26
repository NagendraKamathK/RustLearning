fn main() {
    let mut read_line = String::new();
    std::io::stdin().read_line(&mut read_line).unwrap();

    let n:usize = read_line.trim().parse().expect("Please enter valid number!");

    println!("{0}",if n > 0 {fib_sum_of_squares_mod(n,10)} else {0})//fn+1 * fn
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

fn fib_sum_of_squares_mod(upper_limit:usize,mod_number:usize) ->usize
{
    if upper_limit <=1 {return 1;}
    let mut exp = upper_limit;
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
    (result[0][0]*result[0][1])%mod_number
    //Fn+1 * Fn
}