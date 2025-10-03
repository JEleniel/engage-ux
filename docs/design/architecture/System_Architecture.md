# System Architecture

## Overview

This document describes the overall system architecture of Engage UX, showing the major components, their relationships, and how data flows through the system.

## High-Level Architecture

Engage UX follows a layered architecture pattern with clear separation of concerns:

```mermaid
graph TB
    subgraph "Application Layer"
        APP[Application Code]
    end
    
    subgraph "Component Layer"
        COMP[UI Components<br/>50 Components]
        THEME[Theme System<br/>JSON Themes]
    end
    
    subgraph "Core Layer"
        COLOR[Color System]
        EVENT[Event System]
        INPUT[Input System]
        A11Y[Accessibility]
        MEDIA[Media Support]
        RENDER[Render Abstraction]
    end
    
    subgraph "Platform Layer (OAL)"
        PLATFORM[Platform Detection]
        WINDOW[Window Backend]
        GRAPHICS[Graphics Backend]
    end
    
    subgraph "OS Layer"
        WIN[Windows APIs]
        MAC[macOS APIs]
        LIN[Linux APIs]
        AND[Android APIs]
        IOS[iOS APIs]
    end
    
    APP --> COMP
    APP --> THEME
    COMP --> COLOR
    COMP --> EVENT
    COMP --> INPUT
    COMP --> A11Y
    THEME --> COLOR
    
    COLOR --> RENDER
    EVENT --> INPUT
    A11Y --> INPUT
    MEDIA --> RENDER
    
    RENDER --> GRAPHICS
    INPUT --> WINDOW
    EVENT --> WINDOW
    
    PLATFORM --> WINDOW
    PLATFORM --> GRAPHICS
    
    WINDOW --> WIN
    WINDOW --> MAC
    WINDOW --> LIN
    WINDOW --> AND
    WINDOW --> IOS
    
    GRAPHICS --> WIN
    GRAPHICS --> MAC
    GRAPHICS --> LIN
    GRAPHICS --> AND
    GRAPHICS --> IOS
    
    style APP fill:#e1f5ff
    style COMP fill:#b3e5fc
    style THEME fill:#b3e5fc
    style COLOR fill:#81d4fa
    style EVENT fill:#81d4fa
    style INPUT fill:#81d4fa
    style A11Y fill:#81d4fa
    style MEDIA fill:#81d4fa
    style RENDER fill:#81d4fa
    style PLATFORM fill:#4fc3f7
    style WINDOW fill:#4fc3f7
    style GRAPHICS fill:#4fc3f7
    style WIN fill:#03a9f4
    style MAC fill:#03a9f4
    style LIN fill:#03a9f4
    style AND fill:#03a9f4
    style IOS fill:#03a9f4
```

## Crate Organization

Engage UX is organized as a Cargo workspace with the following crates:

```mermaid
graph LR
    subgraph "Workspace"
        CORE[engage-ux-core<br/>Foundation]
        OAL[engage-ux-oal<br/>OS Abstraction]
        COMP[engage-ux-components<br/>UI Components]
        THEME[engage-ux-themes<br/>Theme System]
        TESTS[engage-ux-tests<br/>Integration Tests]
    end
    
    COMP --> CORE
    COMP --> OAL
    THEME --> CORE
    OAL --> CORE
    TESTS --> CORE
    TESTS --> COMP
    TESTS --> THEME
    TESTS --> OAL
    
    style CORE fill:#4caf50
    style OAL fill:#8bc34a
    style COMP fill:#ffc107
    style THEME fill:#ff9800
    style TESTS fill:#9e9e9e
```

## Component Architecture

The component system follows a trait-based architecture:

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
    
    class Button {
        -properties: ComponentProperties
        -text: String
        -variant: ButtonVariant
        -on_click: EventCallback
        +new(id, text) Button
        +set_text(text)
        +set_on_click(callback)
    }
    
    class TextInput {
        -properties: ComponentProperties
        -value: String
        -input_type: InputType
        -on_change: EventCallback
        +new(id) TextInput
        +set_value(value)
        +set_on_change(callback)
    }
    
    class Card {
        -properties: ComponentProperties
        -header: Option~String~
        -children: Vec~ComponentId~
        +new(id) Card
        +set_header(header)
        +add_child(component)
    }
    
    Component <|.. Button
    Component <|.. TextInput
    Component <|.. Card
    Button o-- ComponentProperties
    TextInput o-- ComponentProperties
    Card o-- ComponentProperties
