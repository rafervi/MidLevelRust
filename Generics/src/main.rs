struct BrowserCommand<T> {
    name: String, // name of the command
    payload: T, // payload associated of the command
}

impl<T> BrowserCommand<T> {
    fn new(name: String, payload: T) -> Self {
        BrowserCommand { 
            name,
             payload,
        }
    }
    fn get_payload(&self) -> &T { //&T because T is generic
        &self.payload
    }
}
//we need to declare our generic twice.
//First: implementating funcionatity to the BrowserCommand on any type
//rather than in other type

impl BrowserCommand<String>{
    fn print_payload(&self){
        println!("{}", self.payload); //we know it is going to print the payload
        //it's because it's going to be a string 
    }
} 

fn main() {
    println!("Hello, world!");
    let cmd1 = BrowserCommand::new(
        "navigate".to_owned(), 
        "https://www.letsgetrusty.com".to_owned(),
    );

    let cmd2 = BrowserCommand::new(
        "zoom".to_owned(),
        200 //200% zoom
    );
    cmd1.print_payload();
    //cmd2.print_payload();// will not print because its a int not a string
    let p1 = cmd1.get_payload();
    let p2 = cmd2.get_payload();

    serialize_payload(p1);
    serialize_payload(p2);

    //serialize_payload_string(p1);
    //serialize_payload_i32(p2);
}

/*enum Option<T>{
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/
fn serialize_payload<T>(payload: &T) -> String {
    //convert payload to JSON string
    "placeholder".to_owned()
}
/*fn serialize_payload_string(payload: &String) -> String {
    //convert payload to JSON string
    "placeholder".to_owned()
}
fn serialize_payload_i32(payload: &i32) -> String {
    //convert payload to JSON string
    "placeholder".to_owned()
}*/