fn main() {

    let mut read_line = String::new();
    std::io::stdin()
        .read_line(&mut read_line)
        .expect("Failed to read line");

    let n:usize = read_line.trim().parse().expect("Please enter a valid number");
    let f_n_plus_2 = fib_matrix(n+2,10);

    println!("{0}",if f_n_plus_2<1 {9} else {f_n_plus_2-1}); //Sn = fn+2-1
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
    result[1][0]
}