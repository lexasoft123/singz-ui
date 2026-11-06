# SingZ native UIKit

Shared GPUI controls for the desktop prototype. This is a Rust counterpart to
`src/tokens/tokens.ts`, `src/tokens/stems.ts` and the CSS primitives; it does not
load fonts, open windows or own application state.

```toml
[dependencies]
singz-uikit = { path = "../../../singz-ui/gpui" }
```

```rust
use singz_uikit::{pill, card, tokens::*};
// Attach host callbacks after constructing each controlled component.
let button = pill("library", "Library", false)
    .on_click(|_, _, _| { /* navigate */ });
```

The host must register the exact static `SingZ Display 400/500/600/650/700/800`
and `SingZ Mono 400/500` faces before rendering. Each face is selected by family
name and normal weight to prevent native variable-font interpolation drift.
No fonts are installed globally.

Call `singz_uikit::init(cx)` immediately after `gpui_kit::init(cx)` to apply the
SingZ input palette, amber focus ring, caret and selection. Reapply it if the
host changes the component theme later.

Public builders:

- `pill(id, label, on)` — original native player's compact 32px pill.
- `card()` — padded raised content group.
- `section_heading(label)` — uppercase 10px eyebrow.
- `toggle(id, on)` — host-controlled 34 × 20px binary switch.
- `badge(label, on)` — uppercase 9.5px status badge.
- `search_input(&Entity<InputState>)` — shared editable 36px search field.
- `nav_tab(id, label, on)` — settings navigation (7 × 10px padding).
- `select_field(id, label)` — settings option field; host supplies menu callback.
- `glow(color, alpha, radius)` / `rim()` — native player shadows.

Interactive builders stop propagation so header clicks cannot trigger frameless
window dragging. Additional callbacks on the same element remain callable.
Application screens own selection, menus, navigation and InputState subscriptions.

The `tokens` module exposes RGB surface/text/status colours, RGBA translucent
materials, explicit font-family names, and stable stem IDs/names/colours. Call
`rgb` for RGB tokens and `rgba` for translucent tokens.
