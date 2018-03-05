fn main() {
    let input: Vec<usize> = vec![3, 2, 5, 9, 6, 10, 1, 4, 7, 8];
    let mut mut_input: Vec<usize> = input.clone();
    let start_id: usize = 0;
    let end_id: usize = mut_input.len();

    let output  = _quick_sort(&mut mut_input, start_id, end_id);
}

fn _quick_sort(slice: &mut [usize], start_id: usize, end_id: usize) -> &mut [usize] {
    let slice_len: usize = slice.len();
    if slice_len < 2 {
        return slice
    }

    let pivot: usize = slice[start_id];
    let mut left: Vec<usize> = vec![];
    let mut right: Vec<usize> = vec![];

    for i in start_id..end_id {
        if slice[i] <= pivot {
            left.push(slice[i]);
        } else {
            right.push(slice[i]);
        }
    }

    let left_len: usize = left.len();
    let right_len: usize = right.len();

    println!("{}, {}", left_len, right_len);

    _quick_sort(&mut left, 0, left_len);
    _quick_sort(&mut right, 0, right_len);

    let mut ret: Vec<usize> = vec![];
    ret.append(&mut left);
    ret.append(&mut vec![pivot]);
    ret.append(&mut right);
    &mut ret
}
