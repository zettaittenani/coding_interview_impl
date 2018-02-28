static mut sorted: Vec<i32> = vec![];

fn main() {
    let input: Vec<i32> = vec![5, 2, 6, 8, 3, 4, 1, 10, 9, 7];
    let mut cloned_input: Vec<i32> = input.clone();
    let start_id: usize = 0;
    let end_id: usize = cloned_input.len();

    _merge_sort(&mut cloned_input, start_id, end_id);

    unsafe {
        for i in 0..sorted.len() {
            println!("{}", sorted[i]);
        }
    }
}

fn _merge_sort(slice: &mut [i32], start_id: usize, end_id: usize) {
    if end_id <= start_id {
        return
    }

    let mut middle_id = (start_id + end_id) / 2;
    let (mut left, mut right) = slice.split_at_mut(middle_id);

    _merge_sort(left, start_id, middle_id);
    _merge_sort(right, middle_id + 1, end_id);

    _merge(left, right, start_id, middle_id, end_id)
}

fn _merge(left: &mut [i32], right: &mut [i32], start_id: usize, middle_id: usize, end_id: usize) {
    for i in start_id..middle_id {
        for j in (middle_id  + 1)..end_id {
            unsafe {
                if left[i] < right[j] {
                    sorted[i] = left[i];
                } else {
                    sorted[i] = right[j];
                }
            }
        }
    }
}
