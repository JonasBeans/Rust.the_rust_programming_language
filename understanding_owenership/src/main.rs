fn main() {
    let mut previous: u32 = 1;
    let mut current: u32  = 1;
    println!("{previous}");
    for _ in 0..10 {
        next_in_sequence(&mut previous, &mut current);
    }
}

fn next_in_sequence(previous: &mut u32, current: &mut u32) {
    let next = *previous + *current;
    println!("{}", next);
    *previous = *current;
    *current = next;
}