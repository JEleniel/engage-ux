# Data Flow Architecture

## Overview

This document describes how data flows through the Engage UX system, from user input to screen output, and how components communicate with each other and the platform layer.

## User Input Flow

This diagram shows how user input (keyboard, mouse, touch) flows from the operating system to application code:

```mermaid
sequenceDiagram
    actor User
    participant OS as Operating System
    participant WB as Window Backend
    participant IS as Input System
    participant EH as Event Handler
    participant FM as Focus Manager
    participant COMP as Component
    participant CB as Callback
    participant APP as Application
    
    User->>OS: Physical input (click, key, touch)
    OS->>WB: OS-specific event
    WB->>IS: Convert to Engage input event
    IS->>IS: Update input state
    IS->>EH: Publish input event
    
    alt Interactive component has focus
        EH->>FM: Check focused component
        FM->>COMP: Route to focused component
    else No focus or global event
        EH->>COMP: Broadcast to all listeners
    end
    
    COMP->>COMP: Process input
    COMP->>CB: Invoke event callback
    CB->>APP: Execute application logic
    APP->>COMP: Update component state
    COMP->>WB: Request redraw
    WB->>OS: Display updated UI
    OS->>User: Visual feedback
```

## Rendering Flow

This diagram shows how components are rendered to the screen:

```mermaid
flowchart TB
    START([Application Updates Component])
    
    UPDATE[Component State Changed]
    TREE[Build Component Tree]
    LAYOUT[Calculate Layout]
    RENDER[Generate Render Commands]
    
    subgraph "Render Backend"
        QUEUE[Command Queue]
        EXEC[Execute Commands]
        BACKEND[Platform Backend]
    end
    
    subgraph "Platform Graphics"
        WIN_GFX[Windows<br/>Direct2D/D3D]
        MAC_GFX[macOS<br/>Core Graphics]
        LIN_GFX[Linux<br/>Cairo]
        AND_GFX[Android<br/>Canvas]
        IOS_GFX[iOS<br/>Core Graphics]
    end
    
    DISPLAY([Screen Display])
    
    START --> UPDATE
    UPDATE --> TREE
    TREE --> LAYOUT
    LAYOUT --> RENDER
    RENDER --> QUEUE
    QUEUE --> EXEC
    EXEC --> BACKEND
    
    BACKEND --> WIN_GFX
    BACKEND --> MAC_GFX
    BACKEND --> LIN_GFX
    BACKEND --> AND_GFX
    BACKEND --> IOS_GFX
    
    WIN_GFX --> DISPLAY
    MAC_GFX --> DISPLAY
    LIN_GFX --> DISPLAY
    AND_GFX --> DISPLAY
    IOS_GFX --> DISPLAY
    
    style START fill:#4caf50
    style UPDATE fill:#8bc34a
    style TREE fill:#cddc39
    style LAYOUT fill:#ffeb3b
    style RENDER fill:#ffc107
    style QUEUE fill:#ff9800
    style EXEC fill:#ff5722
    style BACKEND fill:#f44336
    style DISPLAY fill:#e91e63
```

## Theme Application Flow

This diagram shows how themes are loaded and applied to components:

```mermaid
sequenceDiagram
    participant APP as Application
    participant FILE as JSON File
    participant PARSER as JSON Parser
    participant THEME as Theme Object
    participant COLORS as Color System
    participant COMP as Components
    participant RENDER as Render System
    
    APP->>FILE: Load theme file
    FILE->>PARSER: Read JSON content
    PARSER->>PARSER: Validate against schema
    
    alt Valid theme
        PARSER->>COLORS: Parse color values
        COLORS->>COLORS: Convert to internal format
        COLORS->>THEME: Build theme object
        THEME->>APP: Return theme
        APP->>COMP: Apply theme to components
        COMP->>COMP: Update colors/styles
        COMP->>RENDER: Request redraw
    else Invalid theme
        PARSER->>APP: Return error with details
    end
```

