# Component Architecture

## Overview

This document details the architecture of the component system in Engage UX, showing how the 50 UI components are organized, how they inherit common functionality, and how they interact with the core systems.

## Component Hierarchy

All components implement the `Component` trait and share common properties:

```mermaid
classDiagram
    class Component {
        <<trait>>
        +id() ComponentId
        +properties() ComponentProperties
        +properties_mut() ComponentProperties
    }
    
    class ComponentProperties {
        +id: ComponentId
        +visible: bool
        +enabled: bool
        +bounds: Rect
    }
    
    class Rect {
        +x: f32
        +y: f32
        +width: f32
        +height: f32
    }
    
    Component --> ComponentProperties
    ComponentProperties --> Rect
```

## Component Categories

The 50 components are organized into logical categories:

```mermaid
graph TB
    ROOT[Engage UX Components<br/>50 Total]
    
    INFO[Informational<br/>11 Components]
    INTER[Interactive<br/>14 Components]
    GRAPH[Graphic & Display<br/>2 Components]
    NOTIF[Notification<br/>3 Components]
    MENU[Menus<br/>4 Components]
    WIN[Window Controls<br/>3 Components]
    PANE[Pane Groups<br/>2 Components]
    DIALOG[Dialogs<br/>5 Components]
    LAYOUT[Layout/Grouping<br/>4 Components]
    
    ROOT --> INFO
    ROOT --> INTER
    ROOT --> GRAPH
    ROOT --> NOTIF
    ROOT --> MENU
    ROOT --> WIN
    ROOT --> PANE
    ROOT --> DIALOG
    ROOT --> LAYOUT
    
    INFO --> I1[Label]
    INFO --> I2[Text]
    INFO --> I3[Icon]
    INFO --> I4[Image]
    INFO --> I5[List]
    INFO --> I6[Progress]
    INFO --> I7[Tooltip]
    INFO --> I8[Breadcrumb]
    INFO --> I9[Avatar]
    INFO --> I10[Line Numbers]
    INFO --> I11[Ruler]
    
    INTER --> IN1[Button]
    INTER --> IN2[Checkbox]
    INTER --> IN3[Radio]
    INTER --> IN4[Slider]
    INTER --> IN5[Text Input]
    INTER --> IN6[Text Area]
    INTER --> IN7[Toggle]
    INTER --> IN8[Select]
    INTER --> IN9[Link]
    INTER --> IN10[Pagination]
    INTER --> IN11[Carousel]
    INTER --> IN12[Date Picker]
    INTER --> IN13[Text Editor]
    INTER --> IN14[Console]
    
    style ROOT fill:#e1f5fe
    style INFO fill:#b3e5fc
    style INTER fill:#81d4fa
    style GRAPH fill:#4fc3f7
    style NOTIF fill:#29b6f6
    style MENU fill:#03a9f4
    style WIN fill:#03a9f4
    style PANE fill:#03a9f4
    style DIALOG fill:#03a9f4
    style LAYOUT fill:#03a9f4
```

## Informational Components Detail

```mermaid
classDiagram
    class Component {
        <<trait>>
        +id() ComponentId
    }
    
    class Label {
        -properties: ComponentProperties
        -text: String
        -color: Color
        -font_size: f32
        -alignment: TextAlign
        +new(id, text) Label
        +set_text(text)
        +set_color(color)
    }
    
    class Icon {
        -properties: ComponentProperties
        -name: String
        -size: f32
        -color: Color
        -rotation: f32
        -flip_horizontal: bool
        -flip_vertical: bool
        +new(id, name) Icon
        +set_size(size)
        +set_rotation(degrees)
    }
    
    class Image {
        -properties: ComponentProperties
        -source: String
        -alt_text: String
        -fit: ImageFit
        -lazy: bool
        +new(id, source) Image
        +set_fit(fit)
        +set_lazy(lazy)
    }
    
    class Progress {
        -properties: ComponentProperties
        -value: f32
        -max: f32
        -progress_type: ProgressType
        -label: Option~String~
        +new(id) Progress
        +set_value(value)
        +set_type(type)
    }
    
    Component <|.. Label
    Component <|.. Icon
    Component <|.. Image
    Component <|.. Progress
```

