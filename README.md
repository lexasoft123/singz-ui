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
| tokens | colours, surfaces, status, type — two palettes, generated from `tokens.ts` |
| primitives | `Button` `Chip` `SegmentedControl` `LanguageSwitcher` `StatusDot` `LinkButton` `Badge`, `.eyebrow`, the button reset, the focus ring, scrollbars, one shared `:disabled` and the `prefers-reduced-motion` gate |
| overlays | `Modal` `ModalActions`, `.modal-title` / `.modal-body`, toast, `useDismissable`, `useModalLock` |
| audio | `Waveform`, `fitCanvas`, `.slider` / `.slider.seek` |
| chrome | the room (ambient lamps + film grain), `WindowButtons`, `applyPlatformClasses`, frameless-window CSS |
| native | React Native glass surfaces, navigation, settings, pitch feedback and training controls |
| icon | `paint`, the liquid-glass app-icon recipe as a plain function — see [`recipes/app-icon`](recipes/app-icon) |

## The class names are the contract

`<Button variant="ghost" size="sm">` renders `class="pill ghost small"`. Those
are the names SingZ already used, kept deliberately: its end-to-end tests
select on `.pill.gear`, on `.settings-card .modal-actions .pill`, on
`.mode-seg button:nth-child(2)`, and on `title="Maximize"` flipping to
`"Restore"`. A prefix plus a compatibility shim would have fixed a third of
that and doubled a 374-rule stylesheet to do it.

Every component appends whatever `className` you pass, so host-specific hooks
ride on top.

## Platform entry points, on purpose

CSS custom properties are the natural fit on the web, but React Native cannot
read them — and the phone was exactly the consumer that drifted first: eleven
tokens, eleven mismatches, an accent of `#f2c14e` against the real `#ffa028`.
So `tokens.ts` is the source of truth, `tokens.css` is generated from it, and
both `tokens.ts` and `stems.ts` have **zero imports** so anything can read
them — Vite, Metro, vitest, jest, plain node.

Web components remain on `@singz/ui`; React Native components live on the
separate `@singz/ui/native` entry so Metro never traverses React DOM code:

```tsx
import {
  GlassHeader,
  PitchMeter,
  ReferenceControls,
  nightStudioNativeTheme
} from '@singz/ui/native'
```

The native theme is derived entirely from the same `tokens.ts` object that
generates the web custom properties. Apps may wrap a subtree in
`NativeThemeProvider`; without a provider, components use night-studio.

## Theming

Two palettes ship. **night-studio** is `:root`; **atelier** — warm paper,
burnt sienna — is one attribute away:

```html
<html data-sz-palette="atelier">
```

It used to live only in `demo/index.html`, and two apps then re-typed it by
hand, which is the drift this package exists to stop. Anything else is still
an override: `tokens.css` is the only file with literal colours, everything
else draws from `var(--sz-…)`, so redefining the variables after the kit
re-themes it:

```css
:root {
  --sz-bg: #101418;
  --sz-accent: #4fd1c5;
}
```

`demo/index.html` renders every component with a toggle between the two
shipped palettes. That toggle is the acceptance test, and it is automated
two ways:

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

## The language switcher

```tsx
<LanguageSwitcher
  options={[
    { value: 'en', label: 'English', code: 'EN', hint: t('lang.en') },
    { value: 'zh-CN', label: '简体中文', code: 'ZH', hint: t('lang.zhCN') }
  ]}
  value={prefs.language}
  onChange={(language) => save({ language })}
  system={{ label: t('lang.system'), hint: 'macOS · 简体中文', resolves: 'zh-CN', badge: t('lang.auto') }}
  aria-label={t('prefs.language')}
/>
```

A pill that names the language in use and opens a listbox of the rest;
`variant="list"` lays the same rows out inline for a first-run screen. Each
row is the language's **own** name over its name in the current language —
the reader who needs this control is the one who cannot read the current
language, and the endonym is the one label they are certain to recognise.

