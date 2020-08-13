use crate::types::CartProduct;
use crate::route::Route;
use yew::prelude::*;
use yew_router::components::RouterAnchor;

pub struct Navbar {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub cart_products: Vec<CartProduct>,
}

impl Component for Navbar {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        type Anchor = RouterAnchor<Route>;

        let cart_value = self
            .props
            .cart_products
            .iter()
            .fold(0.0, |acc, cp| acc + (cp.quantity as f64 * cp.product.price));

        html! {
            <div class="navbar">
                <Anchor route=Route::HomePage>
                <div class="navbar_title">{"RustMart"}</div>
                </Anchor>
                <div class="navbar_cart_value">{format!("${:.2}", cart_value)}</div>

            </div>
        }
    }
}
