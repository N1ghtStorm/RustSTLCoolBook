fn main() {
    //println!("Hello, world!");
    let pizza_config = PizzaConfig::default();
    println!("{:?}", pizza_config)
}

#[derive(Default)]
#[derive(Debug)]
struct PizzaConfig{
    wants_cheese: bool,
    number_of_olives: i32, 
    special_message: String,
    crust_type: CrustType
}

#[derive(Debug)]
enum CrustType {
 Thin,
 Thick
}

impl Default for CrustType {
    fn default() -> CrustType {
        CrustType::Thin
    }
}