
fn main() {
    let int_max: i32 = 0x7FFF_FFFF_i32;
    let int_min: i32 = -0x8000_0000_i32;
    println!("max int:{}",int_max);
    println!("min int:{}", int_min);
    let tup: (i32, u32, char) = (100_i32, 200_u32, 'a');
    let (x, y, z) = tup;
    println!("x:{}, y:{}, z:{}", x, y, z);
    let a1: [i32; 5] = [1,2,3,4,5];
    let a2 = [5_i32; 10];
    println!("a1:{:?}", a1);
    println!("a2:{:?}", a2);

}
