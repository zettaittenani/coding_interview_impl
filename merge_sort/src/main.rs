fn main() {
    let input: Vec<usize> = vec![5, 2, 6, 8, 3, 4, 1, 10, 9, 7];
    let mut cloned_input: Vec<usize> = input.clone();
    let mut work: Vec<usize> = input.clone();
    let start_id: usize = 0;
    let end_id: usize = cloned_input.len();

    _merge_sort(&mut cloned_input, &mut work, start_id, end_id);

    for i in 0..work.len() {
        println!("{}", work[i]);
    }
}

fn _merge_sort(slice: &mut [usize], work: &mut [usize], start_id: usize, end_id: usize) {
    if end_id <= start_id {
        return
    }

    let middle_id: usize = (start_id + end_id) / 2;
    _merge_sort(slice, work, start_id, middle_id);
    _merge_sort(slice, work, (middle_id + 1), end_id);

    _merge(slice, work, start_id, middle_id, end_id)
}

fn _merge(slice: &mut [usize], work: &mut [usize], start_id: usize, middle_id: usize, end_id: usize) {
    let mut i: usize = 0;
    let mut j: usize = middle_id + 1;
    let mut k: usize = 0;

    for m in start_id..middle_id {
        work[k] = slice[m];
        k += 1;
    }

    for n in (middle_id + 1)..end_id {
        work[k] = slice[n];
        k += 1;
    }

    for k in start_id..end_id {
        if work[i] < work[j] {
            slice[k] = work[i];
            i += 1;
        } else {
            slice[k] = work[j];
            j -= 1;
        }
    }

    for l in 0..slice.len() {
        println!("i: {}, slice[i]: {}", l, slice[l]);
    }
}
