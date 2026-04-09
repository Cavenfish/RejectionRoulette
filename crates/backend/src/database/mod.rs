mod schema;
pub use schema::{Application, Interview, NewApplication, NewInterview, NewOffer, Offer};

mod dao;
pub use dao::AppDB;