## Event System Data Flow

Events use a publish-subscribe pattern with Tokio channels:

```mermaid
graph TB
    subgraph "Event Sources"
        INPUT[Input Events]
        WINDOW[Window Events]
        CUSTOM[Custom Events]
    end
    
    subgraph "Event Handler"
        CHANNEL[Broadcast Channel]
        QUEUE[Event Queue]
    end
    
    subgraph "Event Consumers"
        COMP1[Component 1]
        COMP2[Component 2]
        COMP3[Component 3]
        APP[Application Handler]
    end
    
    INPUT --> CHANNEL
    WINDOW --> CHANNEL
    CUSTOM --> CHANNEL
    
    CHANNEL --> QUEUE
    
    QUEUE --> COMP1
    QUEUE --> COMP2
    QUEUE --> COMP3
    QUEUE --> APP
    
    COMP1 --> |"State Change"| CHANNEL
    COMP2 --> |"State Change"| CHANNEL
    COMP3 --> |"State Change"| CHANNEL
    
    style INPUT fill:#e3f2fd
    style WINDOW fill:#e3f2fd
    style CUSTOM fill:#e3f2fd
    style CHANNEL fill:#90caf9
    style QUEUE fill:#64b5f6
    style COMP1 fill:#42a5f5
    style COMP2 fill:#42a5f5
    style COMP3 fill:#42a5f5
    style APP fill:#42a5f5
```

## Color Conversion Flow

Colors can be specified in multiple formats and are converted internally:

```mermaid
flowchart LR
    subgraph "Input Formats"
        HEX[Hex String<br/>#RRGGBB]
        RGB_ARR[RGB Array<br/>[r,g,b]]
        HSL_ARR[HSL Array<br/>[h,s,l]]
        LEGACY[Legacy Format<br/>space+components]
    end
    
    subgraph "Parsing"
        PARSE[Format Detection]
        VALIDATE[Validation]
    end
    
    subgraph "Internal Representation"
        INTERNAL[Color Struct<br/>RGBA 0.0-1.0]
    end
    
    subgraph "Output Formats"
        RGB_OUT[RGB for Rendering]
        HSL_OUT[HSL for Manipulation]
        SERIALIZE[JSON Serialization]
    end
    
    HEX --> PARSE
    RGB_ARR --> PARSE
    HSL_ARR --> PARSE
    LEGACY --> PARSE
    
    PARSE --> VALIDATE
    VALIDATE --> INTERNAL
    
    INTERNAL --> RGB_OUT
    INTERNAL --> HSL_OUT
    INTERNAL --> SERIALIZE
    
    style HEX fill:#fff3e0
    style RGB_ARR fill:#fff3e0
    style HSL_ARR fill:#fff3e0
    style LEGACY fill:#fff3e0
    style PARSE fill:#ffe0b2
    style VALIDATE fill:#ffcc80
    style INTERNAL fill:#ffb74d
    style RGB_OUT fill:#ffa726
    style HSL_OUT fill:#ffa726
    style SERIALIZE fill:#ffa726
```

## Component Lifecycle Data Flow

Components have a lifecycle from creation to destruction:

```mermaid
stateDiagram-v2
    [*] --> Created: new()
    Created --> Initialized: initialize()
    Initialized --> Visible: show()
    Visible --> Hidden: hide()
    Hidden --> Visible: show()
    
    Visible --> Focused: focus()
    Focused --> Visible: blur()
    
    Visible --> Disabled: disable()
    Disabled --> Visible: enable()
    
    Initialized --> EventProcessing: on_event()
    EventProcessing --> Initialized: event handled
    
    Visible --> Rendering: render()
    Rendering --> Visible: rendered
    
    Hidden --> Destroyed: destroy()
    Visible --> Destroyed: destroy()
    Disabled --> Destroyed: destroy()
    Destroyed --> [*]
    
    note right of Created
        Component allocated
        Default properties set
    end note
    
    note right of Visible
        Component displayed
        Receives events
        Participates in layout
    end note
    
    note right of Rendering
        Render commands generated
        Sent to backend
    end note
```

