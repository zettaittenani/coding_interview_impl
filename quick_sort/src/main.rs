fn main() {
    let mut input: Vec<u8> = vec![3, 2, 5, 9, 6, 10, 1, 4, 7, 8];
    println!("{:?}", quick_sort(&mut input));
}

fn quick_sort(slice: &mut Vec<u8>) -> Vec<u8> {
    let len = slice.len();
    if len == 1 { return slice.to_vec(); };

    let pivot = slice[len / 2];
    let mut split = 0;

    for i in 0..len {
        for j in (0..len).rev() {
            if i <= j {
                if pivot <= slice[i] && slice[j] <= pivot {
                    slice.swap(i, j);
                    split = j;
                }
                // println!("{:?}", pivot);
                // println!("{:?}", slice);
            }
        }
    }

    if split == 0 { return slice.to_vec(); };
    let mut former: Vec<u8> = quick_sort(&mut slice[0..split].to_vec());
    let mut latter: Vec<u8> = quick_sort(&mut slice[split..len].to_vec());
    former.append(&mut latter);
    former
}
