mod models; // This declares the module to be used.
pub use models::*; // This enables an optional shortcut
use std::any::Any;

fn main() {
    let mut product =
        Base_Product::new(1, "Product 1".to_string(), "Description 1".to_string(), 1.0);
    println!("{:#?}", product);

    product.set_id(2);
    product.set_name("Product 2".to_string());
    product.set_description("Description 2".to_string());
    product.set_price(2.0);
    println!("{}", product.get_data());
    println!("{:#?}", product);

    // Now testing child classes
    let mut weight_product = Weight_Product::new(
        product.id.clone(),
        product.name.clone(),
        product.description.clone(),
        product.price.clone(),
        1.0,
    );
    println!("{:#?}", weight_product);

    weight_product.set_id(3);
    weight_product.set_name("Product 3".to_string());
    weight_product.set_description("Description 3".to_string());
    weight_product.set_price(3.0);
    weight_product.set_weight(3.0);
    println!("{:#?}", weight_product);

    let mut quantity_product = Quantity_Product::new(
        7,
        "Product 7".to_string(),
        "Description 7".to_string(),
        7.0,
        69,
    );
    println!("{:#?}", quantity_product);

    quantity_product.set_id(8);
    println!("{:#?}", quantity_product);

    // Now testing polymorphic vector of products
    // https://stackoverflow.com/questions/25818082/vector-of-objects-belonging-to-a-trait
    // let mut products: Vec<Box<dyn Product>> = Vec::new();
    // products.push(Box::new(weight_product));
    // products.push(Box::new(quantity_product));
    // println!("{:?}", products);

    // let mut dyn_list: Vec<Box<dyn Debug>> = Vec::new();
    // dyn_list.push(Box::new(weight_product));
    // dyn_list.push(Box::new(quantity_product));
    // println!("{:#?}", dyn_list);

    // Note to self: Any is a trait that is implemented by all types.
    // In the docs, it is not the same as T, which should be the type of the object.
    // Any is literally a trait of its own.
    let mut dyn_list: Vec<Box<dyn Any>> = Vec::new();
    dyn_list.push(Box::new(weight_product.clone()));
    dyn_list.push(Box::new(quantity_product.clone()));

    for product in dyn_list {
        if let Some(product) = product.downcast_ref::<Weight_Product>() {
            // println!("{:#?}", product);
            println!("{}", product.weight);
        } else if let Some(product) = product.downcast_ref::<Quantity_Product>() {
            // println!("{:#?}", product);
            println!("{}", product.quantity);
        } else {
            println!("Unknown product type");
        }
    }

    // Now we're in business! 😎

    // Now attempting to use serde.
    println!("{}", serde_json::to_string(&product).unwrap());
    println!("{}", serde_json::to_string(&weight_product).unwrap());
    println!("{}", serde_json::to_string(&quantity_product).unwrap());

    // Q: What happens if you try to deserialize a mismatched type?
    // let q_json = serde_json::to_string(&quantity_product).unwrap();
    // let w_product: Weight_Product = serde_json::from_str(&q_json).unwrap();
    // println!("{:#?}", w_product);
    // A: Rust will panic.
}
