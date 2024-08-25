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

    /// Creates a bounding hierarchy given a vector, reordering elements in the process.
    /// Will panic if given an empty vector.
    pub fn from_vec(geometry: &mut [WorldObject]) -> Arc<Self> {
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
                let left = Self::from_vec(&mut geometry[..midpoint]);
                let right = Self::from_vec(&mut geometry[midpoint..]);

                Arc::new(Self {
                    bounds: left.bounds.union(&right.bounds),
                    data: HierarchyNode::Tree(left, right),
                })
            }
        }
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
