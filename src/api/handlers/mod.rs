pub mod health;
pub mod methods;
pub mod prayer_times;

pub use health::health_check;
pub use methods::get_methods;
pub use prayer_times::{calculate_prayer_times, get_prayer_times, get_prayer_times_range};