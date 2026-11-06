# SingZ native UIKit

Shared GPUI controls for the desktop prototype. This is a Rust counterpart to
`src/tokens/tokens.ts`, `src/tokens/stems.ts` and the CSS primitives; it does not
install system fonts, open windows or own application state.

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

The crate owns the exact `SingZ Display 400/500/600/650/700/800` and
`SingZ Mono 400/500/700` font files in `assets/fonts`. `init(cx)` registers
these faces once per App; hosts need no font files or `add_fonts` call.
Each visual weight is a separate family selected with normal style in the UIKit API.
Windows aliases normalize only style metadata to match DirectWrite; CoreText keeps its
original metadata and fallback behavior. Both variants contain identical static glyph
outlines and layout tables, without variable interpolation.
The same assets and original variable Unicode subsets ship in npm `dist/fonts`,
with `@singz/ui/fonts.css` as the web registration entry. OFL licenses ship beside them.
Regenerate with `python3 scripts/gen-fonts.py` from the UIKit root, then `npm run build`.

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

`window_buttons(maximized)` provides the frameless Windows header controls:
46×52px minimize, maximize/restore and close buttons, with UIKit hover colors
and native window-control hit areas. Add them after modal content so they stay
available, and reserve 150px at the right of the host's 52px header. The host
owns window creation, transparent titlebar policy, dragging and outer framing.
