fn main() {
    let player1 = String::from("player1");//player1 lifetime from this line to line 8
    let player2 = String::from("player2");//player2 lifetime from this line to line 8
    /*let player1 = String::from("player1");
    {
        let player2 = String::from("player2");
        let result = first_turn(player1.as_str(), player2.as_str());
        println!("Player going first is: {}", result);
    } player1 lifetime > player2 lifetime
     */

    let result = first_turn(player1.as_str(), player2.as_str());

    println!("Player going first is: {}", result);
}

fn first_turn<'a>(p1: &str, p2: &str) -> &'static str {// 'a lifetime parameter
   let s: &'static str = "Let's get Rusty"; //lifetime until the end of the program
   s
   //works because s lifetime is greater or equal to p1 lifetime
}
