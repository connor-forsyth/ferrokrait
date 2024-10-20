pub mod color;
pub mod vec2;
pub mod vec3;
pub mod vec4;
pub mod input;
pub mod traits;

pub mod all {
    pub use super::color::*;
    pub use super::vec2::*;
    pub use super::input::*;
    pub use super::traits::*;
    pub use super::vec3::*;
    pub use super::vec4::*;
}