## Window Management Flow

Window operations flow through the OAL to platform-specific implementations:

```mermaid
sequenceDiagram
    participant APP as Application
    participant OAL as OAL Facade
    participant WIN as Window Backend
    participant OS as OS APIs
    participant SCREEN as Screen
    
    APP->>OAL: Create window
    OAL->>WIN: Create platform window
    WIN->>OS: Call native window API
    OS->>SCREEN: Display window
    
    APP->>OAL: Set window title
    OAL->>WIN: Update window title
    WIN->>OS: Set title via native API
    
    APP->>OAL: Show window
    OAL->>WIN: Make window visible
    WIN->>OS: Show window
    OS->>SCREEN: Window appears
    
    loop User Interactions
        SCREEN->>OS: User resizes window
        OS->>WIN: Window resize event
        WIN->>OAL: Resize event
        OAL->>APP: Notify application
        APP->>OAL: Request redraw
        OAL->>WIN: Render content
        WIN->>OS: Draw to screen
        OS->>SCREEN: Updated display
    end
    
    APP->>OAL: Close window
    OAL->>WIN: Destroy window
    WIN->>OS: Close native window
    OS->>SCREEN: Window disappears
```

## Accessibility Data Flow

Accessibility information flows from components to platform accessibility APIs:

```mermaid
flowchart TB
    START[Component with A11Y Props]
    
    subgraph "Component Layer"
        PROPS[Accessibility Properties]
        ROLE[ARIA Role]
        LABEL[Accessible Label]
        STATE[Component State]
    end
    
    subgraph "Accessibility Layer"
        TREE[Accessibility Tree]
        FOCUS[Focus Manager]
        SR[Screen Reader Manager]
    end
    
    subgraph "Platform A11Y APIs"
        WIN_A11Y[Windows<br/>UI Automation]
        MAC_A11Y[macOS<br/>NSAccessibility]
        LIN_A11Y[Linux<br/>AT-SPI]
        AND_A11Y[Android<br/>TalkBack]
        IOS_A11Y[iOS<br/>VoiceOver]
    end
    
    ASSISTIVE[Assistive Technology]
    
    START --> PROPS
    START --> ROLE
    START --> LABEL
    START --> STATE
    
    PROPS --> TREE
    ROLE --> TREE
    LABEL --> TREE
    STATE --> TREE
    
    TREE --> FOCUS
    TREE --> SR
    
    FOCUS --> WIN_A11Y
    FOCUS --> MAC_A11Y
    FOCUS --> LIN_A11Y
    FOCUS --> AND_A11Y
    FOCUS --> IOS_A11Y
    
    SR --> WIN_A11Y
    SR --> MAC_A11Y
    SR --> LIN_A11Y
    SR --> AND_A11Y
    SR --> IOS_A11Y
    
    WIN_A11Y --> ASSISTIVE
    MAC_A11Y --> ASSISTIVE
    LIN_A11Y --> ASSISTIVE
    AND_A11Y --> ASSISTIVE
    IOS_A11Y --> ASSISTIVE
    
    style START fill:#e8eaf6
    style PROPS fill:#c5cae9
    style ROLE fill:#c5cae9
    style LABEL fill:#c5cae9
    style STATE fill:#c5cae9
    style TREE fill:#9fa8da
    style FOCUS fill:#9fa8da
    style SR fill:#9fa8da
    style WIN_A11Y fill:#7986cb
    style MAC_A11Y fill:#7986cb
    style LIN_A11Y fill:#7986cb
    style AND_A11Y fill:#7986cb
    style IOS_A11Y fill:#7986cb
    style ASSISTIVE fill:#5c6bc0
```

## Font and Image Loading Flow

Media assets are loaded lazily and cached:

