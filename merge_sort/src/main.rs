fn main() {
    let mut input: Vec<u8> = vec![5, 2, 6, 8, 3, 4, 1, 10, 9, 7];
    let merged = merge_sort(&mut input);
    for elem in &merged {
        println!("{}", elem);
    }
}

fn merge_sort(array: &mut Vec<u8>) -> Vec<u8> {
    let half = array.len() / 2;
    let mut left: Vec<u8> = array[0..half].to_vec();
    let mut right: Vec<u8> = array[half..(array.len())].to_vec();

    if 1 <= left.len() {
        left = merge_sort(&mut left);
        right = merge_sort(&mut right);
    }

    merge(&mut left, &mut right)
}

fn merge(left: &mut Vec<u8>, right: &mut Vec<u8>) -> Vec<u8> {
    let mut ret: Vec<u8> = vec![];

    loop {
        if left.is_empty() && right.is_empty() {
            break;
        } else if left.is_empty() {
            ret.push(right.remove(0));
        } else if right.is_empty() {
            ret.push(left.remove(0));
        } else {
            if left.first() < right.first() {
                ret.push(left.remove(0));
            } else {
                ret.push(right.remove(0));
            }
        }
    }

    ret
}
