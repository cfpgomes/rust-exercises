//----Chekhov----
//henriquejtneves@gmail.com
//What It Does: Printing "Hello, World" and variations thereof
//To create executable, run: rustc nome.rs

fn main(){
    //1st Part, Simple Println
    println!("Hello, World!");
    //2nd Part, Using Formated Print
    println!("Hallo, {0}!","Kumpel");
    //3rd Part, Using Formated Print with Variations in Arguments 
    println!("{firstWord}, {who} !",
            firstWord =  "Hello",
            who = "pal");
    //4th Part, Let's use some numbers and see what goes
    println!("I hate from {} {1} of every {2:b} people.", 1, 2,3); 
    //kiddos. the numbers aren't exactly...cartesian stfu
    //in fact, :b is my 2 and somehow 10 (binary for 2)
    //5th Part, Now suppose I'm a very orderly person, as in...crazy
    println!("{number:>width$}",
            number = 1,
            width = 6); //This outputs a line with width 6 and a value, number 1
    //You may even repeat and add zeroes if you are that kind of mean
    println!("{number:>0width$}",
            number = 1,
            width = 6); //Now count my cards and tell me I'm wrong!
    //6th Part, I Do Not Understand, Really
    #[allow(dead_code)]
    struct Structure(i32);
    //To which we try:
    //println!("This struct `{}` won't print...", Structure(3));
    //And won't work. We print out because I can't even fathom what's going on.

    //What if one of these doesn't work? Well, we are professionals, right? fds your jokes are on point
    eprintln!("Error: Grab your Duck and make yourself a cup of coffee.");

}   