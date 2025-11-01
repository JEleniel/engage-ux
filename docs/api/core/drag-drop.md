# Drag & Drop

Drag and drop covers `DragData`, `DragOperation`, `DragManager`, and the `DropTarget` trait.

Key items:

- `DragData` — typed payload for drag operations (text, files, images, custom).
- `DragOperation` — Copy, Move, Link, None.
- `DragManager` — manages current drag state and registered drop targets.
- `DropTarget` — trait for drop targets (can_drop, on_drop, on_drag_enter/leave).

Note: unit tests include a negative test for dropping when no target is registered to ensure the operation does not succeed silently.
