fn double(n:i32)->i32{
    n*2
    // it like return n*2
}

fn double_or_nothing(n:i32)->i32{
    if n>0 {
        return n*2;
    }
    else{
        return 0;
    }
}

fn greet(s:String)->String{
    println!("Hello {s}");
    s
}

// dont give me variable ,give me access to variable 
fn greet_borrow(s:&String){
    println!("{s}");
}

fn greet_borrow_mut(s:&mut String){
    *s=format!("Dello this is  {s}");
}

fn main() {
    let n:i32=5;

    let n:i32=5;
    let n:i32=n+1;
    // now n is the most recent one
    // n completely shaodwed the other n's
    println!("{n}");
    {
        // different scope
        let n:i32=10;
        println!("{n}");
        // everythin rust returns a tyep whether explicitly mentions or not

    }
    let n={
        let n=6;
        n
        // dont use semicolon
        // this will return n as 6
    };

    println!("{n}");

    let m =double(5);
    println!("{m}");
    let aux=if m==32{
        5
    }else{
        8
    };
    println!("{aux}");

    let name:String="Hello".to_string();
    let name=greet(name);
    // i cannot greet again 
    // let same = greet(name);
    //  OWNERSHIP one var can have right to own that thing
    // ownership back to here down
    // print!("{name}") //borrowed to move to same thats why thro erorr
    // to make it right we use clone
    let same:String=greet(name.clone());
    let name:String=greet(name.clone());

    println!("{same} {name}");//working fine, deep copy of string , give thst copy to the string
    // clone is slow
    let name_new:String="Hello".to_string();
    greet_borrow(&name_new);
    greet_borrow(&name_new);
    // want to mutability on my borrow
    let mut name_new_ver:String="Hello".to_string();
    greet_borrow_mut(&mut name_new_ver);
    println!("{name_new_ver}");
    greet_borrow_mut(&mut name_new_ver);
    println!("{name_new_ver}")

}
 