fn main() {
    let input: Vec<usize> = vec![5, 2, 6, 8, 3, 4, 1, 10, 9, 7];
    let mut output: Vec<usize> = input.clone();
    let mut work: Vec<usize> = vec![0; 10];
    let start_id: usize = 0;
    let end_id: usize = output.len() - 1;

    _merge_sort(&mut output, &mut work, start_id, end_id);

    for i in 0..output.len() {
        println!("{}", output[i]);
    }
}

fn _merge_sort(slice: &mut [usize], work: &mut [usize], start_id: usize, end_id: usize) {
    if end_id <= start_id {
        return
    }

    let middle_id: usize = (start_id + end_id) / 2;

    _merge_sort(slice, work, start_id, middle_id);
    _merge_sort(slice, work, (middle_id + 1), end_id);

    let mut i: usize = start_id;
    let mut m: usize = start_id;
    let mut n: usize = end_id;

    while m <= middle_id {
        work[i] = slice[m];
        i += 1;
        m += 1;
    }

    i = middle_id + 1;

    while n >= (middle_id + 1) {
        work[i] = slice[n];
        i += 1;
        n -= 1;
    }

    let mut j: usize = start_id;
    let mut k: usize = end_id;
    let mut l: usize = start_id;

    while l <= end_id {
        if work[j] <= work[k] {
            slice[l] = work[j];
            j += 1;
            l += 1;
        } else {
            slice[l] = work[k];
            k -= 1;
            l += 1;
        }
    }
}
