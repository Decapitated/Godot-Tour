# Godot Tour
A GDExtension built with Rust, using [gdext](https://github.com/godot-rust/gdext).

## Description
Provides functionality to implement things like UI tours.

## Available Nodes
### `TheaterRect`
Focus the users attention on any number of `Control`'s.<br>
#### Features
- Toggle the ability to prevent mouse input outside of the focused region.
<table>
  <tr>
    <td><img src="docs/images/TheaterRect.png"></td>
    <td><img src="docs/images/TheaterRect2.png"></td>
  </tr>
</table>

### `PopupContainer`
Moves next to a target `Control`.<br>
#### Features
- Provide your own `Control`. Let the container handle positioning.
- Toggle `Smart Positioning`. The popup will try to stay on-screen.
<table>
  <tr>
    <td><img src="docs/images/PopupContainer.png"></td>
    <td><img src="docs/images/PopupContainer2.png"></td>
  </tr>
</table>