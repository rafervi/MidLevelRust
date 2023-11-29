fn main() {
    /* {
        let s1 = String::from("Let's get Rusty!");
    }*/ 
    //s1 ends in line 4
    let r1;
   {
        let s1 = String::from("Let's get Rusty!");
        r1=&s1;
        println!("r1: {r1}");
    }

    let mut s1 = String::from("Let's get Rusty");
    let r1 = &s1;
    println!("r1: {r1}");
    let r2 = &mut s1;
    r2.push_str("!");
    //println!("r1: {r1}"); moving to line 18 gives back an error because the lifetime of the inmutable r1
    println!("r2: {r2}");
    
    
}
//concrete lifetime is the time which a value exists at a particular memory location

