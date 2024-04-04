use std::io;
fn main() {
    println!("Wolfenstein Fortress v0.0.2-alpha 3");
    println!("Copyright (C) 2024  Jason Kirkpatrick");
    println!("Licensed under GNU General Public License v3.0 (GPLv3)");
    println!("Note: The Game is currently under heavy construction");
    println!("Some functionality might not be implemented yet");
    println!("---------------");
    // Stats
    let mut health: i8 = 100;
    let mut money: f64 = 0.00;
    let mut ammo: i64 = 0;
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
            println!("You got hurt a little bit but now you have a knife, pistol, and 8 bullets");
            ammo += 8;
            health -= 10;
            println!("---------------");
            println!("Health = {health}, Money = {money}, Ammo = {ammo}:");
            println!("---------------");
            println!("Another Guard heard you kill the Guard assigned to your cell");
            println!("He arrives at your cell and about to shoot you");
            println!("What will you do?");
            println!("(1) = Shoot the Guard, (2) = Stab the Guard, (3) = Do Nothing");
            let mut b = String::new();
            io::stdin()
                .read_line(&mut b)
                .expect("Failed to read line");
            let b: i8 = b.trim().parse().expect("Please insert a number");
            if b == 1 || b == 2 || b == 3 {
                if b == 1 {
                    println!("You shoot the guard");
                    println!("You used 3 bullets to kill him");
                    println!("The Guard shot you as well");
                    println!("You got hurt a little bit");
                    println!("You get 8 bullets");
                    health -= 20;
                    ammo += 5;
                    println!("---------------");
                    println!("Health = {health}, Money = {money}, Ammo = {ammo}:");
                    println!("---------------");
                }
                if b == 2 || b == 3 {
                    println!("You got shot by the guard,");
                    println!("GAME OVER");
                }
            }
            else {
                println!("That is not an option");
            }
        }
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
                    health -= 5;
                    println!("---------------");
                    println!("Health = {health}, Money = {money}, Ammo = {ammo}:");
                    println!("---------------");
                    println!("You go to the prision shop");
                    println!("You see 2 items:");
                    println!("What will you buy");
                    println!("(1) = Paper Clip $5.00, (2) = Bread $3.00");
                    let mut c = String::new();
                    io::stdin()
                        .read_line(&mut c)
                        .expect("Failed tp read line");
                    let c: i8 = c.trim().parse().expect("Please insert a number");
                    if c == 1 || c == 2 {
                        if c == 1 {
                            money -= 5.00;
                            println!("---------------");
                            println!("Health = {health}, Money = {money}, Ammo = {ammo}");
                            println!("---------------");
                            println!("You decide to buy the Paper Clip");
                            println!("You remember a method that the Army taught you to lock pick");
                            println!("You are in your cell");
                            println!("What will you do?");
                            println!("(1) = Lock Pick, (2) = Do Nothing");
                            let mut d = String::new();
                            io::stdin()
                                .read_line(&mut d)
                                .expect("Failed to read line");
                            let d: i8 = d.trim().parse().expect("Please insert a number");
                            if d == 1 || d == 2 {
                               if d == 1 {
                                  println!("You decide to pick the lock");
                                  println!("You eventually pick the lock");
                                  println!("You step outside the cell");
                                  println!("You start walking");
                                  println!("A guard spots you");
                                  println!("What will you do?");
                                  println!("(1) = Kill the Guard, (2) = Return to your cell, (3) = Do Nothing");
                                  let mut e = String::new();
                                  io::stdin()
                                      .read_line(&mut e)
                                      .expect("Failed to read line");
                                  let e: i8 = e.trim().parse().expect("Please insert a number");
                                  if e == 1 || e == 2 || e == 3 {
                                      if e == 1 {
                                          println!("You decide to kill the Guard");
                                          println!("You eventually succeed but you are badly hurt");
                                          println!("You though get a pistol, knife, and 5 rounds");
                                          health -= 50;
                                          ammo += 5;
                                          println!("---------------");
                                          println!("Health = {health}, Money = {money}, Ammo = {ammo}");
                                          println!("---------------");
                                          println!("You see another guard chasing after you");
                                          println!("What will you do?");
                                          println!("(1) = Shoot the Guard, (2) = Stab the Guard, (3) = Surrender");
                                          let mut f = String::new();
                                          io::stdin()
                                              .read_line(&mut f)
                                              .expect("Failed to read line");
                                          let f: i8 = f.trim().parse().expect("Please insert a number");
                                          if f == 1 || f == 2 || f == 3 {
                                            if f == 1 {
                                              println!("You decide to shoot the guard,");
                                              println!("The guard also shoots you");
                                              println!("You are very badly hurt,");
                                              println!("You eventually win and get 6 rounds");
                                              health -= 30;
                                              ammo += 6;
                                              println!("---------------");
                                              println!("Health = {health}, Money = {money}, Ammo = {ammo}:");
                                              println!("---------------");
                                            }
                                            if f == 2 || f == 3 {
                                              println!("The guard kills you");
                                              println!("GAME OVER");
                                            }
                                          }
                                          else {
                                              println!("That is not an option");

                                          }
                                      }
                                      if e == 2 {
                                        println!("Unfinished");
                                      }
                                      if e == 3 {
                                        println!("Unfinished");
                                      }
                                  }
                               }
                               if d == 2 {
                                 println!("Unfinished");
                               }
                                  else {
                                      println!("That is not an option");
                                  }
                            }
                        }
                    }
                    else {
                        println!("That is not an option");

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
                println!("That is not an option");
            }
        }
    }
    else {
        println!("That is not an option");
    }
}
