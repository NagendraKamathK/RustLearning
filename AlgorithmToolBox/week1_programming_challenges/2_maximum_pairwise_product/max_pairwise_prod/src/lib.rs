pub mod max_pair {
    pub fn max_pairwise_product(nums: &Vec<i32>) -> i32 {
        if nums.len()<=1 {return nums[0];}

        let mut product:i32 = 1;
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if (nums[i] * nums[j]) > product {
                    product = nums[i] * nums[j];
                }
            }
        }
        product
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
}

mod tests{
    use rand::Rng;
    use crate::max_pair::{max_pairwise_product, max_product_fastest};
    #[test]
    fn simple_test_case_1() {
        let nums = vec![15, 23, 15, 3, 20, 58, 91, 72, 17, 80];
        let expected_result = max_pairwise_product(&nums);
        let actual_result = max_product_fastest(&nums);
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn simple_test_case_2() {
        let nums = vec![2, 3];
        let expected_result = max_pairwise_product(&nums);
        let actual_result = max_product_fastest(&nums);
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn simple_test_case_3() {
        let nums = vec![2];
        let expected_result = max_pairwise_product(&nums);
        let actual_result = max_product_fastest(&nums);
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn stress_test_case_4() {
        let size = 100;
        let mut rng = rand::thread_rng();
        let nums: Vec<i32> = (0..size).
            map(|_| rng.gen_range(0..100)).collect();

        let expected_result = max_pairwise_product(&nums);
        let actual_result = max_product_fastest(&nums);
        assert_eq!(expected_result, actual_result);
    }
}