use authentication::{find_even, greet_user, login, read_line, LoginAction};
// authentication is the library work here
fn main() {
    let numbers=vec![1,2,3,4,5];
    let result=find_even(&numbers);
    match result{
        Some(value)=>println!("Found even numver"),
        None=>println!("No even number found"),
    }
    // println!("{}",greet_user("Mosely"));
    let mut tries=0;

    loop{
        println!("Enter your username");
        let username:String=read_line();
        println!("Enter your password");
        let password:String=read_line();
        // LoginAction::Granted(authentication::LoginRole::Admin) =>  println!("Admin"),
        match login(&username, &password){
             Some(LoginAction::Granted(role))=>{
                match role{
                    authentication::LoginRole::Admin=>println!("Admin"),
                    authentication::LoginRole::User=>println!("User")
                     

                }
                break;
             }
             Some(LoginAction::Denied)=>{
                // Do nothing
             }
             None=>{
                println!("New user system")
             }
        }
        // if login(&username, &password) {
        //     println!("Welcome");
        //     break
        // }
        // we are not now return true or false
        // pattern matching heavily compile time
        // else{
            println!("Incorrect username or password");
            tries+=1;
            if tries>=3{
                println!("too many failed logins");
                break;
            }
        }
    }


