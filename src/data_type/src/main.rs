fn main() {

    /*
     *iは符号付き整数、uは符号なし整数
     *例)
     *i8: -128 ~ 127
     *u8: 0 ~ 255
     *Rustの標準はi32
     */
    // let num_8: i8 = -129; エラー
    let num_8: u8 = 255;
    let num_32: i32 = 2_147_483_647;
    let num_by_hex = 0xff;
    println!("The value of num_8 is: {}", num_8);
    println!("The value of num_32 is: {}", num_32);
    println!("The value of num_by_hex is: {}", num_by_hex);
}
