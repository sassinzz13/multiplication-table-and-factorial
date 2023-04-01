use std::io;
pub fn main(){
  //print ln for enter an integer
   println!("Enter an Integer: ");
   
   //readline for console input
   let mut input = String::new();
   io::stdin().read_line(&mut input).expect("Not a valid string");
   let n: i32 = input.trim().parse().expect("Not a valid number");
   println!("{}", n );

   //if else statement logic
   if n % 2 == 0 {
    println!("Multiplication table of {n} : ");
     let mut i =  1;
     while i <= 10 {
      let mut results = n*i;
      println!("{n} x {i} = {results}");
      i+=1;
     } 
   }else{
    println!("Factorial of {n} :");
    let mut i = 1;
    let mut fact = 1;
    while (i<=n){
      fact *= i;
      i+=1;
    }
    println!("{fact}")
   }
}