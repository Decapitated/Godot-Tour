# Godot Tour
A GDExtension built with Rust, using [gdext](https://github.com/godot-rust/gdext).

## Description
Provides functionality to implement things like UI tours in the Editor & In-Game.

## Available Nodes
### `TheaterRect`

- Focus the users attention on any number of `Control` 's.
- Prevent mouse input outside of the focused region.

<img src="docs/images/TheaterRect-Inspector.png" height="280px" align="left">
<img src="docs/images/TheaterRect.png" height="280px">

### `PopupContainer`

- Provide your own `Control` and let the container handle positioning.
- Toggle `Smart Positioning` for the popup to stay on-screen.

<img src="docs/images/PopupContainer.png" width="405px" align="left">
<img src="docs/images/PopupContainer2.png" width="405px">

## Singletons
### `Tour`
- Adds ability to script editor tours.
- Provides utilities for interacting with the editor UI.

<img src="docs/images/CustomTour-EditorPlugin.png" height="270px" align="left">
<img src="docs/images/CustomTour.png" height="270px">

## Resources
### `FocusedNode`
- `target` The control to focus on.
- `overlay` The panel to use as an overlay.

## Notes for development.
- Attempting to change default resources will cause a multiple bind panic in `gdext`.
