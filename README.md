# @singz/ui

The **night-studio** design language — warm tungsten darks, an amber accent,
pill buttons — extracted from [SingZ](https://github.com/lexasoft123/SingZ) so
other apps can share it instead of transcribing it by hand.

```bash
npm install --save-dev https://github.com/lexasoft123/singz-ui/archive/refs/tags/v1.0.0.tar.gz
```

```ts
import '@singz/ui/kit.css'                 // tokens + every layer
import { Button, Modal, Waveform } from '@singz/ui'
import { tokens } from '@singz/ui/tokens'  // the same values as plain data
import { STEM_META } from '@singz/ui/stems'
```

## What is in it

| Layer | Contents |
|---|---|
| tokens | colours, surfaces, status, type — one `:root` block, generated from `tokens.ts` |
| primitives | `Button` `Chip` `SegmentedControl` `StatusDot` `LinkButton` `Badge`, the button reset and focus ring |
| overlays | `Modal` `ModalActions`, toast, `useDismissable`, `useModalLock` |
| audio | `Waveform`, `fitCanvas` |
| chrome | `WindowButtons`, `applyPlatformClasses`, frameless-window CSS |

## The class names are the contract

`<Button variant="ghost" size="sm">` renders `class="pill ghost small"`. Those
are the names SingZ already used, kept deliberately: its end-to-end tests
select on `.pill.gear`, on `.settings-card .modal-actions .pill`, on
`.mode-seg button:nth-child(2)`, and on `title="Maximize"` flipping to
`"Restore"`. A prefix plus a compatibility shim would have fixed a third of
that and doubled a 374-rule stylesheet to do it.

Every component appends whatever `className` you pass, so host-specific hooks
ride on top.

## Two entry points, on purpose

CSS custom properties are the natural fit on the web, but React Native cannot
read them — and the phone was exactly the consumer that drifted first: eleven
tokens, eleven mismatches, an accent of `#f2c14e` against the real `#ffa028`.
So `tokens.ts` is the source of truth, `tokens.css` is generated from it, and
both `tokens.ts` and `stems.ts` have **zero imports** so anything can read
them — Vite, Metro, vitest, jest, plain node.

## Theming

`tokens.css` is the only file with literal colours; everything else draws from
`var(--sz-…)`. Override the block and the kit follows:

```css
:root {
  --sz-bg: #f6f0e4;
  --sz-text: #1f1a12;
  --sz-accent: #b5491c;
}
```

`demo/index.html` renders every component with a toggle between night-studio
and a light "atelier" palette. That toggle is the acceptance test, and it is
automated two ways:

```bash
npm run themeable   # fails on any literal colour outside tokens.css
```

plus a runtime pass that samples every surface under both palettes and fails
on any that renders *identically* — a surface that does not move when the
tokens move is drawing from a hardcoded value. That check is what caught
`.pill.danger`, which had looked fine to the static one.

The `--sz-` prefix is not decoration. A consumer with its own `:root` would
otherwise collide silently and be resolved by stylesheet order; prefixed, a
collision is a rename error instead of a coin flip.

## Contracts the host fulfils

Two variables the kit reads but never sets:

- `--stem` — a lane's colour, set inline per lane
- `--p` — playback progress as a percentage, written by the host's rAF loop

`--p` is why progress costs no canvas redraws: `Waveform`'s bright layer is
the same waveform clipped at the playhead, so moving it is one CSS variable
write per frame rather than a repaint per lane.

The host also owns the pause policy for `body.modal-open`. SingZ freezes
infinite animations behind a modal because every invalidated pixel under a
`backdrop-filter` scrim forces a full-window recomposite — a weak Intel iGPU
hit ~95% GPU on exactly that.

## Fonts are not shipped

The kit *declares* `--sz-font-display` and `--sz-font-mono` with system
fallbacks; the app *assigns* them. Bundling a duplicate `@font-face` set would
double the woff2 in every consumer.

Assign the **kit's** variables, not parallel ones of your own — a rule that
reads `var(--sz-font-display)` cannot see your `--font-display`, and the
failure mode is every button in your app silently falling back to system-ui.

## React

Peer range `react@^18.3 || ^19`. Components use `forwardRef` rather than
ref-as-prop, which React 19 allows and 18.3 silently ignores.

## Development

```bash
npm run build   # tsc -> dist, then generate the CSS from tokens.ts
npm run check   # dist/ is current AND no un-tokenised colours
```

`dist/` is committed on purpose: consumers install straight from git, and a
`prepare` build would make each of them install this package's toolchain and
turn a failure here into a confusing failure in their CI.
