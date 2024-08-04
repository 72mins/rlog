mod home_handler;
mod not_found_handler;
mod about_handler;
mod blog_handler;
mod post_handler;
mod contact_handler;

pub use home_handler::index;
pub use not_found_handler::not_found;
pub use about_handler::about;
pub use blog_handler::blog;
pub use post_handler::blog_post;
pub use contact_handler::contact;