## Interactive Components Detail

```mermaid
classDiagram
    class Component {
        <<trait>>
        +id() ComponentId
    }
    
    class Button {
        -properties: ComponentProperties
        -text: String
        -variant: ButtonVariant
        -color: Color
        -disabled: bool
        -on_click: EventCallback
        +new(id, text) Button
        +set_variant(variant)
        +set_on_click(callback)
    }
    
    class TextInput {
        -properties: ComponentProperties
        -value: String
        -placeholder: String
        -input_type: InputType
        -max_length: Option~usize~
        -on_change: EventCallback
        +new(id) TextInput
        +set_value(value)
        +set_type(type)
    }
    
    class Checkbox {
        -properties: ComponentProperties
        -checked: bool
        -indeterminate: bool
        -label: String
        -on_change: EventCallback
        +new(id, label) Checkbox
        +set_checked(checked)
        +toggle()
    }
    
    class Slider {
        -properties: ComponentProperties
        -value: f32
        -min: f32
        -max: f32
        -step: f32
        -on_change: EventCallback
        +new(id) Slider
        +set_value(value)
        +set_range(min, max)
    }
    
    Component <|.. Button
    Component <|.. TextInput
    Component <|.. Checkbox
    Component <|.. Slider
```

## Dialog Components Detail

```mermaid
classDiagram
    class Component {
        <<trait>>
        +id() ComponentId
    }
    
    class AlertDialog {
        -properties: ComponentProperties
        -title: String
        -message: String
        -alert_type: AlertType
        -on_ok: EventCallback
        +new(id, title, message) AlertDialog
        +set_type(type)
        +show()
    }
    
    class ConfirmDialog {
        -properties: ComponentProperties
        -title: String
        -message: String
        -on_confirm: EventCallback
        -on_cancel: EventCallback
        +new(id, title, message) ConfirmDialog
        +show()
    }
    
    class Modal {
        -properties: ComponentProperties
        -title: String
        -children: Vec~ComponentId~
        -backdrop_close: bool
        -on_close: EventCallback
        +new(id, title) Modal
        +add_child(component)
        +show()
    }
    
    class FileDialog {
        -properties: ComponentProperties
        -dialog_type: FileDialogType
        -filters: Vec~FileFilter~
        -on_select: EventCallback
        +new(id, type) FileDialog
        +add_filter(filter)
        +show()
    }
    
    Component <|.. AlertDialog
    Component <|.. ConfirmDialog
    Component <|.. Modal
    Component <|.. FileDialog
```

## Layout Components Detail

```mermaid
classDiagram
    class Component {
        <<trait>>
        +id() ComponentId
    }
    
    class Container {
        -properties: ComponentProperties
        -children: Vec~ComponentId~
        -padding: Spacing
        -gap: f32
        +new(id) Container
        +add_child(component)
        +set_padding(padding)
    }
    
    class Card {
        -properties: ComponentProperties
        -header: Option~String~
        -footer: Option~String~
        -children: Vec~ComponentId~
        +new(id) Card
        +set_header(header)
        +add_child(component)
    }
    
    class Table {
        -properties: ComponentProperties
        -columns: Vec~TableColumn~
        -rows: Vec~TableRow~
        -sortable: bool
        -selectable: bool
        +new(id) Table
        +add_column(column)
        +add_row(row)
    }
    
    class Window {
        -properties: ComponentProperties
        -title: String
        -icon: Option~String~
        -children: Vec~ComponentId~
        -controls: WindowControls
        +new(id, title) Window
        +set_title(title)
        +add_child(component)
    }
    
    Component <|.. Container
    Component <|.. Card
    Component <|.. Table
    Component <|.. Window
```

## Component State Management

Each component manages its own state:

```mermaid
stateDiagram-v2
    [*] --> Initialized: new()
    
    Initialized --> Idle: ready
    
    Idle --> Active: user interaction
    Active --> Idle: interaction complete
    
    Idle --> Disabled: disable()
    Disabled --> Idle: enable()
    
    Idle --> Focused: focus()
    Focused --> Idle: blur()
    
    Active --> Pressed: button down
    Pressed --> Active: button up
    
    Idle --> Hovering: mouse enter
    Hovering --> Idle: mouse leave
    
    note right of Initialized
        Default properties set
        Event handlers registered
    end note
    
    note right of Active
        Processing user input
        Visual feedback active
    end note
    
    note right of Focused
        Keyboard events received
        Focus indicator visible
    end note
```

