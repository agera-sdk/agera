# agera::display progress track

## Geometry

* [x] `Vector2d`
* [x] `Matrix2d`

## Rendering

Rendering a display object takes parent inherited fields such as alpha and returns a `BitmapData`. It is not decided yet if it should be done in the GPU or CPU.

## Colors

`agera::display` uses `agera::util::Color` to represent colors, which additionally includes an `alpha` channel.

## DisplayObject

* [ ] Alpha
* [ ] Position
* [ ] Skew
* [ ] Filters
  - Blur, color multiplier, shadow
* [ ] Scale (a non-uniform `Vector2d`)
* [ ] Rotation (`rotation_degrees()` and `rotation_radians()` as well as `set_`)
* [ ] Rotation X/Y/Z for 3D rotation
* [ ] Registration point (`registration_point()`, `set_registration_point()`)
  - Each component of the point must be in the range 0 to 1. A value of 0.5 means center.
* [ ] `transform_matrix`
  - If set, defines the transformation matrix for this display object, overriding all other transformation fields (position, rotation, scale, skew).

## Controls

* [ ] AgeraControlDelegate for types that are controls
  - [ ] Implement `AsRef<AgeraControl>`
  - [ ] Implement `AsRef<DisplayObject>`
  - [ ] Implement `AsRef<Entity>`
  - [ ] Implement `Deref<DisplayObject>`

## Shape

`Shape` should support fill and stroke operations. Additionally, text filling should be supported.

```rust
use agera::{display::*, geom::*};
let shape = Shape::new();
shape.clear()
    .begin_fill("green".parse().unwrap())
    .draw(position, Rectangle(Vector2d(0.0, 0.0), Vector2d(100.0, 100.0)))
    .draw(position, Text::new("Text").set_font_family("Verdana"))
    .move_to(position)
    .draw_arc(radius)
    .end_fill();
```

* [ ] `shape.clear()`
  - [ ] Clear graphics commands
  - [ ] Clear fill and line style settings
* [ ] `shape.draw_round_rect(position, rectangle, corner_radius)`
* [ ] `shape.draw_circle(position, radius)`
* [ ] `shape.draw_graphics_data(graphics_data)` (takes a `Vec<GraphicsData>`)
* [ ] `Draw<T>` trait with a `.draw()` method
  - [ ] `Draw<Rectangle>`
  - [ ] `Draw<Text>`
  - [ ] `Draw<Ellipse>`
* [ ] Gradient line style (`shape.line_gradient_style(...)`)
  - [ ] Linear
  - [ ] Radial
* [ ] Solid line style (`shape.line_style(...)`)
* [ ] Gradient fill (`shape.begin_gradient_fill(...)`)
  - [ ] Linear
  - [ ] Radial
* [ ] Bitmap fill (`shape.begin_bitmap_fill(...)`)
* [ ] Solid fill (`shape.begin_fill(...)`)
* [ ] `shape.cubic_curve_to(control_1_position, control_2_position, anchor_position)`
* [ ] `shape.curve_to(control_position, anchor_position)`
* [ ] `GraphicsData`
  - An enumeration consisting of command variants resulting from line style, fill style and path calls in `Shape`.
* [ ] `GraphicsData::from_svg(string)`
  - Returns `Result<Vec<GraphicsData>, SvgParseError>`
- [ ] `Gradient`
  - Enumeration consisting of `Linear` and `Radial` variants, with common methods that applies to both variants such as `colors()` and `ratios()`.

## Bitmap

