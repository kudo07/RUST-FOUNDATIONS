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

pub fn login(username:&str,password:&str)->bool{
    username.to_lowercase()=="admin" && password=="password"
}

pub fn read_line()->String{
    let mut input:String=String::new();
    std::io::stdin().read_line(&mut input).expect("Std not working");
    input.trim().to_string()
}

pub enum LoginAction{
    Admin,
    User,
    Denied
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
        assert!(login("admin","password"));
        assert!(!login("admin","Password"));
        assert!(!login("Admin","Password"));
    }
}