use  std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
   // println!("Hello, world!");

   loop {
 
    let secret_number = rand::thread_rng().gen_range(1, 101);
  
    println!("Secret guess :{}", secret_number);


    println!("Please enter guess name");

  
      let mut guess : String = String::new();
  
  
       io::stdin()
           .read_line(&mut guess)
           .expect("Failed to read input");
  
  
          let guess: u32 = match  guess.trim().parse()  {
                 Ok(num) => num,
              Err(_) => {
                println!("this is not a valid");
                continue;
              }
          };
  
         

          println!("Your guess: {}", guess);
  
  
      match guess.cmp(&secret_number){
          Ordering::Less => println!("{}","Too Small !".red()),
          Ordering::Greater => println!("{}","Too Big !".red()),
          Ordering::Equal =>{ 
            
            println!("{}","You Win!".green());
            break;

        },
      }
  
  
  
  
         
   }


}
