use super::*;
use geometry::Geometry;
use std::sync::Arc;

#[derive(Clone)]
pub struct BoundingHierarchy {
    bounds: BoundingBox,
    data: HierarchyNode,
}

#[derive(Clone)]
pub enum HierarchyNode {
    Leaf(WorldObject),
    Tree(Arc<BoundingHierarchy>, Arc<BoundingHierarchy>),
}

#[derive(Debug, Clone, Copy)]
pub enum Heuristic {
    RandomAxis,
    LongestAxis,
    SurfaceArea,
}

impl BoundingHierarchy {
    pub fn new(geometry: &WorldObject) -> Arc<Self> {
        Arc::new(Self {
            bounds: geometry.bounding_box(),
            data: HierarchyNode::Leaf(geometry.clone()),
        })
    }

    pub fn pair(left: &WorldObject, right: &WorldObject) -> Arc<Self> {
        let bounds = left.bounding_box().union(&right.bounding_box());

        Arc::new(Self {
            bounds,
            data: HierarchyNode::Tree(Self::new(left), Self::new(right)),
        })
    }

    /// Creates a bounding hierarchy given a vector, reordering elements in the process according to the
    /// surface area heuristic.
    ///
    /// Will panic if given an empty vector.
    #[inline]
    pub fn from_vec(geometry: &mut [WorldObject]) -> Arc<Self> {
        Self::from_vec_with_heuristic(geometry, Heuristic::SurfaceArea)
    }

    /// Creates a bounding hierarchy given a vector and a heuristic, reordering elements in the process.
    /// Will panic if given an empty vector.
    #[inline]
    pub fn from_vec_with_heuristic(
        geometry: &mut [WorldObject],
        heuristic: Heuristic,
    ) -> Arc<Self> {
        match heuristic {
            Heuristic::RandomAxis => Self::from_vec_with_random(geometry),
            Heuristic::LongestAxis => Self::from_vec_with_longest(geometry),
            Heuristic::SurfaceArea => Self::from_vec_with_surface_area(geometry),
        }
    }

    fn from_vec_with_random(geometry: &mut [WorldObject]) -> Arc<Self> {
        match geometry.len() {
            0 => panic!("There should not be a scene with 0 objects"),
            1 => Self::new(&geometry[0]),
            2 => Self::pair(&geometry[0], &geometry[1]),
            _ => {
                geometry.sort_by(|obj_a, obj_b| {
                    let box_a = obj_a.bounding_box();
                    let box_b = obj_b.bounding_box();

                    box_a.compare_axis(&box_b, random::random_index(3))
                });

                let midpoint = geometry.len() / 2;
                let left = Self::from_vec_with_random(&mut geometry[..midpoint]);
                let right = Self::from_vec_with_random(&mut geometry[midpoint..]);

                Arc::new(Self {
                    bounds: left.bounds.union(&right.bounds),
                    data: HierarchyNode::Tree(left, right),
                })
            }
        }
    }

    fn from_vec_with_longest(geometry: &mut [WorldObject]) -> Arc<Self> {
        match geometry.len() {
            0 => panic!("There should not be a scene with 0 objects"),
            1 => Self::new(&geometry[0]),
            2 => Self::pair(&geometry[0], &geometry[1]),
            _ => {
                let bounds = geometry.bounding_box();

                geometry.sort_by(|obj_a, obj_b| {
                    let box_a = obj_a.bounding_box();
                    let box_b = obj_b.bounding_box();

                    box_a.compare_axis(&box_b, bounds.longest_axis())
                });

                let midpoint = geometry.len() / 2;
                let left = Self::from_vec_with_longest(&mut geometry[..midpoint]);
                let right = Self::from_vec_with_longest(&mut geometry[midpoint..]);

                Arc::new(Self {
                    bounds,
                    data: HierarchyNode::Tree(left, right),
                })
            }
        }
    }

    pub fn from_vec_with_surface_area(geometry: &mut [WorldObject]) -> Arc<Self> {
        match geometry.len() {
            0 => panic!("There should not be a scene with 0 objects"),
            1 => Self::new(&geometry[0]),
            2 => Self::pair(&geometry[0], &geometry[1]),
            _ => {
                let (axis, midpoint) = Self::evaluate_best_area_split(geometry);

                geometry.sort_by(|obj_a, obj_b| {
                    let box_a = obj_a.bounding_box();
                    let box_b = obj_b.bounding_box();

                    box_a.compare_axis(&box_b, axis)
                });

                let left = Self::from_vec_with_surface_area(&mut geometry[..midpoint]);
                let right = Self::from_vec_with_surface_area(&mut geometry[midpoint..]);

                Arc::new(Self {
                    bounds: left.bounding_box().union(&right.bounding_box()),
                    data: HierarchyNode::Tree(left, right),
                })
            }
        }
    }

    fn calculate_split_area(geometry: &[WorldObject]) -> Float {
        let mut area = 0.0;
        for elem in geometry.iter() {
            area += elem.bounding_box().surface_area();
        }

        area
    }

    fn evaluate_best_area_split(geometry: &mut [WorldObject]) -> (usize, usize) {
        let mut best_split = 0;
        let mut best_axis = 0;
        let mut best_cost = Float::INFINITY;

        for axis in 0..3 {
            for split in 1..geometry.len() - 1 {
                geometry.sort_by(|obj_a, obj_b| {
                    let box_a = obj_a.bounding_box();
                    let box_b = obj_b.bounding_box();

                    box_a.compare_axis(&box_b, axis)
                });

                let left = &geometry[..split];
                let right = &geometry[split..];

                let total_cost = split as Float * Self::calculate_split_area(left)
                    + (geometry.len() - split) as Float * Self::calculate_split_area(right);

                if total_cost < best_cost {
                    best_axis = axis;
                    best_split = split;
                    best_cost = total_cost;
                }
            }
        }

        (best_axis, best_split)
    }
}

impl Geometry for BoundingHierarchy {
    fn collide(&self, ray: &Ray, t_range: Range) -> Option<geometry::Collision> {
        let mut range = match self.bounds.check_intersection(ray, t_range) {
            Some(range) => range,
            None => return None,
        };

        match &self.data {
            HierarchyNode::Leaf(geometry) => geometry.collide(ray, t_range),
            HierarchyNode::Tree(left, right) => {
                let left_collision = left.collide(ray, t_range);
                if let Some(ref collision) = left_collision {
                    range.1 = collision.t;
                }

                let right_collision = right.collide(ray, range);

                // Order is important here: since right uses a smaller range of t based on where left hit,
                // right sees things in front of left and therefore should be returned preferentially
                right_collision.or(left_collision)
            }
        }
    }

    fn bounding_box(&self) -> BoundingBox {
        self.bounds
    }
}
