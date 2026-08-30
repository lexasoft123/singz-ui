# design-sync notes — @singz/ui

## Repo setup
- `react`, `react-dom`, `@types/react`, `@types/react-dom` were added as **devDependencies**
  during the first sync. They were missing entirely, so `npm run build` (which runs `tsc`
  over `.tsx`) could not succeed from a fresh clone. Not a sync workaround — a real gap.
- A top-level `"types": "./dist/index.d.ts"` was added to package.json. The package declared
  types only via `exports['.'].types`, and the converter's `projectFor()` reads the
  **top-level** `types`/`typings`, falling back to `<pkgDir>/index.d.ts` — which does not
  exist here. Without it, discovery found **0 components** and the build silently degraded to
  "tokens-only DS". Older TS resolution and some editors read the top-level field too, so
  this is correct packaging regardless.

## Design-system findings (fixed in the kit, not worked around)
- `kit.css` defined `--sz-bg`/`--sz-text` but applied them to **nothing**. Every consumer —
  and every design the Claude Design agent builds — got near-white text on a white page; the
  kit's own `demo/index.html` had been compensating by hand. Fixed with
  `:where(body) { background/color/font-family }` in the primitives layer: zero specificity so
  any host rule wins, `var()` references so a light theme flows through, and body-only because
  CSS propagates body's background to the canvas (targeting `html` too made preview harnesses
  paint a second, conflicting ground).

- `Chip`'s `active` prop set `aria-pressed` and added a class that **nothing styled**, so a
  pressed chip was pixel-identical to a resting one — the whole point of the prop was
  unreachable. Fixed with `.chip.active` using the DS's own pressed treatment (copied from
  `.mode-seg button.on`, not invented). Hosts that colour the pressed state per lane (SingZ
  fills it with `--stem`) outrank it on specificity and are unaffected, which the Chip
  preview's `HostColoured` cell demonstrates.

## Design-system findings (recorded, deliberately NOT fixed here)
- ~~**There is no shared `:disabled` treatment.**~~ **Fixed in 1.4.0** — see the
  second-consumer log below. The condition this entry set (its own commit, not folded
  into a sync) is met, and SingZ pins a tarball tag, so it repaints nothing until that
  pin moves. Original reasoning kept below because it is still the argument for how
  such a change gets made.
- **There is no shared `:disabled` treatment.** Only `.pill.primary:disabled` and
  `.mode-seg button:disabled` dim (`opacity: .45` / `.4`). `.pill.ghost`, `.chip`, `.linkish`
  and `.round-ghost` render a disabled control **identically to a live one** — the control is
  dead to the pointer but reads as clickable.
  Not fixed during this sync on purpose: a generic `.chip:disabled` would also match SingZ's
  own raw `<button className="chip" disabled>` in the BPM stepper (`Transport.tsx:158-186`)
  and several `pill ghost small disabled` in modals, so it is a **visible change to the
  shipped app**, not a mechanical extraction. Under the migration's Rule 3 that belongs in its
  own commit gated by the pixel harness, not folded into a sync. The app currently dims
  disabled controls in five domain-scoped places instead (`.zoom-seg button:disabled`,
  `.variant:disabled`, `.chip.zoom:disabled`, `.src-link:disabled`, `.bpm-entry.disabled`) —
  those five are the evidence the shared rule is wanted, and the values to unify on.
  Until then the `Disabled` cells of `Button`, `Chip` and `LinkButton` each show the kit's
  rendering **beside** a host-dimmed one, and say in their doc comment that the host owns it.

## Preview conventions
- The converter's card scaffold hardcodes `body{background:#fff}`, which outranks a
  zero-specificity rule. Dark-DS previews therefore wrap their content in a local `Surface`
  div using `var(--sz-bg)`/`var(--sz-text)`. Keep that pattern for new previews — without it
  ghost buttons are near-white on white and effectively invisible.
- `cfg.overrides.Modal` / `cfg.overrides.ModalActions` = `{cardMode:"single",
  viewport:"760x520"}` — the scrim is `position: fixed` and escapes a grid cell otherwise.
- Anything `position: fixed` needs an ancestor with a `transform` to be its containing block,
  or it escapes to the viewport. `WindowButtons` previews two titlebars; without
  `transform: translateZ(0)` on each they stacked in one corner. Same trick applies to any
  host embedding the chrome outside the real window.
- `Waveform` is absolutely positioned and reads two **host-owned** variables (`--stem`,
  `--p`). Its preview supplies a sized, positioned parent and sets both — that is the
  integration contract, so it is shown rather than hidden.

## Known render warns (triaged, not new)
- `[RENDER_THIN]` on `Modal.html` and `ModalActions.html` (`rendered height is 0px`) — benign,
  and permanent. The scrim is `position: fixed`, so measured height is 0 while it paints
  full-viewport. Both captured sheets show every cell rendering correctly. Expect these two
  warns on every future sync; they are not a regression to chase.

