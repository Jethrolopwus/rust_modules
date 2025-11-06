
// privacy in modules

use array_tool::vec::*;
use exporting_and_privacy::{Customer, Category, Order, Product};

fn main() {
  
let product = Product::new(1, String::from("Laptop"), 799.99, Category::Electronics);
let customer = Customer::new(1, String::from("James"), String::from("James@gmail.com"));

let order = Order::new(1, product, customer, 2);
println!("Total cost of Order: ${:.2}", order.total_bill());


//external dependencies//
let product1 = Product::new(1, String::from("Laptop"), 799.99, Category::Electronics);
let product2 = Product::new(2, String::from("T-shirt"), 20.0, Category::Clothing);
let product3 = Product::new(3, String::from("Book"), 10.0, Category::Books);


let set1: Vec<&Product> = vec![&product1, &product2];
let set2: Vec<&Product> = vec![&product2, &product3];

//calling the intersect method 

let intersection = set1.intersect(set2);
println!("The intersection is: {:?} ", intersection);




    

}