```

## Event Flow

Events flow through the system from OS to components:

```mermaid
sequenceDiagram
    participant OS as Operating System
    participant WIN as Window Backend
    participant EVT as Event Handler
    participant COMP as Component
    participant APP as Application Code
    
    OS->>WIN: OS Event (mouse click, key press)
    WIN->>EVT: Convert to Engage Event
    EVT->>EVT: Broadcast to subscribers
    EVT->>COMP: Component receives event
    COMP->>COMP: Process event
    COMP->>APP: Execute callback
    APP->>COMP: Update component state
    COMP->>WIN: Request render
    WIN->>OS: Draw to screen
```

## Rendering Pipeline

The rendering pipeline abstracts platform-specific graphics:

```mermaid
flowchart TB
    START([Render Request])
    COMP[Component Tree]
    RENDER[Render Abstraction]
    CMD[Render Commands]
    BACKEND[Platform Backend]
    
    subgraph "Platform Implementations"
        D2D[Direct2D<br/>Windows]
        CG[Core Graphics<br/>macOS]
        CAIRO[Cairo<br/>Linux]
        CANVAS[Canvas<br/>Android]
        CGIO[Core Graphics<br/>iOS]
    end
    
    OUTPUT([Screen Output])
    
    START --> COMP
    COMP --> RENDER
    RENDER --> CMD
    CMD --> BACKEND
    BACKEND --> D2D
    BACKEND --> CG
    BACKEND --> CAIRO
    BACKEND --> CANVAS
    BACKEND --> CGIO
    D2D --> OUTPUT
    CG --> OUTPUT
    CAIRO --> OUTPUT
    CANVAS --> OUTPUT
    CGIO --> OUTPUT
    
    style START fill:#4caf50
    style COMP fill:#8bc34a
    style RENDER fill:#cddc39
    style CMD fill:#ffeb3b
    style BACKEND fill:#ffc107
    style D2D fill:#ff9800
    style CG fill:#ff9800
    style CAIRO fill:#ff9800
    style CANVAS fill:#ff9800
    style CGIO fill:#ff9800
    style OUTPUT fill:#ff7043
```

## Theme System Architecture

Themes are loaded from JSON and applied to components:

```mermaid
flowchart LR
    JSON[Theme JSON File]
    PARSE[JSON Parser]
    THEME[Theme Object]
    COLORS[Color Palette]
    TYPO[Typography]
    SPACING[Spacing]
    
    JSON --> PARSE
    PARSE --> THEME
    THEME --> COLORS
    THEME --> TYPO
    THEME --> SPACING
    
    COLORS --> BUTTON[Button]
    COLORS --> INPUT[Text Input]
    COLORS --> CARD[Card]
    
    TYPO --> LABEL[Label]
    TYPO --> TEXT[Text]
    
    SPACING --> CONTAINER[Container]
    SPACING --> LAYOUT[Layout]
    
    style JSON fill:#e3f2fd
    style PARSE fill:#bbdefb
    style THEME fill:#90caf9
    style COLORS fill:#64b5f6
    style TYPO fill:#64b5f6
    style SPACING fill:#64b5f6
    style BUTTON fill:#42a5f5
    style INPUT fill:#42a5f5
    style CARD fill:#42a5f5
    style LABEL fill:#42a5f5
    style TEXT fill:#42a5f5
    style CONTAINER fill:#42a5f5
    style LAYOUT fill:#42a5f5
