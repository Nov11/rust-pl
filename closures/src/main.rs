fn main() {
    println!("Hello, world!");
    // unimplemented!();
    let mut vec = vec![1, 2, 3];
    // vec = move_around(vec);
    let v2 = vec;
    // dbg!(vec);

    default_move2();
}
#[derive(Copy, Clone)]
struct Label {
    number: i32,
}

fn default_ref() {
    let mut magic_num = 5;
    let magic_johnson = 32;
    let plus_magic = |x: i32| x + magic_num;
    let more_magic = &mut magic_num; // Err!
    println!("{}", magic_johnson); // Ok!
}

fn default_move() {
    let numbers = vec![2, 5, 32768];
    let alphabet_soup = || {
        numbers;
        vec!['a', 'b']
    };
    // ^ throw away unneeded ingredients
    alphabet_soup();
    // alphabet_soup(); // use of moved value
}

fn default_move2() {
    let numbers = vec![2, 5, 32768];
    let alphabet_soup = move || println!("{:?}", numbers);
    alphabet_soup();
    alphabet_soup(); // Delicious soup
    // println!("{:?}", numbers) not workable with this line
}
