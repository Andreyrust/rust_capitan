use std::io;
fn main(){
    let name_capitan = "xxx";
    let password = "X";
    let mut gradient = String::new();
    let fuel = 3000;
    let mut cap = String::new();
    let mut pass = String::new();
    //спроби введення імені
    let mut attempts = 0;
    let name_corect = loop {
    attempts += 1;
    println!("enter your name:");
    cap.clear();
    io::stdin().read_line(&mut cap).unwrap();
    let cap = cap.trim();
        if name_capitan == cap{ break true;}
        println!("wrong name!");
        if attempts == 5 {break false;}
    };
    if !name_corect {
        println!("access denided!");
        return;
    }
    //спроби введення пароля
    let mut attempts_pass = 0;
    let password_correct = loop {
        attempts_pass += 1;
        println!("enter the password");
        pass.clear();
        io::stdin().read_line(&mut pass).unwrap();
        let pass = pass.trim();
        if password == pass{
        break true;
       }
       println!("wrong password");
       if attempts_pass == 5 {break false;}
        
    };
        if !password_correct {
            println!("access denided!");
            return;
        }
    //вітання
    println!("hallo{}", name_capitan);
      //введення палива
       println!("enter fuel in %:");
       gradient.clear();
       io::stdin().read_line(&mut gradient).unwrap();
       let mut gradient:u32 = gradient. trim().parse().unwrap();
       //розрахунок кількості палива
    let current_fuel = fuel / 100 * gradient;
    if current_fuel >= 2800 {
    println!("🚀 start!");}
    else {println!("no fuel!!");}
}