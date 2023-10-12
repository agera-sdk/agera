# Roadmap

## purplelight::application

## purplelight::display2d

## purplelight::ecs

The Entity-Component-System pattern used by Purplelight applications.

## purplelight::ui

Skinnable UI control foundation over `purplelight::display2d`.

- Set `renderedState = RenderedState::Outdated` for a control when its skin updates. The skin updates when a parent or the control itself gets its theme assigned a new value or a control's ECS component is added, removed or updated.
- Controls are rendered a certain frame if their rendered state is outdated.