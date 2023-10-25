# Plans

## Agera application project

Agera application projects will be created through the Agera command line interface. The generated application projects have the following structure characteristics:

- The application descriptor, `agera-application.json`, contains metadata that sets Agera application settings, such as the application ID.
- The Cargo build script (`build.rs`) executes Agera build bootstrap code.
- The application entry point (`src/main.rs`) executes Agera bootstrap code.

## agera::application

Centralization for application settings and entry points, including:

- the user interface's root control;
- `Input`

## agera::display

Two-dimensional display featuring a main `Stage` type that includes:

- `fit = "optimal"` option, indicating that the stage scales and centers to the screen using the optimal scale ratio that fits the stage to the screen (`Math.min(horizontalRatio, verticalRatio)`, `horizontalRatio = screen.size.x / stage.size.x`, `verticalRatio = screen.size.y / stage.size.y`).

## agera::geom

Provides basic geometry types, such as `Vector`. It is used instead of `nalgebra` for ease of use.

## agera::controls

Skinnable user interface controls foundation over `agera::display`.

- Set `renderedState = "outdated""` for a control when its skin updates. The skin updates when a parent or the control itself gets its theme assigned a new value or when a control's ECS component has been updated (added, changed or removed, which the ECS module is already ready for handling).
- Controls are rendered at a certain frame if their rendered state is outdated.

## agera::skins

Work with skinning of the user interface.

## agera::themes

Work with themes for the user interface.