An option's `flag` — an inline SVG with the `.lang-flag` class, or an
emoji — takes the row's cell over the code, and the pill wears the flag of
the language in use instead of the globe. The kit draws none itself: a flag
is an asset with a palette of its own, and which flag stands for a language
is the host's decision.

`compact` makes the pill the flag alone — for a title bar or a rail, where a
word would not fit. The current language's name moves to the tooltip, and the
rows underneath are unchanged.

`system` adds a follow-the-machine entry (value `SYSTEM_LANGUAGE`); the pill
then shows the language it resolves to with the `badge` beside it, so
"automatic" and "chosen" are told apart at a glance.

The kit ships **no strings** for it, and no default language list. Every
label comes from the host, which is the only party that knows what its
dictionaries hold.

Keyboard: arrows move, Enter or Space picks, Escape and Tab leave and hand
focus back to the pill. Escape is negotiated with `Modal`: an open menu inside
a dialog takes the press and the dialog stays.

## Words the native components show

The kit carries no strings of its own beyond an English default. The two
native training components that draw words take a `labels` prop, so a host
in another language passes its own — anything left out stays English:

```tsx
<ReferenceControls {...props} labels={{ title: t('refSound'), pitchWindow: t('pitchWindow'),
  volume: t('refVolume'), volumeValue: (p) => t('percent', { p }),
  decrease: t('lower'), increase: t('raise') }} />
<PitchMeter {...props} labels={{ flat: t('flat'), sharp: t('sharp'),
  youAreSinging: t('youAreSinging'), progress: (instruction, p) => t('holdProgress', { instruction, p }) }} />
```

`title`, `pitchWindow`, `flat`, `sharp` and `youAreSinging` are shown
upper-cased (`toLocaleUpperCase`), the way the English always was.

## Contracts the host fulfils

Two variables the kit reads but never sets:

- `--stem` — a lane's colour, set inline per lane
- `--p` — playback progress as a percentage, written by the host's rAF loop

`--p` is why progress costs no canvas redraws: `Waveform`'s bright layer is
the same waveform clipped at the playhead, so moving it is one CSS variable
write rather than a repaint per lane. `.slider.seek` fills its track from the
same variable, so a scrub bar and a waveform cannot disagree about where the
playhead is.

A re-clip is not free, though: it damages the layer's whole visible part, so
every move of `--p` recomposites the played part of every lane. A host that
draws its own playhead line should move the line every frame and `--p` on a
clock — SingZ moves it at 4 Hz while a song rolls and exactly on pause, seek
and zoom, which is invisible on a played edge that is only a brightness step.
For the same reason `Waveform` draws its 2px lane glow into the canvas once
per redraw instead of as a CSS `drop-shadow`: a drop-shadow moves pixels, so
the compositor widens any damage touching the layer to the whole layer.
`--stem` is still where that glow takes its colour from, read from the canvas
when it draws. The layers' saturation and brightness stay CSS filters (they
move no pixels), so a host `filter` on `.wave-base` / `.wave-bright` still
replaces those — but no longer removes the glow, which is in the bitmap now.

A redraw is the one thing here that costs per event. A new window onto the
audio — a pan, a zoom, a resize — redraws every lane: one pass over the
envelope per lane, a column per pixel of lane width, with both layers stamped
from it (two passes where they cannot share one: a device pixel ratio at
which the 4px pad is not a whole number of bitmap pixels, or a stylesheet
that sizes the two canvases apart). A host that pans a zoomed view on every
wheel event pays that pass per event and per lane, so it should not hand
`Waveform` more new windows than it has frames to show them in — SingZ
applies a wheel event at once when nothing is waiting, and holds the ones
that follow for the next frame, where they land together.

The room — two ambient lamps and a film grain — is painted on `body::before`
and `body::after` at `:where()` specificity. A host with its own `body::before`
keeps it; a host that wants no room sets `--sz-ambient-warm` /
`--sz-ambient-cool` to `transparent` and `--sz-grain-opacity` to `0`.

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
