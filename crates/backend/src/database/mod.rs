mod schema;
pub use schema::{
    Application, Interview, NewApplication, NewInterview, NewOffer, NewResume, Offer, Resume,
};

mod dao;
pub use dao::AppDB;
