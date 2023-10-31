use std::{
    fmt::{Debug, Display},
    ops::{Mul, MulAssign},
};
use embed_doc_image::embed_doc_image;
use crate::geom::Vector2d;

/// Represents a two-dimensional transformation matrix that determines how to map points
/// from one coordinate space to another. You can perform various graphical transformations
/// on a display object by setting the properties of a `Matrix2d` object, applying that
/// `Matrix2d` object to the display object through its `set_transform_matrix()` method.
/// These transformation functions include translation (*x* and *y* repositioning), rotation,
/// scaling, and skewing.
/// 
/// Together these types of transformations are known as *affine* transformations.
/// Affine transformations preserve the straightness of lines while transforming, so
/// that parallel lines stay parallel.
/// 
/// A transformation matrix object is a 3 x 3 matrix with the following contents:
/// 
/// ![props1][matrix_props1.jpg]
/// 
/// In traditional transformation matrices, the *u*, *v*, and *w* properties provide extra
/// capabilities. The `Matrix2d` type can only operate in two-dimensional space, so it always
/// assumes that the property values *u* and *v* are 0.0, and that the property value
/// *w* is 1.0. The effective values of the matrix are as follows:
/// 
/// ![props2][matrix_props2.jpg]
/// 
/// # Transformations
/// 
/// `Matrix2d` supports four major types of transformations: translation, scaling,
/// rotation, and skewing. You can set three of these transformations by using specialized
/// methods.
/// 
/// Each transformation function alters the current matrix properties so that you can effectively
/// combine multiple transformations. To do this, you can call more than one transformation
/// function before applying the matrix to a display object (through its `set_transform_matrix()` method).
/// 
/// ## Translation (displacement)
/// 
/// The method `translate(t)` moves the image `t.x()` pixels to the right and `t.y()` pixels down.
/// 
/// The resulting matrix values are as follows:
/// 
/// ![translate][matrix_translate.jpg]
/// 
/// Display result:
/// 
/// ![translate][matrix_translate_image.jpg]
/// 
/// ## Scaling
/// 
/// The method `scale(s)` resizes the image, multiplying the location of each pixel by `s.x()` on the *x* axis and `s.y()` on the *y* axis.
/// 
/// The resulting matrix values are as follows:
/// 
/// ![scale][matrix_scale.jpg]
/// 
/// Display result:
/// 
/// ![scale][matrix_scale_image.jpg]
/// 
/// ## Rotation
///
/// The method `rotate(q)` rotates the image by an angle `q`, which is measured in radians.
/// 
/// The resulting matrix values are as follows:
/// 
/// ![rotate][matrix_rotate.jpg]
/// 
/// Display result:
/// 
/// ![rotate][matrix_rotate_image.jpg]
/// 
/// ## Skewing or shearing
/// 
/// Skewing or shearing can be done by setting the properties *b* and *c*,
/// which progressively slides the image in a direction parallel to the *x* or *y* axis.
/// The *b* property of the matrix represents the tangent of the skew angle along the *y* axis;
/// the *c* property of the matrix represents the tangent of the skew angle along the *x* axis.
/// 
/// The resulting matrix values are as follows:
/// 
/// ![skew][matrix_skew.jpg]
/// 
/// Display result:
/// 
/// ![skew][matrix_skew_image.jpg]
///
#[derive(Copy, Clone)]
#[embed_doc_image("matrix_props1.jpg", "src/geom/docs/assets/matrix_props1.jpg")]
#[embed_doc_image("matrix_props2.jpg", "src/geom/docs/assets/matrix_props2.jpg")]
#[embed_doc_image("matrix_translate.jpg", "src/geom/docs/assets/matrix_translate.jpg")]
#[embed_doc_image("matrix_translate_image.jpg", "src/geom/docs/assets/matrix_translate_image.jpg")]
#[embed_doc_image("matrix_scale.jpg", "src/geom/docs/assets/matrix_scale.jpg")]
#[embed_doc_image("matrix_scale_image.jpg", "src/geom/docs/assets/matrix_scale_image.jpg")]
#[embed_doc_image("matrix_rotate.jpg", "src/geom/docs/assets/matrix_rotate.jpg")]
#[embed_doc_image("matrix_rotate_image.jpg", "src/geom/docs/assets/matrix_rotate_image.jpg")]
#[embed_doc_image("matrix_skew.jpg", "src/geom/docs/assets/matrix_skew.jpg")]
#[embed_doc_image("matrix_skew_image.jpg", "src/geom/docs/assets/matrix_skew_image.jpg")]
pub struct Matrix2d {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    tx: f64,
    ty: f64,
}

