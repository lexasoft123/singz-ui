# @singz/ui

The **night-studio** design language — warm tungsten darks, an amber accent,
pill buttons — extracted from [SingZ](https://github.com/lexasoft123/SingZ) so
other apps can share it instead of transcribing it by hand.

```bash
npm install github:lexasoft123/singz-ui#v0.1.0
```

```ts
import '@singz/ui/kit.css'      // the --sz-* custom properties
import { tokens } from '@singz/ui/tokens'  // the same values as plain data
```

## Two entry points, on purpose

CSS custom properties are the natural fit on the web, but React Native cannot
read them — and the phone app is exactly the consumer that drifted first
(eleven tokens, eleven mismatches, including an accent of `#f2c14e` against
the real `#ffa028`). So `tokens.ts` is the source of truth, `tokens.css` is
generated from it, and `dist/` is committed.

`tokens.ts` has **zero imports**. Anything can consume it — Vite, Metro,
vitest, jest, plain node — without dragging a DOM or React Native type
environment along.

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

The `--sz-` prefix is not decoration. A consumer with its own `:root` would
otherwise collide silently and be resolved by stylesheet order; prefixed, a
collision is a rename error instead of a coin flip.

## Fonts are not shipped

The kit *declares* `--sz-font-display` and `--sz-font-mono` with system
fallbacks; the app *assigns* them. Bundling a duplicate `@font-face` set would
double the woff2 in every consumer.

## Status

v0.1.0 is tokens only. Primitives, overlays, audio widgets and Electron chrome
land one layer at a time, because moving *where* a value comes from and
changing *what it is* must never share a commit — otherwise nothing can say
which of the two moved a pixel.

## Development

```bash
npm run build   # tsc -> dist, then generate the CSS from tokens.ts
npm run check   # fails if the committed dist/ is stale
```
