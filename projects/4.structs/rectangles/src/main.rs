fn main() {
    let width = 40;
    let height = 60;
    
    println!{
        "직사각형의 면적은 {}입니다.",
        area(width, height)
    };
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}