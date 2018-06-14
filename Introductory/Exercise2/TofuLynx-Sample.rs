fn main() {

let mult_tab_num: i32 = 7;

println!("==== MULTIPLICATION TABLE OF {:?} ====", mult_tab_num);

let mut mult_table: [(i32, i32, i32); 10] = [(0, 0, 0); 10];

for x in 0..10 {
    mult_table[x] = ( mult_tab_num, (x as i32) + 1, mult_tab_num * ((x as i32) + 1) );
}

for x in 0..10 {
    println!("| {:?}\tx {:>7?}\t= {:>7?} |", mult_table[x].0, mult_table[x].1, mult_table[x].2);
}

println!("===================================");

}