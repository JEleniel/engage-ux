# Component Reference

Engage UX provides 50 fully-implemented, production-ready components organized into 9 categories.

## Component Categories

- [Informational Components](#informational-components) (11)
- [Interactive Components](#interactive-components) (14)
- [Layout Components](#layout-components) (4)
- [Notification Components](#notification-components) (3)
- [Menu Components](#menu-components) (4)
- [Dialog Components](#dialog-components) (5)
- [Window Controls](#window-controls) (3)
- [Pane Groups](#pane-groups) (2)
- [Graphic & Display](#graphic--display) (2)

---

## Informational Components

Display information to users.

| Component | Description | Key Features |
|-----------|-------------|--------------|
| [Label](label.md) | Simple text display | Alignment, font size, color |
| [Text](text.md) | Rich text display | Font weight, italic, underline, strikethrough |
| [Icon](icon.md) | Icon display | Rotation, flip, size, color, multiple formats |
| [Image](image.md) | Image display | Fit modes, lazy loading, alt text |
| [Avatar](avatar.md) | User avatar | Image/initials/icon modes, shapes, borders |
| [List](list.md) | List of items | Single/multi-select, custom items, colors |
| [Progress](progress.md) | Progress indicator | Linear/circular/indeterminate, labels, colors |
| [Tooltip](tooltip.md) | Contextual help | Positioning, delays, custom styling |
| [Breadcrumb](breadcrumb.md) | Navigation path | Custom separators, clickable items |
| [Line Numbers](line-numbers.md) | Code editor line numbers | Highlighting, custom colors |
| [Ruler](ruler.md) | Measurement ruler | Pixels/cm/inches/points, orientation |

## Interactive Components

Allow user interaction.

| Component | Description | Key Features |
|-----------|-------------|--------------|
| [Button](button.md) | Clickable button | Multiple variants, icons, loading state |
| [Checkbox](checkbox.md) | Toggle selection | Indeterminate state, sizes, disabled |
| [Radio Button](radio.md) | Single selection | Radio groups, values, colors |
| [Toggle](toggle.md) | On/off switch | Sizes, labels, colors |
| [Slider](slider.md) | Range selector | Min/max/step, value display, tooltips |
| [Text Input](text-input.md) | Single-line text | Types, validation, max length, placeholder |
| [Text Area](text-area.md) | Multi-line text | Rows/cols, max length, read-only, resize |
| [Select](select.md) | Dropdown selection | Options, search, multi-select, custom render |
| [Link](link.md) | Navigation link | Targets, underline, colors, external indicator |
| [Pagination](pagination.md) | Page navigation | Total pages, current page, page size options |
| [Carousel](carousel.md) | Image/content carousel | Auto-play, indicators, navigation, transitions |
| [Date Picker](date-picker.md) | Date selection | Min/max dates, custom formats, inline/popup |
| [Text Editor](text-editor.md) | Rich text editing | Formatting, undo/redo, custom toolbar |
| [Console](console.md) | Terminal-like display | ANSI colors, scrolling, line wrapping |

## Layout Components

Structure and organize content.

| Component | Description | Key Features |
|-----------|-------------|--------------|
| [Container](container.md) | Generic container | Flex layout, padding, margins, borders |
| [Card](card.md) | Content card | Header/body/footer, elevation, hover effects |
| [Table](table.md) | Data table | Sorting, filtering, pagination, row selection |
| [Window](window.md) | Application window | Title bar, controls, resizing, modals |

## Notification Components

Alert and inform users.

| Component | Description | Key Features |
|-----------|-------------|--------------|
| [Badge](badge.md) | Notification badge | Numbers, dots, positioning, colors |
| [Toast](toast.md) | Temporary message | Auto-dismiss, positioning, types, actions |
| [Banner](banner.md) | Persistent message | Dismissible, types, icons, actions |

## Menu Components

Navigation and command menus.

| Component | Description | Key Features |
|-----------|-------------|--------------|
| [Drawer](drawer.md) | Side drawer | Left/right/top/bottom, overlay/push modes |
| [Dropdown](dropdown.md) | Dropdown menu | Nested items, icons, shortcuts, separators |
| [Hamburger Menu](hamburger-menu.md) | Mobile menu | Animated icon, slide-in drawer |
| [Title Menu](title-menu.md) | Application menu bar | File/Edit/View menus, shortcuts |

## Dialog Components

Modal dialogs and overlays.

| Component | Description | Key Features |
|-----------|-------------|--------------|
| [Alert Dialog](alert.md) | Simple alert | Title, message, single action |
| [Confirm Dialog](confirm.md) | Confirmation prompt | Title, message, confirm/cancel |
| [Custom Modal](modal.md) | Custom modal dialog | Custom content, backdrop, sizing |
| [Open Dialog](open-dialog.md) | File open dialog | File types, multi-select, preview |
| [Save-As Dialog](save-as-dialog.md) | File save dialog | Default name, file types, overwrite confirm |

## Window Controls

Window management buttons.

| Component | Description | Key Features |
|-----------|-------------|--------------|
| [Close Button](close.md) | Close window | Custom icon, confirmation |
| [Maximize/Restore](maximize.md) | Maximize/restore window | State toggle, animations |
| [Minimize/Restore](minimize.md) | Minimize/restore window | Taskbar integration |

## Pane Groups

Organize content into groups.

| Component | Description | Key Features |
|-----------|-------------|--------------|
| [Accordion](accordion.md) | Collapsible panels | Single/multi-expand, icons, animations |
| [Tabs](tabs.md) | Tabbed interface | Closeable tabs, overflow, drag reorder |

## Graphic & Display

Graphics and media display.

| Component | Description | Key Features |
|-----------|-------------|--------------|
| [Group](group.md) | Group multiple components | Layout, styling, events |
| [Video](video.md) | Video player | Controls, seeking, volume, subtitles |

---

## Component Properties

All components share these common properties through the `Component` trait:

```rust
pub trait Component {
    fn id(&self) -> ComponentId;
    fn is_visible(&self) -> bool;
    fn set_visible(&mut self, visible: bool);
    fn is_enabled(&self) -> bool;
    fn set_enabled(&mut self, enabled: bool);
    fn bounds(&self) -> Rect;
    fn set_bounds(&mut self, bounds: Rect);
}
```

## Usage Patterns

### Basic Component Creation

```rust
use engage_ux_components::Button;

let mut button = Button::new(1, "Click Me");
button.set_bounds(Rect::new(100.0, 100.0, 120.0, 40.0));
```

### Event Handling

```rust
button.set_on_click(|event| {
    println!("Button clicked!");
});
```

### Theming

```rust
use engage_ux_themes::Theme;

let theme = Theme::dark();
button.set_background_color(theme.colors.primary);
button.set_text_color(theme.colors.on_primary);
```

### Container Layout

```rust
use engage_ux_components::Container;

let mut container = Container::new(1);
container.add_child(Box::new(button));
container.add_child(Box::new(label));
```

## Next Steps

- [Getting Started Guide](../getting-started.md) - Set up your first project
- [Theme Guide](../guides/theming.md) - Customize component appearance
- [Examples](../examples/) - See components in action
- [API Reference](../api/) - Detailed API documentation

---

[← Back to Documentation](../index.md)
