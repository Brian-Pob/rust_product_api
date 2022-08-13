#[readonly::make]
#[derive(Debug)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub count: f64,
}

impl Product {
    // new is the constructor
    pub fn new(id: i32, name: String, description: String, price: f64, count: f64) -> Product {
        Product {
            id,
            name,
            description,
            price,
            count,
        }
    }

    // setters
    pub fn set_id(&mut self, id: i32) {
        self.id = id;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn set_price(&mut self, price: f64) {
        self.price = price;
    }

    pub fn set_count(&mut self, count: f64) {
        self.count = count;
    }

    // getters
    // pub fn get_id(&self) -> i32 {
    //     self.id
    // }

    // pub fn get_name(&self) -> String {
    //     self.name.clone()
    // }

    // pub fn get_description(&self) -> String {
    //     self.description.clone()
    // }

    // pub fn get_price(&self) -> f64 {
    //     self.price
    // }

    // pub fn get_count(&self) -> f64 {
    //     self.count
    // }

    pub fn get_total_price(&self) -> f64 {
        self.price * self.count
    }
}
