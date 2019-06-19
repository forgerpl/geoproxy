use geo::{
    algorithm::{bounding_rect::BoundingRect, contains::Contains},
    Point, Polygon,
};
use rstar::{self, PointDistance, RTreeObject, AABB};

use crate::ty::IndexCoordinate;

#[derive(Debug, PartialEq)]
pub(crate) struct IndexEntry<V: IndexCoordinate = f32> {
    envelope: AABB<[V; 2]>,
    polygon: Polygon<V>,
    value_index: usize,
}

impl<V> RTreeObject for IndexEntry<V>
where
    V: IndexCoordinate,
    [V; 2]: rstar::Point,
{
    type Envelope = AABB<[V; 2]>;

    fn envelope(&self) -> Self::Envelope {
        self.envelope
    }
}

impl<V> PointDistance for IndexEntry<V>
where
    V: IndexCoordinate,
    [V; 2]: rstar::Point,
{
    fn distance_2(
        &self,
        point: &<Self::Envelope as rstar::Envelope>::Point,
    ) -> <<Self::Envelope as rstar::Envelope>::Point as rstar::Point>::Scalar {
        self.envelope.distance_2(point)
    }
}

impl<V> IndexEntry<V>
where
    V: IndexCoordinate,
    [V; 2]: rstar::Point,
{
    fn envelope_from_polygon(poly: &Polygon<V>) -> AABB<[V; 2]> {
        let bb = poly
            .bounding_rect()
            .expect("Cannot calculate bounding box of a polygon");
        let min = bb.min;
        let max = bb.max;

        // convert to rstar types
        let rmin = [min.x, min.y];
        let rmax = [max.x, max.y];

        AABB::from_corners(rmin, rmax)
    }

    pub fn value_index(&self) -> usize {
        self.value_index
    }

    pub fn contains(&self, point: &Point<V>) -> bool {
        self.polygon.contains(point)
    }
}

impl<'a, V> From<(&'a Polygon<V>, usize)> for IndexEntry<V>
where
    V: IndexCoordinate,
    [V; 2]: rstar::Point,
{
    fn from((polygon, value_index): (&'a Polygon<V>, usize)) -> Self {
        let envelope = Self::envelope_from_polygon(polygon);

        Self {
            envelope,
            polygon: polygon.clone(),
            value_index,
        }
    }
}
