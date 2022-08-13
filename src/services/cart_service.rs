// The services will be Singletons that contain the product lists.
#[derive(Debug)]
pub struct CartService {
    pub products: Vec<Product>,
    inventory_service: InventoryService,
}

impl CartService {
    pub fn new(inventory_service: InventoryService) -> CartService {
        CartService {
            products: Vec::new(),
            inventory_service,
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

    // pub fn get_total_inventory(&self) -> f64 {
    //     let mut total_inventory = 0.0;
    //     for product in &self.products {
    //         total_inventory += product.count * product.inventory_service.get_count(product.id);
    //     }
    //     total_inventory
    // }

    // pub fn get_total_inventory_price(&self) -> f64 {
    //     let mut total_inventory_price = 0.0;
    //     for product in &self.products {
    //         total_inventory_price +=
    //             product.count * product.inventory_service.get_price(product.id);
    //     }
    //     total_inventory_price
    // }
}