impl Debug for Matrix2d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl Display for Matrix2d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(a={}, b={}, c={}, d={}, tx={}, ty={})", self.a(), self.b(), self.c(), self.d(), self.tx(), self.ty())
    }
}

impl Default for Matrix2d {
    /// Returns an identity `Matrix2d`. The identity matrix
    /// has `a = 1.0`, `b = 0.0`, `c = 0.0`, `d = 1.0`, `tx = 0.0`,
    /// `ty = 0.0`.
    /// 
    /// In matrix notation, the identity matrix looks like this:
    /// 
    /// ![Identity][matrix_identity.jpg]
    /// 
    #[embed_doc_image("matrix_identity.jpg", "src/geom/docs/assets/matrix_identity.jpg")]
    fn default() -> Self {
        Matrix2d { a: 1.0, b: 0.0, c: 0.0, d: 1.0, tx: 0.0, ty: 0.0 }
    }
}

impl Matrix2d {
    pub fn new(a: f64, b: f64, c: f64, d: f64, tx: f64, ty: f64) -> Self {
        Self { a, b, c, d, tx, ty }
    }

    /// The value that affects the positioning of pixels along the *x* axis
    /// when scaling or rotating an image.
    pub fn a(&self) -> f64 {
        self.a
    }
    /// The value that affects the positioning of pixels along the *x* axis
    /// when scaling or rotating an image.
    pub fn set_a(&mut self, value: f64) {
        self.a = value;
    }

    /// The value that affects the positioning of pixels along the *y* axis
    /// when rotating or skewing an image.
    pub fn b(&self) -> f64 {
        self.b
    }
    /// The value that affects the positioning of pixels along the *y* axis
    /// when rotating or skewing an image.
    pub fn set_b(&mut self, value: f64) {
        self.b = value;
    }

    /// The value that affects the positioning of pixels along the *x* axis
    /// when rotating or skewing an image.
    pub fn c(&self) -> f64 {
        self.c
    }
    /// The value that affects the positioning of pixels along the *x* axis
    /// when rotating or skewing an image.
    pub fn set_c(&mut self, value: f64) {
        self.c = value;
    }

    /// The value that affects the positioning of pixels along the *y* axis
    /// when scaling or rotating an image.
    pub fn d(&self) -> f64 {
        self.d
    }
    /// The value that affects the positioning of pixels along the *y* axis
    /// when scaling or rotating an image.
    pub fn set_d(&mut self, value: f64) {
        self.d = value;
    }

    /// The distance by which to translate each point along the *x* axis.
    pub fn tx(&self) -> f64 {
        self.tx
    }
    /// The distance by which to translate each point along the *x* axis.
    pub fn set_tx(&mut self, value: f64) {
        self.tx = value;
    }

    /// The distance by which to translate each point along the *y* axis.
    pub fn ty(&self) -> f64 {
        self.ty
    }
    /// The distance by which to translate each point along the *y* axis.
    pub fn set_ty(&mut self, value: f64) {
        self.ty = value;
    }

    /// Includes parameters for scaling, rotation, and translation. When applied to a matrix
    /// it sets the matrix's values based on those parameters.
    /// 
    /// Using the `create_box()` method lets you obtain the same matrix as you would if you applied
    /// the `identity()`, `rotate()`, `scale()`, and `translate()` methods in succession. For example,
    /// `mat1.create_box(&Vector2d(2.0, 2.0), PI / 4.0, Vector2d(10.0, 20.0))` has the same effect
    /// as the following:
    /// 
    /// ```ignore
    /// mat1.identity();
    /// mat1.rotate(PI / 4.0);
    /// mat1.scale(&Vector2d(2.0, 2.0));
    /// mat1.translate(&Vector2d(10.0, 20.0));
    /// ```
    pub fn create_box(&mut self, scale: &Vector2d, rotation_radians: f64, translation: &Vector2d) {
        self.identity();
        self.rotate(rotation_radians);
        self.scale(&scale);
        self.translate(&translation);
    }

