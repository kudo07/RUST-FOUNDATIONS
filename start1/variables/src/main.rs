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
        // this will return n as 6
    };

    println!("{n}");
}
 