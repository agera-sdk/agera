use crate::{display::*, entity::*, geom::*};

entity_type! {
    use agera = crate;
    pub struct DisplayObject: Entity {
        /// The opacity of a display object.
        pub alpha: f64 = 1.0,
        pub position: Vector2d = Vector2d::zero(),
        pub skew: Vector2d = Vector2d::zero(),
        pub filters: Vec<BitmapFilter> = vec![],
    }
}