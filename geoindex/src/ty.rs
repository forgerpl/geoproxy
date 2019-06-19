use geo::{CoordinateType, Polygon};
use num_traits::{Bounded, Float, Signed};

use std::fmt::Debug;

pub type IndexDefinition<T, V = f32> = Vec<(Vec<Polygon<V>>, T)>;

/// Marker trait for index coordinate values
pub trait IndexCoordinate: CoordinateType + Bounded + Signed + Float + Debug {}

impl<T> IndexCoordinate for T where T: CoordinateType + Bounded + Signed + Float + Debug {}