```mermaid
sequenceDiagram
    participant COMP as Component
    participant CACHE as Media Cache
    participant LOADER as Media Loader
    participant FILE as File System
    participant PARSER as Format Parser
    
    COMP->>CACHE: Request font/image
    
    alt Asset in cache
        CACHE->>COMP: Return cached asset
    else Asset not in cache
        CACHE->>LOADER: Load asset
        LOADER->>FILE: Read file
        FILE->>LOADER: File data
        LOADER->>PARSER: Parse format
        
        alt Valid format
            PARSER->>LOADER: Parsed data
            LOADER->>CACHE: Store in cache
            CACHE->>COMP: Return asset
        else Invalid format
            PARSER->>LOADER: Error
            LOADER->>COMP: Return error
        end
    end
    
    COMP->>COMP: Use asset for rendering
```

## Error Handling Flow

Errors are handled gracefully throughout the system:

```mermaid
flowchart TB
    START[Operation Attempted]
    
    VALIDATE{Input Valid?}
    PROCESS[Process Operation]
    
    EXEC{Execution Successful?}
    SUCCESS[Return Success]
    
    ERROR{Error Type}
    VALIDATION_ERR[Validation Error]
    IO_ERR[I/O Error]
    PARSE_ERR[Parse Error]
    RUNTIME_ERR[Runtime Error]
    
    LOG[Log Error]
    RECOVER[Attempt Recovery]
    
    FALLBACK{Fallback Available?}
    USE_FALLBACK[Use Fallback]
    PROPAGATE[Propagate Error]
    
    END_SUCCESS([Success])
    END_ERROR([Error Result])
    
    START --> VALIDATE
    VALIDATE -->|No| VALIDATION_ERR
    VALIDATE -->|Yes| PROCESS
    
    PROCESS --> EXEC
    EXEC -->|Yes| SUCCESS
    EXEC -->|No| ERROR
    
    SUCCESS --> END_SUCCESS
    
    ERROR --> VALIDATION_ERR
    ERROR --> IO_ERR
    ERROR --> PARSE_ERR
    ERROR --> RUNTIME_ERR
    
    VALIDATION_ERR --> LOG
    IO_ERR --> LOG
    PARSE_ERR --> LOG
    RUNTIME_ERR --> LOG
    
    LOG --> RECOVER
    RECOVER --> FALLBACK
    
    FALLBACK -->|Yes| USE_FALLBACK
    FALLBACK -->|No| PROPAGATE
    
    USE_FALLBACK --> END_SUCCESS
    PROPAGATE --> END_ERROR
    
    style START fill:#e8f5e9
    style VALIDATE fill:#c8e6c9
    style PROCESS fill:#a5d6a7
    style EXEC fill:#81c784
    style SUCCESS fill:#66bb6a
    style ERROR fill:#ef5350
    style VALIDATION_ERR fill:#f44336
    style IO_ERR fill:#f44336
    style PARSE_ERR fill:#f44336
    style RUNTIME_ERR fill:#f44336
    style LOG fill:#ff7043
    style RECOVER fill:#ffa726
    style USE_FALLBACK fill:#ffb74d
    style PROPAGATE fill:#ff5722
    style END_SUCCESS fill:#4caf50
    style END_ERROR fill:#d32f2f
```

## Performance Optimization Data Flow

The system includes several optimization strategies:

### Lazy Loading
- Fonts loaded only when first used
- Images loaded only when component becomes visible
- Platform backends initialized on demand

### Caching
- Rendered components cached when possible
- Font metrics cached
- Layout calculations cached until invalidated
- Theme colors cached after conversion

### Batching
- Render commands batched per frame
- Event processing batched
- Layout calculations batched

### Async Operations
- Long-running operations use Tokio async
- File I/O is async
- Network operations are async (future feature)

### Memory Management
- RAII ensures proper cleanup
- Reference counting (Arc) for shared data
- Weak references to prevent cycles
- Explicit drop for large resources
