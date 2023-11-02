use crate::{display::*, entity::*, geom::*};

entity_type! {
    use agera = crate;
    pub struct DisplayObject: Entity {
        /// The opacity of a display object.
        pub alpha: f64 = 1.0,
        /// Position relative to the parent.
        pub position: Vector2d = Vector2d::zero(),
        pub skew: Vector2d = Vector2d::zero(),
        pub filters: Vec<BitmapFilter> = vec![],
        pub scale: Vector2d = Vector2d(1.0, 1.0),
        pub rotation_radians: f64 = 0.0,
        pub rotation_x_degrees: f64 = 0.0,
        pub rotation_y_degrees: f64 = 0.0,
        pub rotation_z_degrees: f64 = 0.0,

        /// Indicates the registration point of the display object,
        /// used when positioning and rotating the object.
        /// Each component is in the range between 0 and 1.
        /// A component value of 0.5 means center.
        pub registration_point: Vector2d = Vector2d::zero(),

        /// An optional transformation matrix.
        pub transform_matrix: Option<Matrix2d> = None,
    }
}

impl DisplayObject {
    pub fn rotation_degrees(&self) -> f64 {
        self.rotation_radians().to_degrees()
    }

    pub fn set_rotation_degrees(&self, value: f64) -> Self {
        self.set_rotation_radians(value.to_radians());
        self.clone()
    }

    /*
    /// Position relative to the root display object.
    pub fn global_position(&self) -> Vector2d {
        if let Some(p) = self.parent() {
            if let Ok(p) = p.to::<DisplayObject>() {
                return p.global_position() + self.position();
            }
        }
        self.position()
    }

    /// Sets the position relative to the root display object.
    /// Calling this method is equivalent to `set_position(value - parent_global_position)`.
    pub fn set_global_position(&self, value: Vector2d) -> Self {
        if let Some(p) = self.parent() {
            if let Ok(p) = p.to::<DisplayObject>() {
                self.set_position(value - p.global_position());
                return self.clone();
            }
        }
        self.set_position(value);
        self.clone()
    }
    */
}