```

## Input System Architecture

Input is processed through a unified handler system:

```mermaid
graph TB
    subgraph "Input Sources"
        KB[Keyboard]
        MOUSE[Mouse]
        TOUCH[Touch Screen]
    end
    
    subgraph "Input Processing"
        KBSTATE[Keyboard State]
        MOUSESTATE[Mouse State]
        TOUCHSTATE[Touch State]
        GESTURE[Gesture Recognition]
    end
    
    subgraph "Input Events"
        KBEVT[Key Events]
        MOUSEEVT[Mouse Events]
        TOUCHEVT[Touch Events]
        GESTEVT[Gesture Events]
    end
    
    subgraph "Components"
        HANDLER[Input Handler Trait]
        COMP1[Interactive Components]
    end
    
    KB --> KBSTATE
    MOUSE --> MOUSESTATE
    TOUCH --> TOUCHSTATE
    TOUCHSTATE --> GESTURE
    
    KBSTATE --> KBEVT
    MOUSESTATE --> MOUSEEVT
    TOUCHSTATE --> TOUCHEVT
    GESTURE --> GESTEVT
    
    KBEVT --> HANDLER
    MOUSEEVT --> HANDLER
    TOUCHEVT --> HANDLER
    GESTEVT --> HANDLER
    
    HANDLER --> COMP1
    
    style KB fill:#e8f5e9
    style MOUSE fill:#e8f5e9
    style TOUCH fill:#e8f5e9
    style KBSTATE fill:#c8e6c9
    style MOUSESTATE fill:#c8e6c9
    style TOUCHSTATE fill:#c8e6c9
    style GESTURE fill:#c8e6c9
    style KBEVT fill:#a5d6a7
    style MOUSEEVT fill:#a5d6a7
    style TOUCHEVT fill:#a5d6a7
    style GESTEVT fill:#a5d6a7
    style HANDLER fill:#81c784
    style COMP1 fill:#66bb6a
```

## Accessibility Architecture

Accessibility is integrated throughout the component system:

```mermaid
graph TB
    subgraph "Components"
        COMP[UI Components]
        A11Y[Accessibility Props]
    end
    
    subgraph "Accessibility Layer"
        ARIA[ARIA Roles & Attributes]
        FOCUS[Focus Manager]
        SR[Screen Reader Support]
    end
    
    subgraph "Platform Accessibility"
        MSAA[MSAA/UI Automation<br/>Windows]
        NSA11Y[NSAccessibility<br/>macOS]
        ATSPI[AT-SPI<br/>Linux]
        TALK[TalkBack<br/>Android]
        VOICE[VoiceOver<br/>iOS]
    end
    
    COMP --> A11Y
    A11Y --> ARIA
    A11Y --> FOCUS
    A11Y --> SR
    
    ARIA --> MSAA
    ARIA --> NSA11Y
    ARIA --> ATSPI
    ARIA --> TALK
    ARIA --> VOICE
    
    FOCUS --> MSAA
    FOCUS --> NSA11Y
    FOCUS --> ATSPI
    FOCUS --> TALK
    FOCUS --> VOICE
    
    SR --> MSAA
    SR --> NSA11Y
    SR --> ATSPI
    SR --> TALK
    SR --> VOICE
    
    style COMP fill:#f3e5f5
    style A11Y fill:#e1bee7
    style ARIA fill:#ce93d8
    style FOCUS fill:#ce93d8
    style SR fill:#ce93d8
    style MSAA fill:#ce93d8
    style NSA11Y fill:#ce93d8
    style ATSPI fill:#ce93d8
    style TALK fill:#ce93d8
    style VOICE fill:#ce93d8
```

## Design Patterns Used

### 1. Trait-Based Architecture
- `Component` trait for all UI elements
- `RenderBackend` trait for platform-specific rendering
- `WindowBackend` trait for platform-specific windows
- `InputHandler` trait for input processing

### 2. Factory Pattern
- Platform-specific backend creation
- Component creation with builders

### 3. Observer Pattern
- Event system with broadcast channels
- Component callbacks for state changes

### 4. Strategy Pattern
- Different rendering strategies per platform
- Different window management strategies per platform

### 5. Adapter Pattern
- OS APIs adapted to unified interface
- Color format conversions

### 6. Command Pattern
- Render commands for graphics operations
- Event commands for user interactions

## Thread Safety

Engage UX is designed to be fully thread-safe:

- **Components**: Wrapped in `Arc<RwLock<T>>` for concurrent access
- **Events**: Tokio broadcast channels for async event distribution
- **State**: Atomic types and locks for shared state
- **Rendering**: Render commands queued and executed on render thread

## Performance Considerations

- **Lazy Loading**: Images and fonts loaded on demand
- **Caching**: Rendered elements cached where appropriate
- **Async Operations**: Long-running tasks use async/await
- **Memory Management**: RAII ensures proper cleanup
- **Minimal Allocations**: Reduce heap allocations in hot paths

## Security Architecture

- **Memory Safety**: `#![forbid(unsafe_code)]` in all crates
- **Input Validation**: All external input validated
- **SVG Security**: Scripts and external resources blocked
- **Dependency Vetting**: Only maintained, secure dependencies
- **Sandboxing**: Platform-specific sandboxing where available
