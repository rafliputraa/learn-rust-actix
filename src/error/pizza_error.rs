use derive_more::Display;

#[derive(Debug, Display)]
pub enum PizzaError {
    NoPizzasFound,
    PizzaCreationFailure,
    NoSuchPizzaFound,
}