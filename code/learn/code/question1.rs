use std::io;

fn take_input() -> [u32; 2] {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to get input");

    let buffer = input.split(" ");
    let mut arr: [u32; 2] = [0; 2];
    let mut itr: usize = 0;
    for stream in buffer {
        let num: u32 = stream.trim().parse().expect("not a number");
        arr[itr] = num;
        itr = itr + 1;
    }

    return arr;
}

fn main() {
    let arr: [u32; 2] = take_input();
    let num1 = arr[0];
    let num2 = arr[1];

    if num1 > num2 {
        println!("greater is {num1}");
    } else {
        println!("greater is {num2}");
    }
}
