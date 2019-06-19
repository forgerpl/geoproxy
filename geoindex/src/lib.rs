use geo::Point;
use rstar::{self, RTree};

use std::fmt::Debug;

use crate::entry::IndexEntry;
use crate::ty::{IndexCoordinate, IndexDefinition};

mod entry;
mod ty;

#[derive(Debug)]
pub struct GeoIndex<T: Debug, V: IndexCoordinate = f32> {
    index: RTree<IndexEntry<V>>,
    values: Vec<T>,
    default: T,
}

impl<T: Debug, V: IndexCoordinate> GeoIndex<T, V> {
    pub fn new(defs: IndexDefinition<T, V>, default: T) -> Self {
        let index = defs
            .iter()
            .enumerate()
            .flat_map(|(id, (polys, _))| polys.iter().map(move |poly| IndexEntry::from((poly, id))))
            .collect();

        let values = defs.into_iter().map(|(_, v)| v).collect();

        let index = RTree::bulk_load(index);

        Self {
            index,
            values,
            default,
        }
    }

    pub fn lookup_coords(&self, coords: Option<&Point<V>>) -> &T {
        coords
            .map(|coords| {
                self.index
                    .locate_all_at_point(&[coords.x(), coords.y()])
                    .filter_map(move |entry| {
                        if entry.contains(&coords) {
                            Some(&self.values[entry.value_index()])
                        } else {
                            None
                        }
                    })
                    .nth(0)
                    .unwrap_or(&self.default)
            })
            .unwrap_or(&self.default)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // we have to define these helpers, as floats don't impl Ord
    macro_rules! min {
        ($a: expr, $b: expr) => {
            if $a < $b {
                $a
            } else {
                $b
            }
        };
    }

    macro_rules! max {
        ($a: expr, $b: expr) => {
            if $a > $b {
                $a
            } else {
                $b
            }
        };
    }

    macro_rules! point {
        ($x: expr, $y: expr) => {
            Point::new($x, $y)
        };
    }

    macro_rules! rect {
        (f32 $x0: expr, $y0: expr, $x1: expr, $y1: expr) => {
            rect!($x0 as f32, $y0 as f32, $x1 as f32, $y1 as f32)
        };

        ($x0: expr, $y0: expr, $x1: expr, $y1: expr) => {{
            use geo::polygon;

            let x0 = min!($x0, $x1);
            let x1 = max!($x0, $x1);

            let y0 = min!($y0, $y1);
            let y1 = max!($y0, $y1);

            polygon![
                (x: x0, y: y0),
                (x: x0, y: y1),
                (x: x1, y: y1),
                (x: x1, y: y0),
            ]
        }};
    }

    fn simple_data() -> IndexDefinition<u32, f32> {
        let polygons_1 = vec![rect!(f32 0, 0, 10, 10), rect!(f32 10, 0, 20, 10)];
        let polygons_2 = vec![rect!(f32 0, 10, 10, 20), rect!(f32 10, 10, 20, 20)];

        vec![(polygons_1, 1), (polygons_2, 2)]
    }

    #[test]
    fn within_bounds() {
        let defs = simple_data();
        let db = GeoIndex::new(defs, 0);

        assert_eq!(db.lookup_coords(Some(&point!(5f32, 5f32))), &1);
        assert_eq!(db.lookup_coords(Some(&point!(15f32, 15f32))), &2);
    }

    #[test]
    fn fallback() {
        let defs = simple_data();
        let db = GeoIndex::new(defs, 0);

        assert_eq!(db.lookup_coords(Some(&point!(45f32, 5f32))), &0);
    }

    #[test]
    fn default() {
        let defs = simple_data();
        let db = GeoIndex::new(defs, 0);

        assert_eq!(db.lookup_coords(None), &0);
    }

    #[test]
    fn open() {
        use geo::polygon;

        let polygons = vec![polygon![
            (x: 0f32, y: 0f32),
            (x: 5f32, y: 5f32),
            (x: 5f32, y: 0f32),
        ]];
        let defs = vec![(polygons, 1)];

        let db = GeoIndex::new(defs, 0);

        assert_eq!(db.lookup_coords(Some(&point!(2f32, 1f32))), &1);
    }
}
