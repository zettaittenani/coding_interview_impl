fn main() {
    let input: [i32; 5] = [5, 2, 4, 3, 1];
    let mut cloned_input = input.clone();
    let output = bubble_sort(&mut cloned_input);
    print(output);
}

fn bubble_sort(input: &mut [i32]) -> &mut [i32] {
    let input_len = input.len();

    for i in 0..input_len {
        for j in 1..(input_len - i) {
            if input[j-1] > input[j] {
                input.swap(j-1, j);
            }
        }
    }
    input
}

fn print(output: &mut [i32]) {
    for j in 0..output.len() {
        println!("{}", output[j]);
    }
}