    /// Creates the specific style of matrix expected by the `begin_gradient_fill()` and `line_gradient_style()`
    /// methods of the `Shape` display object. Width and height are scaled to a `scale_x`/`scale_y` pair and the
    /// `translation.x()`/`translation.y()` values are offset by half the width and height.
    ///
    /// For example, consider a gradient with the following characteristics:
    /// 
    /// * `gradient.is_linear()`
    /// * Two colors, green and blue, with the ratios array set to `vec![0, 255]`
    /// * `SpreadMethod::Pad`
    /// * `InterpolationMethod::LinearRgb`
    ///
    /// The following illustrations show gradients in which the matrix was defined using
    /// the `create_gradient_box()` method with different parameter settings:
    /// 
    /// # Ilustration 1
    /// 
    /// `create_gradient_box()` settings:
    /// 
    /// ```ignore
    /// size = Vector2d(25, 25);
    /// rotation_radians = 0;
    /// translate = Vector2d(0, 0);
    /// ```
    /// 
    /// Resulting gradient:
    /// 
    /// ![Gradient][create_gradient_box_1.jpg]
    /// 
    /// # Ilustration 2
    /// 
    /// `create_gradient_box()` settings:
    /// 
    /// ```ignore
    /// size = Vector2d(25, 25);
    /// rotation_radians = 0;
    /// translate = Vector2d(25, 0);
    /// ```
    /// 
    /// Resulting gradient:
    /// 
    /// ![Gradient][create_gradient_box_2.jpg]
    /// 
    /// # Ilustration 3
    /// 
    /// `create_gradient_box()` settings:
    /// 
    /// ```ignore
    /// size = Vector2d(50, 50);
    /// rotation_radians = 0;
    /// translate = Vector2d(0, 0);
    /// ```
    /// 
    /// Resulting gradient:
    /// 
    /// ![Gradient][create_gradient_box_3.jpg]
    /// 
    /// # Ilustration 4
    /// 
    /// `create_gradient_box()` settings:
    /// 
    /// ```ignore
    /// size = Vector2d(50, 50);
    /// rotation_radians = PI / 4.0; // 45 degrees
    /// translate = Vector2d(0, 0);
    /// ```
    /// 
    /// Resulting gradient:
    /// 
    /// ![Gradient][create_gradient_box_4.jpg]
    /// 
    #[embed_doc_image("create_gradient_box_1.jpg", "src/geom/docs/assets/create_gradient_box_1.jpg")]
    #[embed_doc_image("create_gradient_box_2.jpg", "src/geom/docs/assets/create_gradient_box_2.jpg")]
    #[embed_doc_image("create_gradient_box_3.jpg", "src/geom/docs/assets/create_gradient_box_3.jpg")]
    #[embed_doc_image("create_gradient_box_4.jpg", "src/geom/docs/assets/create_gradient_box_4.jpg")]
    pub fn create_gradient_box(&mut self, size: &Vector2d, rotation_radians: f64, translation: &Vector2d) {
        self.identity();
        self.rotate(rotation_radians);
        self.scale(&size);
        self.translate(&((*size / 2.0) + *translation));
    }

    /// Given a point in the pretransform coordinate space, returns the coordinates
    /// of that point after the transformation occurs. Unlike the standard transformation
    /// applied using the `transform_point()` method, the `delta_transform_point()` method's
    /// transformation does not consider the translation parameters `tx` and `ty`.
    /// 
    /// # Parameters
    /// 
    /// * `point` — The point for which you want to get the result of the matrix transformation.
    ///
    pub fn delta_transform_point(&mut self, point: &Vector2d) -> Vector2d {
        Vector2d(
            self.a() * point.x() + self.c() * point.y(),
            self.b() * point.x() + self.d() * point.y(),
        )
    }

