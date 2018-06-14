//----Chekhov----
//henriquejtneves@gmail.com
//What It Does: Today, Primitives!
//To create executable, run: rustc NAME.rs


/* A brief Lesson on Scalar Types
signed integers: i8, i16, i32, i64 and isize (pointer size)
unsigned integers: u8, u16, u32, u64 and usize (pointer size)
floating point: f32, f64
char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
bool either true or false
and the unit type (), whose only possible value is an empty tuple: () */

fn main() {
    let pi: f64 = 3.141592653589793238462643383279502884197169399375105820974944;
    let mut multiplication_values: [(i8,f64);11] = [(0,0.0);11];
    println!("Multiplication Table for {:.6}", pi);
    for x in 1..11  {
        multiplication_values[x] = (x as i8, (x as f64)*pi) ;
        println!("pi*{:>4} = {:2>.6}",x,multiplication_values[x].1 as f32);
    } 
}