fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        //長方形の面積は{}平方ピクセルです
        "The aread of th rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