    /// Sets each matrix property to a value that causes a null transformation.
    /// An object transformed by applying an identity matrix will be identical
    /// to the original.
    /// 
    /// After calling the `identity()` method, the resulting matrix has the following
    /// properties: `a = 1.0`, `b = 0.0`, `c = 0.0`, `d = 1.0`, `tx = 0.0`,
    /// `ty = 0.0`.
    /// 
    /// In matrix notation, the identity matrix looks like this:
    /// 
    /// ![Identity][matrix_identity.jpg]
    /// 
    #[embed_doc_image("matrix_identity.jpg", "src/geom/docs/assets/matrix_identity.jpg")]
    pub fn identity(&mut self) {
        self.set_a(1.0);
        self.set_b(0.0);
        self.set_c(0.0);
        self.set_d(1.0);
        self.set_tx(0.0);
        self.set_ty(0.0);
    }

    /// Performs the opposite transformation of the original matrix.
    /// You can apply an inverted matrix to an object to undo the transformation
    /// performed when applying the original matrix.
    pub fn invert(&mut self) {
        let norm = self.a() * self.d() - self.b() * self.c();
        if norm == 0.0 {
            self.set_a(0.0);
            self.set_b(0.0);
            self.set_c(0.0);
            self.set_d(0.0);
            self.set_tx(-self.tx());
            self.set_ty(-self.ty());
        } else {
            let norm = 1.0 / norm;
            let a1 = self.d() * norm;
            self.set_d(self.a() * norm);
            self.set_a(a1);
            self.set_b(self.b() * -norm);
            self.set_c(self.c() * -norm);

            let tx1 = -self.a() * self.tx() - self.c() * self.ty();
            self.set_ty(-self.b() * self.tx() - self.d() * self.ty());
            self.set_tx(tx1);
        }
    }

    /// Applies a rotation transformation to the matrix.
    /// 
    /// The `rotate()` method alters the `a`, `b`, `c`, and `d` properties.
    /// In matrix notation, this is the same as multiplying the current matrix with the
    /// following matrix:
    /// 
    /// ![rotate][matrix_rotate.jpg]
    ///
    #[embed_doc_image("matrix_rotate.jpg", "src/geom/docs/assets/matrix_rotate.jpg")]
    pub fn rotate(&mut self, rotation_radians: f64) {
        let cos = f64::cos(rotation_radians);
        let sin = f64::sin(rotation_radians);

        let new_a = self.a() * cos - self.b() * sin;
        let new_b = self.a() * sin + self.b() * cos;
        let new_c = self.c() * cos - self.d() * sin;
        let new_d = self.c() * sin + self.d() * cos;
        let new_tx = self.tx() * cos - self.ty() * sin;
        let new_ty = self.tx() * sin + self.ty() * cos;

        self.set_a(new_a);
        self.set_b(new_b);
        self.set_c(new_c);
        self.set_d(new_d);
        self.set_tx(new_tx);
        self.set_ty(new_ty);
    }

    /// Applies a scaling transformation to the matrix. The *x* axis is multiplied
    /// by `scale.x()` and the *y* axis is multiplied by `scale.y()`.
    /// 
    /// The `scale()` method alters the `a` and `d` properties of the matrix.
    /// In matrix notation, this is the same as multiplying the current matrix with
    /// the following matrix:
    /// 
    /// ![scale][matrix_scale.jpg]
    /// 
    #[embed_doc_image("matrix_scale.jpg", "src/geom/docs/assets/matrix_scale.jpg")]
    pub fn scale(&mut self, scale: &Vector2d) {
        let new_a = self.a() * scale.x();
        let new_b = self.b() * scale.y();
        let new_c = self.c() * scale.x();
        let new_d = self.d() * scale.y();
        let new_tx = self.tx() * scale.x();
        let new_ty = self.ty() * scale.y();
        self.set_a(new_a);
        self.set_b(new_b);
        self.set_c(new_c);
        self.set_d(new_d);
        self.set_tx(new_tx);
        self.set_ty(new_ty);
    }

    /// Returns the result of applying the geometric transformation represented by
    /// the matrix to the specified point.
    pub fn transform_point(&mut self, point: &Vector2d) -> Vector2d {
        self.delta_transform_point(point) + Vector2d(self.tx, self.ty)
    }

