fn main() {
     let mut number = 3;

     while number != 0 {
         println!("{number}!");

         number -= 1;
     }

     println!("LIFTOFF!!!");


     let a = [10, 20, 30, 40, 50];
     let mut index = 0;

     println!("---while loop---");

     while index < 5 {
         println!("the value is: {}", a[index]);
         index += 1;
     }

     println!("---for loop---");

     for element in a {
         println!("The value is: {element}");
     }

     println!("---for loop with range---");

     for element in (1..4).rev() {
         println!("{element}!");
     }
     println!("LIFTOFF!!!");
}
