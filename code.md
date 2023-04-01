Translate to rust
Converted on 01/04/2023 at 9:47 pm

-----------------------------------Java code-----------------------------------------------
import java.util.Scanner;

public class MultiplicationTableOrFactorial {
    public static void main(String[] args) {
        Scanner input = new Scanner(System.in);

        System.out.print("Enter an integer: ");
        int n = input.nextInt();

        if (n % 2 == 0) {
            System.out.println("Multiplication table of " + n + ":");
            int i = 1;
            while (i <= 10) {
                System.out.println(n + " x " + i + " = " + (n * i));
                i++;
            }
        } else {
            System.out.println("Factorial of " + n + ":");
            int i = 1;
            int fact = 1;
            while (i <= n) {
                fact *= i;
                i++;
            }
            System.out.println(fact);
        }
    }
}
--------------------------------------Rust Code--------------------------------------------

use std::io;
pub fn main(){
 
   println!("Enter an Integer: ");
   
   let mut input = String::new();
   io::stdin().read_line(&mut input).expect("Not a valid string");
   let n: i32 = input.trim().parse().expect("Not a valid number");
   println!("{}", n );

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