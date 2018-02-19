fn main() {
    let input: [i32; 5] = [5, 2, 4, 3, 1];
    let mut output = input.clone();

    for m in 0..output.len() {
        for n in 0..output.len() {
            if output[m] < output[n] {
                output.swap(m, n);
            }
        }
    }
    
    println!("{}", output[0]);
    println!("{}", output[1]);
    println!("{}", output[2]);
    println!("{}", output[3]);
    println!("{}", output[4]);
}
