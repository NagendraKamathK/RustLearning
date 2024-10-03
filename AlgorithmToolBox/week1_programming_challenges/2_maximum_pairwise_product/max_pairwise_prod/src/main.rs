pub fn main() {
    let mut n = String::new();
    let mut buff = String::new();
    ::std::io::stdin().read_line(&mut n);
    ::std::io::stdin().read_line(&mut buff);
    let mut words = buff.split_whitespace();

    let mut nums:Vec<i32> = words.map(|x| x.parse::<i32>().unwrap()).collect();

    println!("{}", max_product_fastest(&nums));
}

pub fn max_product_fastest(nums: &Vec<i32>) -> i32 {
    let mut first_largest = i32::MIN;
    let mut second_largest = i32::MIN;

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




