use std::time::{SystemTime, UNIX_EPOCH};

use crate::services::inventory_service::InventoryService;
use crate::Product;

#[derive(Debug, Clone)]
pub struct CartService {
    products: Vec<Product>,
    cart_name: String,
    // inventory_service: InventoryService,
}

impl CartService {
    pub fn new() -> CartService {
        CartService {
            products: Vec::new(),
            cart_name: "Cart".to_string(),
            // inventory_service: InventoryService::new(),
        }
    }

    pub fn add_product(&mut self, product: Product) {
        self.products.push(product);
    }

    pub fn remove_product(&mut self, product: Product) {
        self.products.retain(|p| p.id != product.id);
    }

    pub fn get_products(&self) -> Vec<Product> {
        self.products.clone()
    }

    pub fn get_total_price(&self) -> f64 {
        let mut total_price = 0.0;
        for product in &self.products {
            total_price += product.price * product.count;
        }
        total_price
    }

    pub fn get_total_count(&self) -> f64 {
        let mut total_count = 0.0;
        for product in &self.products {
            total_count += product.count;
        }
        total_count
    }
}