    /// Translates the matrix along the *x* and *y* axes.
    pub fn translate(&mut self, translation: &Vector2d) {
        self.set_tx(self.tx() + translation.x());
        self.set_ty(self.ty() + translation.y());
    }

    /*
    fn copy_from_array(&mut self, array: &[[f64; 3]; 3]) {
        self.set_a(array[0][0]);
        self.set_b(array[0][1]);
        self.set_c(array[1][0]);
        self.set_d(array[1][1]);
        self.set_tx(array[2][0]);
        self.set_ty(array[2][1]);
    }

    fn to_nalgebra_matrix(&self) -> nalgebra::base::Matrix3<f64> {
        nalgebra::base::Matrix3::new(self.a(), self.b(), 0.0, self.c(), self.d(), 0.0, self.tx(), self.ty(), 1.0)
    }
    */
}

impl Mul for Matrix2d {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.a() * rhs.a(), self.b() * rhs.b(), self.c() * rhs.c(), self.d() * rhs.d(), self.tx() * rhs.tx(), self.ty() * rhs.ty())
    }
}

impl MulAssign for Matrix2d {
    fn mul_assign(&mut self, rhs: Self) {
        self.set_a(self.a() * rhs.a());
        self.set_b(self.b() * rhs.b());
        self.set_c(self.c() * rhs.c());
        self.set_d(self.d() * rhs.d());
        self.set_tx(self.tx() * rhs.tx());
        self.set_ty(self.ty() * rhs.ty());
    }
}

#[cfg(test)]
mod tests {
    use crate::geom::*;
    use std::f64::consts::PI;

    #[test]
    fn test_transform_point() {
        let mut matrix = Matrix2d::default();
        matrix.identity();
        matrix.rotate(PI / 4.0);
        matrix.scale(&Vector2d(2.0, 2.0));
        matrix.translate(&Vector2d(10.0, 20.0));

        println!(
            "matrix\n\
            - Got: {matrix}\n\
            - Expected approximation = (a=1.4142135623730951, b=1.414213562373095, c=-1.414213562373095, d=1.4142135623730951, tx=10, ty=20)"
        );

        println!(
            "matrix.delta_transform_point(&Vector2d(0.0, 0.0))\n\
            - Got: {}\n\
            - Expected approximation: (x=0, y=0)",
            matrix.delta_transform_point(&Vector2d(0.0, 0.0))
        );

        println!(
            "matrix.delta_transform_point(&Vector2d(1.0, 1.0))\n\
            - Got: {}\n\
            - Expected approximation: (x=2.220446049250313e-16, y=2.82842712474619)",
            matrix.delta_transform_point(&Vector2d(1.0, 1.0))
        );

        println!(
            "matrix.transform_point(&Vector2d(0.0, 0.0))\n\
            - Got: {}\n\
            - Expected approximation: (x=10, y=20)",
            matrix.transform_point(&Vector2d(0.0, 0.0))
        );

        println!(
            "matrix.transform_point(&Vector2d(1.0, 1.0))\n\
            - Got: {}\n\
            - Expected approximation: (x=10, y=22.82842712474619)",
            matrix.transform_point(&Vector2d(1.0, 1.0))
        );

        println!(
            "matrix.transform_point(&Vector2d(128.0, 56.0))\n\
            - Got: {}\n\
            - Expected approximation: (x=111.82337649086287, y=280.2152954766495)",
            matrix.transform_point(&Vector2d(128.0, 56.0))
        );
    }

    #[test]
    fn test_invert() {
        let mut matrix = Matrix2d::default();
        matrix.identity();
        matrix.rotate(PI / 4.0);
        matrix.scale(&Vector2d(2.0, 2.0));
        matrix.translate(&Vector2d(10.0, 20.0));
        matrix.invert();
        println!(
            "matrix\n\
            - Got: {matrix}\n\
            - Expected approximation = (a=0.3535533905932738, b=-0.35355339059327373, c=0.35355339059327373, d=0.3535533905932738, tx=-10.606601717798213, ty=-3.535533905932738)"
        );
    }
}