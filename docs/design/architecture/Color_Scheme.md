# Architecture Diagram Color Scheme

## Overview

All diagrams in the Engage UX architecture documentation use a standardized, WCAG AAA-compliant color scheme. Every color has been verified to provide at least a 7:1 contrast ratio with black text, meeting the highest accessibility standards.

## Color Palette

### Architectural Layers

The color scheme follows the architectural layers of the system, with lighter colors for higher-level abstractions and darker colors for lower-level implementations:

#### Application Layer (Lightest)
- **#e1f5ff** - Light Blue 50 (18.71:1 contrast) - Application code, user-facing layer
- **#e3f2fd** - Blue 50 (18.39:1 contrast) - High-level constructs

#### Component/UI Layer (Light)
- **#b3e5fc** - Light Blue 100 (15.53:1 contrast) - UI components
- **#bbdefb** - Blue 100 (14.95:1 contrast) - Theme system
- **#c8e6c9** - Green 100 (15.62:1 contrast) - Containers, layouts

#### Core System Layer (Medium)
- **#81d4fa** - Light Blue 200 (12.73:1 contrast) - Core systems (Color, Event, Input, A11Y, Media, Render)
- **#90caf9** - Blue 200 (12.00:1 contrast) - State management
- **#a5d6a7** - Green 200 (12.78:1 contrast) - Processing, logic

#### Platform/OAL Layer (Medium-Dark)
- **#4fc3f7** - Light Blue 300 (10.48:1 contrast) - Platform detection, factory
- **#03a9f4** - Light Blue 500 (7.99:1 contrast) - Window/Graphics backends
- **#81c784** - Green 300 (10.44:1 contrast) - Platform services

#### OS/Native Layer (Darkest)
- **#03a9f4** - Light Blue 500 (7.99:1 contrast) - OS APIs (all platforms)
- **#29b6f6** - Light Blue 400 (9.12:1 contrast) - Native services

### Special Purpose Colors

#### Accessibility
- **#e8eaf6** - Indigo 50 (17.53:1) - Accessibility components
- **#c5cae9** - Indigo 100 (13.00:1) - ARIA attributes, focus management
- **#9fa8da** - Indigo 300 (9.09:1) - Accessibility services, screen readers

#### Data/Content
- **#fff3e0** - Orange 50 (19.15:1) - Data, content
- **#ffe0b2** - Orange 100 (16.56:1) - Parsing, validation
- **#ffcc80** - Orange 200 (14.20:1) - Transformation
- **#ffa726** - Orange 400 (10.81:1) - Processing

#### Events/Messages
- **#e8f5e9** - Green 50 (18.67:1) - Event sources
- **#c8e6c9** - Green 100 (15.62:1) - Event processing
- **#a5d6a7** - Green 200 (12.78:1) - Event distribution

#### State/Status
- **#4caf50** - Green 500 (7.56:1) - Success, complete
- **#81c784** - Green 300 (10.44:1) - Active, processing
- **#66bb6a** - Green 400 (8.88:1) - Intermediate success state
- **#ffeb3b** - Yellow 500 (17.20:1) - Warning, attention
- **#ffc107** - Amber 500 (12.88:1) - Secondary actions
- **#cddc39** - Lime 600 (13.89:1) - Alternative success

#### Errors/Critical
- **#ff7043** - Deep Orange 400 (7.65:1) - Errors, critical states
- **#ff9800** - Orange 500 (9.74:1) - High priority

#### Meta/Special
- **#f3e5f5** - Purple 50 (17.34:1) - Theming, styling
- **#e1bee7** - Purple 100 (12.72:1) - Configuration
- **#ce93d8** - Purple 300 (8.79:1) - Metadata, display complete
- **#9e9e9e** - Grey 500 (7.84:1) - Testing, mocks

## Usage Guidelines

### 1. Layer Consistency
Always use the same color for the same architectural layer across all diagrams. For example:
- Application code is always **#e1f5ff**
- UI components are always **#b3e5fc**
- Core systems are always **#81d4fa**
- Platform backends are always **#03a9f4**

### 2. Semantic Consistency
Use the same color family for similar concepts:
- **Blue family** - System architecture layers
- **Green family** - Events, messages, success states
- **Orange family** - Data processing, transformations
- **Indigo family** - Accessibility features
- **Purple family** - Theming, configuration, metadata

### 3. Visual Hierarchy
Follow the natural top-to-bottom flow:
- Lighter colors (higher contrast) = Higher abstraction levels
- Darker colors (lower contrast, still 7:1+) = Lower implementation levels

### 4. Color Families

#### Light Blue Family (System Architecture)
- #e1f5ff, #e3f2fd, #b3e5fc, #bbdefb, #81d4fa, #90caf9, #4fc3f7, #29b6f6, #03a9f4
- **Use for**: Architectural layers from application to OS

#### Green Family (Events, Processing, Success)
- #e8f5e9, #c8e6c9, #a5d6a7, #81c784, #66bb6a, #4caf50
- **Use for**: Event flow, processing stages, success states

#### Orange Family (Data, Transformation)
- #fff3e0, #ffe0b2, #ffcc80, #ffb74d, #ffa726, #ff9800
- **Use for**: Data flow, parsing, transformation, high priority

#### Indigo Family (Accessibility)
- #e8eaf6, #c5cae9, #9fa8da
- **Use for**: Accessibility features, ARIA, screen readers

#### Purple Family (Meta/Configuration)
- #f3e5f5, #e1bee7, #ce93d8
- **Use for**: Theming, configuration, metadata

### 5. Contrast Requirements

All colors in this palette meet WCAG AAA Level requirements:
- **Normal text**: Minimum 7:1 contrast ratio ✓
- **Large text**: Minimum 4.5:1 contrast ratio ✓
- **UI components**: Minimum 3:1 contrast ratio ✓

This ensures:
- Excellent readability for all users
- Support for users with color vision deficiencies
- Compliance with international accessibility standards

## Example Applications

### System Architecture Diagram
```
Application Layer:  #e1f5ff (lightest)
Component Layer:    #b3e5fc
Core Layer:         #81d4fa
Platform Layer:     #4fc3f7, #03a9f4
OS Layer:           #03a9f4 (darkest in this context)
```

### Data Flow Diagram
```
User Input:         #e8f5e9 (green - events)
Event Processing:   #c8e6c9 (green)
Data Transform:     #ffcc80 (orange)
Backend:            #ff7043 (critical/output)
Display:            #ce93d8 (purple - metadata)
```

### Component Architecture
```
Component Trait:    #e8eaf6 (base/abstract)
Informational:      #b3e5fc (light blue)
Interactive:        #81d4fa (medium blue)
Layout:             #03a9f4 (darker blue)
```

## Verification

All colors have been programmatically verified for WCAG AAA compliance using the relative luminance formula defined in WCAG 2.1 specification. The contrast ratios are calculated as:

```
Contrast Ratio = (L1 + 0.05) / (L2 + 0.05)
```

Where L1 is the relative luminance of the lighter color and L2 is the relative luminance of the darker color.

## Maintenance

When adding new diagrams:
1. Choose colors from this palette
2. Match the architectural layer or semantic meaning
3. Maintain visual hierarchy (light to dark)
4. Verify 7:1+ contrast ratio if using custom colors
5. Update this document if new color patterns emerge

## Color Accessibility Testing

You can verify any color combination using online tools:
- [WebAIM Contrast Checker](https://webaim.org/resources/contrastchecker/)
- [Contrast Ratio Calculator](https://contrast-ratio.com/)

For this project, all colors are tested against black text (#000000) to ensure maximum accessibility.
