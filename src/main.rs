mod models; // This declares the module to be used.
pub use models::*; // This enables an optional shortcut

fn main() {
    let mut product = Product::new(
        1,
        "Product 1".to_string(),
        "Description 1".to_string(),
        1.0,
        1.0,
    );
    product.set_id(2);
    product.set_name("Product 2".to_string());
    product.set_description("Description 2".to_string());
    product.set_price(2.0);
    product.set_count(2.0);
    product.get_total_price();
    println!("{:?}", product);
}
