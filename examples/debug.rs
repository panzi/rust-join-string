use join_string::Join;

#[derive(Debug)]
struct MyStruct {
    #[allow(unused)]
    value: u32
}

fn main() {
    println!("{:?}", ['a', 'b', 'c'].join(", "));
    println!("{:?}", [
        &MyStruct { value: 1 },
        &MyStruct { value: 2 },
        &MyStruct { value: 3 },
    ].join(", "));
}
