use std::time::{SystemTime, UNIX_EPOCH};

use crate::services::inventory_service::InventoryService;
use crate::Product;
// The services will be Singletons that contain the product lists.
#[derive(Debug, Clone)]
pub struct CartService {
    pub products: Vec<Product>,
    inventory_service: Box<InventoryService>,
    current: Box<CartService>,
    pub debug_name: String,
    option: Option<Box<CartService>>,
}

impl CartService {
    fn new() -> CartService {
        CartService {
            products: Vec::new(),
            inventory_service: Box::new(InventoryService::new()),
            // set debug_name as current time
            debug_name: format!(
                "{}",
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            ),
            current: Box::new(CartService::new()),
            option: None,
        }
    }

    pub fn is_none(&self) -> bool {
        self.option.is_none()
    }

    pub fn get_current(&self) -> &CartService {
        if self.current.is_none() {
            self.current = Box::new(CartService::new());
            self.option = Some(self.current.clone());
        }
        self.current.as_ref()
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