## Re-sync log — 2026-08-29
- Re-synced against `main` @ `d327641` (v1.3.0). Since the first sync, the kit grew a
  `@singz/ui/native` entry (React Native components — glass surfaces, navigation,
  training/pitch controls) and a `@singz/ui/icon` entry (a canvas-drawing recipe, not
  React components) — both under `src/native/` and `src/icon/`, both **separate package
  exports** from the root `.` entry this sync bundles. Neither shows up here, correctly:
  the converter only bundles `dist/index.js` (`exports['.']`), and RN components
  couldn't render in this tool's browser-based preview harness anyway (no
  react-native-web in the loop). If native ever needs its own claude.ai/design
  presence, that is a **second, separate sync target** — not an extension of this one.
- The 8 new `glass-*`/`control-*`/`footer-fill`/`shadow` tokens added to `tokens.ts` for
  the native kit flowed into `dist/kit.css`/`dist/tokens.css` (they're generated from the
  same `tokens.ts`), so `tokens: 31 defined, 19 referenced` now undercounts — those 8 are
  legitimately unused on the web side. Non-blocking, not a `[TOKENS_MISSING]` situation
  (the reverse: defined-but-unreferenced, which the validator doesn't flag).
- Found `fine` listed in `conventions.md`'s class-name vocabulary line with zero real
  uses anywhere in `dist/kit.css`, `src/`, or the README — a phantom, not a doc lag.
  Removed it from the header. If it was meant to name something real, it needs
  re-adding with the actual class.
- Component set, render hashes, and source keys are byte-identical to the first sync's
  anchor — this run was upload-only (new bundle bytes from the version bump + new
  tokens, and `_ds_sync.json` was current in-project so grades all carried forward at
  zero cost, none re-captured or re-graded).

## Second consumer — 2026-08-30 (v1.4.0)

ChordZ, a chord/lyric notebook, adopted the kit end to end. Everything it had to
write for itself is either a gap in the kit or a bug in it, so each one was fixed
here rather than in the app:

- **`<Button variant="danger">` had no border.** `.pill.danger` set border-*colour*
  only and rode on the border `.ghost` draws — which works for SingZ's hand-written
  `pill ghost small danger` and not at all for the component's own variant. The
  documented promise ("danger is an outline, not a fill") was therefore false for
  every consumer who used the API rather than the class names. Now declares the whole
  border; idempotent for the ghost+danger spelling.
- **`Chip` defaulted `active` to `false`**, so a chip that is not a toggle was still
  announced as an unpressed one. ChordZ's transpose/capo steppers had to drop to raw
  `<button className="chip">` to stay honest. `active` is now genuinely optional and
  aria-pressed is omitted when it is.
- **New: `.pill.on`** (`<Button active>`). The "this is engaged" treatment existed
  four times as host CSS — `.catalog-btn.active` and `.pill.karaoke.active` in SingZ,
  a panel toggle and a helper toggle in ChordZ — always the same three declarations.
- **New: `.chip.wide`.** 26px is right for M/S and wrong for "Maj Pent".
- **New: `.eyebrow`.** There were nine uppercase micro-labels in SingZ at seven sizes
  and six letter-spacings, and six more in ChordZ. A type style, so a class.
- **New: `.modal-title` / `.modal-body`.** The kit shipped the card and the button row
  and nothing for the sentence in between; eight dialogs had each sized their own.
- **New: `.slider` / `.slider.seek`**, in the audio layer because `seek` fills from
  `--p` — the same variable `Waveform` clips on, so a scrub bar and a waveform cannot
  disagree about the playhead.
- **New: `Badge caps={false}`.** ChordZ badges a song's key, and the badge's uppercase
  turned "Am" into "AM" — A minor into A major. Case is content sometimes.
- **New: the room.** `body::before`/`::after` (tungsten lamps + film grain) and
  `:where(.app){z-index:1}` were transcribed by hand in both apps, base64 noise
  included. Now four tokens and two `:where()` rules; a host with its own
  `body::before` still wins.
- **New: shared `:disabled`** at `opacity: .45` for pill/chip/linkish/round-ghost/
  slider, plus `:not(:disabled)` on the hover lifts. This is the entry above, finally
  done — ChordZ's font-size steppers disable at both ends of the range and read as
  live buttons.
- **Shipped: the atelier palette.** It was the acceptance test living in
  `demo/index.html`; ChordZ then re-typed a light palette by hand, which is the exact
  drift this package exists to prevent. Now `[data-sz-palette='atelier']`, generated
  from `tokens.ts` like everything else, and the demo toggles the shipped attribute so
  the test and the artifact are the same thing.
- Also named `--sz-accent-line` (the 55%-accent border that `.chip.active` computed
  inline and SingZ wrote by hand) and `--sz-scroll-thumb`, since both apps had
  restyled scrollbars to different greys.

Not done here: `dist/` is rebuilt but **not committed**, so `npm run check` will fail
on `check-dist` until it is. Everything else (`npm run themeable`, `tsc`) passes.

## Re-sync risks
- `react`/`@types/react` are devDependencies now, so a fresh clone builds — but if they are
  ever removed, discovery silently returns to 0 components rather than erroring loudly.
- The `Surface` wrapper in each preview is a workaround for the harness's white scaffold. If a
  future converter version themes the card, the wrappers become redundant rather than wrong.
- Fonts: the kit deliberately ships **no** `@font-face` — it declares `--sz-font-display` with
  a `system-ui` fallback and expects the host to supply Bricolage Grotesque / Martian Mono.
  Previews therefore render in system-ui. That is correct for this DS, not a `[FONT_MISSING]`
  to chase.
