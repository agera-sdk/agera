/*!
The core functionality of the Purplelight platform.
*/

pub mod app;

/// The Entity-Component-System pattern which
/// Purplelight uses as the main way of running
/// applications, graphics and simulations.
///
/// # Overview
///
/// - A **system** queries a set of entities and performs
///   operations based on them.
/// - **Components**, generally viewed as data or alternative to structure fields,
///   can be attached to an **entity**. Every component structure
///   can be attached at most once. For example,
///   a `LinearVelocity` can be attached to an entity at most once.
/// - The `Entity` structure is merely an identifier.
///
/// # Hierarchy
/// 
/// - Entities can be spawned as _children_ of another entity.
/// - Certain components are actions that alter child entities.
///   For example, the `RemoveChild` component can be
///   attached to an entity to request the runtime
///   to remove a child entity from its parent entity at the next frame.
/// - Components can be inherited from parent entities
///   while a system runs.
///
/// # Entity Paths
///
/// Attaching the `Name` component to an entity allows it to be
/// found through an _entity path_ based on the entity tree.
/// Entity paths use slashes as separators and
/// `..` portions allow to ascend when walking the tree.
/// 
pub mod ecs;