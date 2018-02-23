fn main() {
    let input: [i32; 10] = [5, 2, 6, 8, 3, 4, 1, 10, 9, 7];
    let mut output = input.clone();
    let middle_id: usize = input.len() / 2;

    _merge_sort(&mut output, middle_id);

    for i in 0..output.len() {
        println!("{}", output[i]);
    }
}

fn _merge_sort(slice: &mut [i32], middle_id: usize) {
    let (mut left, mut right) = slice.split_at_mut(middle_id);
    let left_middle_id: usize = left.len() / 2;
    let right_middle_id: usize = right.len() / 2;

    if (left.len()> 2) || (right.len() > 2) {
        _merge_sort(left, left_middle_id);
        _merge_sort(right, right_middle_id);
    }

    _merge(left, right);
}

fn _merge(left: &mut [i32], right: &mut [i32]) {
    // TODO impl
    println!("left: {}, right: {}", left[0], right[0])
}
