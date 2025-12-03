
fn main() {
    println!("{} days", 31);
    println!("{1} this is {0}", "Alice", "Bob");
    println!("{subject} {verb}", verb="jumps over", subject="the fox");
    // 
    println!("Base 10: {}", 69420);
    println!("Base 2: {:b}", 69420);
    println!("Base 8: {:o}", 69420);
    println!("Base 16: {:x}", 69420);
    // 
    println!("{:>5}",1);
    println!("{:0>5}",1);
    println!("{num:0<5}",num=1);
    println!("{num:0<width$}", num=1, width=5);
    //
    #[allow(dead_code)]
    struct Structure(i32);
    //
    let num: f64 = 1.0;
    let width: usize = 5;
    println!("{num:>width$}");
}