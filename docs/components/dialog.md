# Dialog Component

Modal dialog components for displaying important content and collecting user input.

## Overview

The Dialog components (AlertDialog, ConfirmDialog, Modal, FileDialog) display content in overlay windows that require user interaction before continuing.

## Alert Dialog

A simple dialog for displaying messages.

```rust
use engage_ux_components::AlertDialog;

let mut alert = AlertDialog::new(1);
alert.set_title("Success");
alert.set_message("Your changes have been saved.");
alert.set_button_text("OK");

alert.show();
```

## Confirm Dialog

A dialog for confirming actions.

```rust
use engage_ux_components::ConfirmDialog;

let mut confirm = ConfirmDialog::new(1);
confirm.set_title("Delete File");
confirm.set_message("Are you sure you want to delete this file?");
confirm.set_confirm_text("Delete");
confirm.set_cancel_text("Cancel");

confirm.set_on_confirm(|_| {
    delete_file();
});

confirm.show();
```

## Custom Modal

A flexible modal for custom content.

```rust
use engage_ux_components::Modal;

let mut modal = Modal::new(1);
modal.set_title("Edit Profile");

// Add custom content
modal.add_child(Box::new(form));

modal.set_on_close(|_| {
    // Handle close
});

modal.show();
```

## File Dialogs

### Open Dialog

```rust
use engage_ux_components::FileDialog;

let mut open = FileDialog::open(1);
open.set_title("Open File");
open.set_file_types(vec!["txt", "md", "rs"]);

open.set_on_select(|path| {
    open_file(&path);
});

open.show();
```

### Save Dialog

```rust
let mut save = FileDialog::save(1);
save.set_title("Save As");
save.set_default_name("document.txt");
save.set_file_types(vec!["txt", "md"]);

save.set_on_select(|path| {
    save_file(&path);
});

save.show();
```

## Properties

| Property | Type | Description | Default |
|----------|------|-------------|---------|
| `title` | `String` | Dialog title | `""` |
| `message` | `String` | Dialog message | `""` |
| `visible` | `bool` | Dialog visibility | `false` |
| `modal` | `bool` | Block background | `true` |

## Accessibility

Dialogs trap focus and are keyboard accessible:

```rust
dialog.set_aria_label("Important message");
dialog.set_aria_modal(true);
```

## Related Components

- [Toast](toast.md) - For non-blocking messages
- [Banner](banner.md) - For persistent messages

---

[← Back to Components](index.md)
