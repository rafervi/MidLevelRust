use std::fs::File;

fn main() {
    let file = File::open("example.txt").expect("Failed to open file"); //we can also use unwrap
   /*  let file = match file{
        Ok(file) => file,
        Err(error) =>{
            panic!("Failed to open file: {:?}", error);
        }
    };*/
}

/*fn get_user_id(username: &str) -> Result<i32, String> {
    if username.is_empty() {
        Err("Username can not be empty".to_owned())
    } else {
        Ok(1)
    }

}*/
