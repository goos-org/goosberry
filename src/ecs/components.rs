use downcast_rs::Downcast;
use nalgebra::{Quaternion, Vector2, Vector3};
use num_traits::Float;
use std::fmt::Debug;

pub trait Component: Downcast + Debug + Send + Sync {}
impl<T> Component for T where T: Downcast + Debug + Send + Sync {}

#[derive(Default, Debug, Clone)]
pub struct Transform3<T: 'static + Float + Debug> {
    pub position: Vector3<T>,
    pub rotation: Quaternion<T>,
    pub scale: Vector3<T>,
}

#[derive(Default, Debug, Clone)]
pub struct Transform2<T: 'static + Float + Debug> {
    pub position: Vector2<T>,
    pub rotation: T,
    pub scale: Vector2<T>,
}
