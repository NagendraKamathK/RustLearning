pub fn main() {
    let mut n = String::new();
    let mut buff = String::new();
    ::std::io::stdin().read_line(&mut n);
    ::std::io::stdin().read_line(&mut buff);
    let mut words = buff.split_whitespace();

    let mut nums:Vec<i64> = words.map(|x| x.parse::<i64>().unwrap()).collect();

    println!("{}", max_product_fastest(&nums));
}

pub fn max_product_fastest(nums: &Vec<i64>) -> i64 {
    let mut first_largest = i64::MIN;
    let mut second_largest = i64::MIN;

    if nums.len() <= 1 { return nums[0];}

    for num in nums {
        if *num > first_largest {
            second_largest = first_largest;
            first_largest = *num;
        }
        else if *num > second_largest {
            second_largest = *num;
        }
    }
    first_largest * second_largest
}




