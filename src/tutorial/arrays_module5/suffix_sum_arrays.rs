use crate::tutorial::common_util::create_array;

pub fn suffix_sum_arrays() {
    println!("==> Suffix-Sum Arrays (Accidental implementation)");
    let arr_source_values = create_array(3);
    let prefix_sum_array = make_sufix_sum_array(&arr_source_values);
    println!("{:?}", prefix_sum_array);
    print!("\n\n");
}

fn make_sufix_sum_array(arr: &[i32]) -> Vec<i32> {
    // I accidently made the suffix one misunderstanding the question - I mis read
    // the expected outcome.
    // We will linear process - pre calculate all the results from right to left.
    // Sums from i to len for value of i, but does it cumlatively from right to left.
    let size = arr.len();
    if size == 0 {
        return Vec::new();
    }
    let mut suffix_sum_array = vec![0; size];
    suffix_sum_array[size - 1] = arr[size - 1];
    if size > 1 {
        for i in (0..(size - 1)).rev() {
            // current slot value + the following sum value already calculated.
            suffix_sum_array[i] = arr[i] + suffix_sum_array[i + 1];
        }
    }
    suffix_sum_array
}

#[test]
fn test_suffix_array_sums() {
    let cases = vec![
        // (input, expected_suffix)
        (vec![], vec![]),
        (vec![5], vec![5]),
        (vec![1, 2, 3], vec![6, 5, 3]),
        (vec![2, 4, 6, 8, 10], vec![30, 28, 24, 18, 10]),
        (vec![10, -2, 3, -1], vec![10, 0, 2, -1]),
        (vec![-1, -2, -3], vec![-6, -5, -3]),
    ];

    for (input, expected) in cases {
        let output = make_sufix_sum_array(&input);
        assert_eq!(output, expected, "Failed on input: {:?}", input);
    }
}
