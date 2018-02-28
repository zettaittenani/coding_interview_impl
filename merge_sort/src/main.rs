fn main() {
    let input: Vec<usize> = vec![5, 2, 6, 8, 3, 4, 1, 10, 9, 7];
    let mut cloned_input: Vec<usize> = input.clone();
    let mut work: Vec<usize> = input.clone();
    let start_id: usize = 0;
    let end_id: usize = cloned_input.len() - 1;

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

    let mut i: usize = 0;

    for m in start_id..middle_id {
        work[i] = slice[m];
        i += 1;
    }

    i = end_id;

    for n in (middle_id + 1)..end_id {
        work[i] = slice[n];
        i -= 1;
    }

    let mut j: usize = 0;
    let mut k: usize = end_id;

    for l in start_id..end_id {
        if work[j] < work[k] {
            slice[l] = work[j];
            j += 1;
        } else {
            slice[l] = work[k];
            k -= 1;
        }
    }

    for o in 0..slice.len() {
        println!("i: {}, slice[i]: {}", o, slice[o]);
    }
}