## Component Communication Patterns

### 1. Parent-Child Communication

```mermaid
sequenceDiagram
    participant Parent as Parent Component
    participant Child as Child Component
    participant EventBus as Event Bus
    
    Parent->>Child: Add child to container
    Parent->>Child: Set child properties
    Child->>EventBus: Subscribe to events
    
    loop Event Processing
        EventBus->>Child: Dispatch event
        Child->>Child: Handle event
        Child->>Parent: Notify state change
        Parent->>Parent: Update layout
    end
```

### 2. Sibling Communication

```mermaid
sequenceDiagram
    participant Comp1 as Component 1
    participant EventBus as Event Bus
    participant Comp2 as Component 2
    
    Comp1->>Comp1: State changes
    Comp1->>EventBus: Publish custom event
    EventBus->>Comp2: Broadcast to subscribers
    Comp2->>Comp2: Handle event
    Comp2->>EventBus: May publish response
```

### 3. Component-to-Application Communication

```mermaid
sequenceDiagram
    participant Comp as Component
    participant Callback as Event Callback
    participant App as Application Logic
    participant Store as State Store
    
    Comp->>Callback: Invoke on_event
    Callback->>App: Execute handler
    App->>Store: Update application state
    Store->>App: Notify observers
    App->>Comp: Update component props
    Comp->>Comp: Re-render
```

## Component Event Handling

Components can handle various event types:

```mermaid
graph TB
    COMP[Component]
    
    subgraph "Input Events"
        CLICK[Click]
        DBLCLICK[Double Click]
        KEYDOWN[Key Down]
        KEYUP[Key Up]
        MOUSEMOVE[Mouse Move]
        MOUSEENTER[Mouse Enter]
        MOUSELEAVE[Mouse Leave]
        WHEEL[Mouse Wheel]
        TOUCHSTART[Touch Start]
        TOUCHMOVE[Touch Move]
        TOUCHEND[Touch End]
    end
    
    subgraph "Focus Events"
        FOCUS[Focus Gained]
        BLUR[Focus Lost]
    end
    
    subgraph "State Events"
        CHANGE[Value Changed]
        SELECT[Selection Changed]
        TOGGLE[Toggle]
    end
    
    subgraph "Custom Events"
        CUSTOM[Custom Event]
    end
    
    CLICK --> COMP
    DBLCLICK --> COMP
    KEYDOWN --> COMP
    KEYUP --> COMP
    MOUSEMOVE --> COMP
    MOUSEENTER --> COMP
    MOUSELEAVE --> COMP
    WHEEL --> COMP
    TOUCHSTART --> COMP
    TOUCHMOVE --> COMP
    TOUCHEND --> COMP
    
    FOCUS --> COMP
    BLUR --> COMP
    
    CHANGE --> COMP
    SELECT --> COMP
    TOGGLE --> COMP
    
    CUSTOM --> COMP
    
    style COMP fill:#4caf50
    style CLICK fill:#81c784
    style FOCUS fill:#aed581
    style CHANGE fill:#c5e1a5
    style CUSTOM fill:#dcedc8
```

## Component Rendering Pipeline

Each component follows a rendering pipeline:

