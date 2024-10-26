fn main() {
    let mut read_line = String::new();

    std::io::stdin().
        read_line(&mut read_line).
        expect("Failed to read line");

    let mut numbers = read_line.split_whitespace();
    let lower_limit:usize = numbers.next().unwrap().parse().expect("Please enter a valid number");

    let upper_limit:usize = numbers.next().unwrap().parse().expect("Please enter a valid number");

    if lower_limit == upper_limit
    {
        println!("{0}",fib_matrix(upper_limit,10));
        return
    }
    let (f_upperlimit_plus_2,f_lowerlimit_plus_1) = (fib_matrix(upper_limit+2,10),fib_matrix(lower_limit+1,10));
    println!("{0}", if f_upperlimit_plus_2 < f_lowerlimit_plus_1
                    {10-(f_lowerlimit_plus_1-f_upperlimit_plus_2)}
                    else
                    {f_upperlimit_plus_2-f_lowerlimit_plus_1}
    );

    //Sn=fn+2-1 so Sum from M to n is Sn-Sm-1 => f(n+2) - f(m+1)1
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

fn fib_matrix(upper_limit:usize,mod_number:usize) ->usize
{
    if upper_limit ==0 {return 0;}
    if upper_limit ==1 {return 1;}
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
    return result[1][0];
}