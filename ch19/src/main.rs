fn main() {
    println!("Hello, world!");
}


trait Add<RHS=Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}

struct A {

}

impl Add for A {
    type Output = i32;
    fn add(self, rhs: Self) -> Self::Output {
        return 11
    }
}

fn return_fn() -> Box<dyn Fn(i32) -> i32>{
    Box::new(|x| x)
}