```mermaid
flowchart TB
    START[Component Render Called]
    
    CHECK_VIS{Visible?}
    CHECK_BOUNDS{Has Bounds?}
    
    PREP[Prepare Render State]
    APPLY_THEME[Apply Theme Colors]
    CALC_LAYOUT[Calculate Layout]
    
    GEN_CMD[Generate Render Commands]
    
    subgraph "Render Commands"
        FILL[Fill Background]
        BORDER[Draw Border]
        CONTENT[Draw Content]
        CHILDREN[Render Children]
    end
    
    SUBMIT[Submit to Backend]
    END_RENDER([Render Complete])
    
    START --> CHECK_VIS
    CHECK_VIS -->|No| END_RENDER
    CHECK_VIS -->|Yes| CHECK_BOUNDS
    
    CHECK_BOUNDS -->|No| END_RENDER
    CHECK_BOUNDS -->|Yes| PREP
    
    PREP --> APPLY_THEME
    APPLY_THEME --> CALC_LAYOUT
    CALC_LAYOUT --> GEN_CMD
    
    GEN_CMD --> FILL
    FILL --> BORDER
    BORDER --> CONTENT
    CONTENT --> CHILDREN
    
    CHILDREN --> SUBMIT
    SUBMIT --> END_RENDER
    
    style START fill:#4caf50
    style CHECK_VIS fill:#8bc34a
    style CHECK_BOUNDS fill:#cddc39
    style PREP fill:#ffeb3b
    style APPLY_THEME fill:#ffc107
    style CALC_LAYOUT fill:#ff9800
    style GEN_CMD fill:#ff7043
    style SUBMIT fill:#ff7043
    style END_RENDER fill:#ce93d8
```

## Component Theming Integration

Components integrate with the theme system:

```mermaid
graph TB
    THEME[Theme Object]
    
    subgraph "Theme Properties"
        COLORS[Color Palette]
        TYPO[Typography]
        SPACING[Spacing Scale]
        BORDERS[Border Styles]
        SHADOWS[Shadow Effects]
    end
    
    THEME --> COLORS
    THEME --> TYPO
    THEME --> SPACING
    THEME --> BORDERS
    THEME --> SHADOWS
    
    subgraph "Component Theming"
        BTN_THEME[Button Theme]
        INPUT_THEME[Input Theme]
        CARD_THEME[Card Theme]
    end
    
    COLORS --> BTN_THEME
    COLORS --> INPUT_THEME
    COLORS --> CARD_THEME
    
    TYPO --> BTN_THEME
    TYPO --> INPUT_THEME
    
    SPACING --> INPUT_THEME
    SPACING --> CARD_THEME
    
    BORDERS --> BTN_THEME
    BORDERS --> INPUT_THEME
    BORDERS --> CARD_THEME
    
    SHADOWS --> CARD_THEME
    
    subgraph "Components"
        BTN[Button]
        INPUT[Text Input]
        CARD[Card]
    end
    
    BTN_THEME --> BTN
    INPUT_THEME --> INPUT
    CARD_THEME --> CARD
    
    style THEME fill:#e1f5fe
    style COLORS fill:#b3e5fc
    style BTN fill:#4fc3f7
    style INPUT fill:#4fc3f7
    style CARD fill:#4fc3f7
```

## Component Accessibility Integration

Components expose accessibility information:

```mermaid
graph LR
    COMP[Component]
    
    subgraph "Accessibility Props"
        ROLE[ARIA Role]
        LABEL[Accessible Label]
        DESC[Description]
        STATE[ARIA States]
        PROPS[ARIA Properties]
    end
    
    COMP --> ROLE
    COMP --> LABEL
    COMP --> DESC
    COMP --> STATE
    COMP --> PROPS
    
    subgraph "Accessibility Features"
        TAB[Tab Order]
        FOCUS[Focus Indicator]
        KEYBOARD[Keyboard Nav]
        ANNOUNCE[Screen Reader]
    end
    
    ROLE --> ANNOUNCE
    LABEL --> ANNOUNCE
    DESC --> ANNOUNCE
    STATE --> ANNOUNCE
    PROPS --> ANNOUNCE
    
    COMP --> TAB
    COMP --> FOCUS
    COMP --> KEYBOARD
    
    style COMP fill:#e8eaf6
    style ROLE fill:#c5cae9
    style TAB fill:#9fa8da
    style ANNOUNCE fill:#9fa8da
```

## Performance Optimizations

Components implement several performance optimizations:

### Lazy Rendering
- Components only render when visible
- Off-screen components don't generate render commands
- Visibility culling based on viewport

### Caching
- Layout calculations cached until invalidated
- Render commands cached for static content
- Theme lookups cached per component

### Batching
- Multiple component updates batched per frame
- Render commands batched before submission
- Event processing batched

### Memory Management
- Components use Arc<RwLock> for shared ownership
- Weak references prevent circular dependencies
- Resources released in component Drop impl
