use std::mem::replace;

use crate::services::cart_service::CartService;

pub struct CartWrapper {
    cart_service: Option<CartService>,
}

impl CartWrapper {
    #[inline]
    pub fn get_current_cart(&mut self) -> CartService {
        let c = replace(&mut self.cart_service, None);
        c.unwrap()
    }
}

static mut CARTWRAPPER: CartWrapper = CartWrapper {
    cart_service: Some(CartService {
        products: Vec::new(),
        cart_name: "Cart".to_string(),
    }),
};
