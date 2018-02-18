fn main() {
    let mut input: [i32; 5] = [5, 2, 4, 3, 1];
    test(input);
}

fn test(input: [i32; 5]){
    swap(&input[0], &input[1]);
    println!("{}", input[0]);
}

fn swap<'a>(first: &'a mut i32, second: &'a mut i32){
    let mut temp: &i32 = &0;
    
    if first > second {
        temp = first;
        first = second;
        second = temp;
    }
}
