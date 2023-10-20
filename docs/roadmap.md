# Roadmap

## purplelight::application

Some centralization for the application, including:

- the ECS's root entity;
- the user interface's root `Entity`;
- `Input`

## purplelight::display

Two-dimensional display featuring a main `Stage` type that includes:

- `fit = "optimal"` option, indicating that the stage scales and centers to the screen using the optimal scale ratio that fits the stage to the screen (`Math.min(horizontalRatio, verticalRatio)`, `horizontalRatio = screen.size.x / stage.size.x`, `verticalRatio = screen.size.y / stage.size.y`).

## purplelight::ecs

The entity-component-system pattern used by Purplelight applications.

## purplelight::geom

Provides basic geometry types, such as `Vector`. It is used instead of `nalgebra` for ease of use.

## purplelight::ui

Skinnable UI control foundation over `purplelight::display2d`.

- Set `renderedState = "outdated""` for a control when its skin updates. The skin updates when a parent or the control itself gets its theme assigned a new value or when a control's ECS component has been updated (added, changed or removed, which the ECS module is already ready for handling).
- Controls are rendered at a certain frame if their rendered state is outdated.