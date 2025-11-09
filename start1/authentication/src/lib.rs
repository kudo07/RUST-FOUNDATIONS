// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

pub fn greet_user(name:&str)->String{
    format!("Hello {name}")
}
// fn for login

// pub fn login(username:&str,password:&str)->bool{
//     username.to_lowercase()=="admin" && password=="password"
// }
// make another enumerations

#[derive(PartialEq,Debug)]
pub enum LoginAction{
    Granted(LoginRole),
    Denied
}

#[derive(PartialEq,Eq,Debug)]
pub enum LoginRole{
    Admin,
    User,
    // Denied
}

pub fn login(username:&str, password:&str)->Option<LoginAction>{
    let username=username.to_lowercase();
    if username != "admin" && username != "bob"{
        return None;
    }
    if username=="admin" && password=="password"{
        Some(LoginAction::Granted(LoginRole::Admin))
    }
    else if username == "bob" && password == "password"{
        Some(LoginAction::Granted(LoginRole::User))
    }
    else {
        Some(LoginAction ::Denied)
    }
}
pub fn read_line()->String{
    let mut input:String=String::new();
    std::io::stdin().read_line(&mut input).expect("Std not working");
    input.trim().to_string()
}



#[cfg(test)]

mod tests{
    use super::*;

    use crate::greet_user;
    #[test]

    fn test_greet_user(){
        assert_eq!("Hello Mosely",greet_user("Mosely"))
    }
    #[test]
    fn test_login(){
        // assert!(login("admin","password"));
        // assert!(!login("admin","Password"));
        // assert!(!login("Admin","Password"));
        // assert_eq!(login("admin","password"),LoginAction::Admin);
        // assert_eq!(login("bob","password"),LoginAction::User);
    //      assert_eq!(login("admin", "wrong"), LoginAction::Denied);
    // assert_eq!(login("wrong", "password"), LoginAction::Denied); 
    }
}