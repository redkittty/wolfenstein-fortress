use std::io;
fn main() {
    let error = format!("That is not an option");
    println!("Wolfenstein Fortress v0.0.1");
    println!("Licensed under GNU General Public License v3.0 (GPLv3)");
    println!("---------------");
    // Stats
    let mut health: i8 = 100;
    let mut money: f64 = 0.00;
    let mut ammo: i64 = 0; 
    let _stats = format!("Health = {health}, Money = {money}, Ammo = {ammo}:");
    println!("Backstory:");
    println!("You have been captured by Nazi Soldiers after your squad's surrendered");
    println!("You have been stuck in your cell for 3 days already");
    println!("Your goal is to escape Wolfenstein Fortress and hook back up with the Army");
    println!("---------------");
    println!("A guard comes into your cell to bring you to an interrogation room,");
    println!("You see his back is turned");
    println!("What will you do?");
    println!("(1) = Kill the Guard, (2) = Do nothing");
    let mut a = String::new();
    io::stdin()
        .read_line(&mut a)
        .expect("Failed to read line");
    let a: i8 = a.trim().parse().expect("Please insert a number");
    if a == 1 || a == 2 {
        if a == 1 {
            println!("You fight with the guard,");
            println!("You manage to achieve victory,");
            println!("You got hurt a little bit but now you have a pistol and have 8 bullets");
            ammo += 8;
            health -= 10;
            println!("---------------");
            println!("Health = {health}, Money = {money}, Ammo = {ammo}:");
        }
        if a == 2 {
            println!("You decide to do nothing");
            println!("The guard leads you into the interrogation room");
            println!("The interrogator asks you what the Allies plans are");
            println!("You see a guard next to you");
            println!("What will you do?");
            println!("(1) = Do nothing, (2) = Snitch, (3) = Attack the Guard");
            let mut b = String::new();
            io::stdin()
                .read_line(&mut b)
                .expect("Failed to read line");
            let b: i8 = b.trim().parse().expect("Please insert a number");
            if b == 1 || b == 2 || b == 3 {
                if b == 1 {
                    println!("The interrogator gets angry at you");
                    println!("The interrogator starts to beat you");
                    println!("It hurts a bit");
                    println!("The Guard sends you back to your cell to rot");
                    health -= 5;
                    println!("---------------");
                    println!("Health = {health}. Money = {money}, Ammo = {ammo}:");
                }
                if b == 2 {
                    println!("The interrogator is happy with you");
                    println!("He gives you $5");
                    println!("The Allies lose the next battle");
                    println!("The new prisioners are mad at you and beat you up");
                    println!("The guards stop this and give you some medicine");
                    money += 5.00;
                    health -= 10;
                    health += 5;
                    println!("---------------");
                    println!("Health = {health}, Money = {money}, Ammo = {ammo}:");
                }
                if b == 3 {
                    println!("You decide to attack the guard");
                    println!("Unfortunately, This isn't a good time to do so");
                    println!("Because the interrogator sees this and shoots you");
                    println!("---------------");
                    println!("GAME OVER");
                }
            }
            else {
                println!("{}", error);
            }
        }
    }
    else {
        println!("{}", error);
    }
}
