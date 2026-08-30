/*
 * The night-studio design tokens — the source of truth.
 *
 * Pure data, ZERO imports. That is deliberate and load-bearing: React Native
 * cannot read CSS custom properties, so the phone needs these same values as
 * a plain object, and a file with no imports can be consumed by anything
 * (Vite, Metro, vitest, jest, node) without dragging a DOM or RN type
 * environment along with it.
 *
 * tokens.css is GENERATED from this file — never edit it by hand.
 *
 * Values here are exactly what SingZ's styles.css :root held before the kit
 * existed. Phase 1 moves ownership, not appearance.
 */

/** Colour, surface and type tokens. Consumers override these to re-theme. */
export const tokens = {
  // surfaces
  bg: '#12100d',
  panel: '#1b1814',
  'panel-deep': '#0f0d0a',

  // translucent native surfaces. These are semantic design-system values,
  // not component implementation details: Player, Training and navigation
  // all use the same material recipe.
  'glass-fill': 'rgba(24, 20, 17, 0.55)',
  'glass-line': 'rgba(255, 240, 220, 0.05)',
  'glass-rim': 'rgba(255, 240, 220, 0.14)',
  'control-fill': 'rgba(24, 20, 17, 0.46)',
  'control-line': 'rgba(255, 240, 220, 0.10)',
  'control-rim': 'rgba(255, 240, 220, 0.18)',
  'footer-fill': 'rgba(12, 10, 8, 0.96)',
  shadow: '#000000',

  // hairlines
  line: 'rgba(255, 240, 214, 0.08)',
  'line-strong': 'rgba(255, 240, 214, 0.2)',

  // foreground
  text: '#f4efe6',
  dim: '#9b917e',
  faint: '#6b6355',

  // brand
  accent: '#ffa028',
  'accent-deep': '#ff8a1f',
  'accent-soft': 'rgba(255, 160, 40, 0.13)',
  /** Text ON an accent fill — dark enough to read against #ffa028. */
  'accent-ink': '#241705',
  /** The border of anything engaged — a pressed chip, an `on` pill, a
   *  focused field. It was computed inline in three places (and written by
   *  hand as rgba(255,160,40,.55) in SingZ) before it was named. */
  'accent-line': 'rgba(255, 160, 40, 0.55)',

  /** A surface that floats ABOVE the app — modal cards, popovers. Lighter
   *  than --panel because it sits over a darkened, blurred scrim. */
  'surface-raised': '#1e1a15',

  // status
  danger: '#ff7a5c',
  success: '#58d68a',

  /** Danger as FOREGROUND — a button's text and border, which needs more
   *  lift than --sz-danger has against a dark ground. */
  'danger-strong': '#ff8a7a',
  'danger-strong-line': 'rgba(255, 138, 122, 0.4)',
  'danger-strong-wash': 'rgba(255, 138, 122, 0.12)',

  /** A danger-tinted raised surface and its text — the toast. Separate
   *  tokens because a light theme needs a light tint here, not a dark one. */
  'danger-surface': '#2b1c13',
  'danger-surface-text': '#ffb9a6',
  'danger-surface-line': 'rgba(255, 122, 92, 0.45)',

  /* ——— the room the app sits in ———
   * Two low lamps and a film grain, applied by the chrome layer. They are
   * tokens rather than a fixed backdrop because a light palette wants a
   * fainter warm wash and a MULTIPLY grain — over paper, an overlay grain
   * at 5% reads as dirt. */
  'ambient-warm': 'rgba(255, 160, 40, 0.07)',
  'ambient-cool': 'rgba(69, 214, 181, 0.05)',
  'grain-opacity': '0.05',
  'grain-blend': 'overlay',

  /** The scrollbar thumb, and its hover. Every app that has used this kit
   *  has restyled scrollbars; two of them picked different greys. */
  'scroll-thumb': 'rgba(255, 240, 214, 0.12)',
  'scroll-thumb-hover': 'rgba(255, 240, 214, 0.22)',

  // type. The kit declares the variables but ships no fonts: the two
  // @fontsource-variable packages belong to the app, and a duplicate
  // @font-face set would double the woff2 in every consumer's bundle.
  'font-display': 'system-ui, sans-serif',
  'font-mono': "ui-monospace, 'SF Mono', monospace"
} as const

export type TokenName = keyof typeof tokens

/** `--sz-accent` etc. The prefix exists so a consumer's own :root cannot
 *  silently collide with the kit's — a collision should be a rename error,
 *  not a cascade coin-flip decided by stylesheet order. */
export const cssVar = (name: TokenName): string => `--sz-${name}`

/**
 * ATELIER — the shipped light palette: warm paper, burnt sienna.
 *
 * It lived in demo/index.html as the kit's acceptance test ("if anything
 * stays dark when this is applied, some colour is still hardcoded"). Two
 * apps then re-typed it by hand, which is precisely the drift this package
 * exists to stop — so the test is now the artifact. Only the values that
 * MOVE are listed; everything else is inherited from the block above.
 *
 * Apply with `data-sz-palette="atelier"` on <html> (or any subtree).
 */
export const atelier: Readonly<Partial<Record<TokenName, string>>> = {
  bg: '#f6f0e4',
  panel: '#f1ead8',
  'panel-deep': '#eee6d2',
  'surface-raised': '#fbf6eb',
  'glass-fill': 'rgba(251, 246, 235, 0.7)',
  'glass-line': 'rgba(31, 26, 18, 0.06)',
  'glass-rim': 'rgba(31, 26, 18, 0.16)',
  'control-fill': 'rgba(255, 255, 255, 0.5)',
  'control-line': 'rgba(31, 26, 18, 0.1)',
  'control-rim': 'rgba(31, 26, 18, 0.18)',
  'footer-fill': 'rgba(238, 230, 210, 0.96)',
  line: 'rgba(31, 26, 18, 0.14)',
  'line-strong': 'rgba(31, 26, 18, 0.28)',
  text: '#1f1a12',
  dim: '#594a33',
  faint: '#8e7f66',
  accent: '#b5491c',
  'accent-deep': '#983d16',
  'accent-soft': 'rgba(181, 73, 28, 0.13)',
  'accent-ink': '#fff6ec',
  'accent-line': 'rgba(181, 73, 28, 0.55)',
  danger: '#b03a2e',
  'danger-strong': '#8f2d22',
  'danger-strong-line': 'rgba(143, 45, 34, 0.4)',
  'danger-strong-wash': 'rgba(143, 45, 34, 0.12)',
  'danger-surface': '#f7e2dc',
  'danger-surface-text': '#7a2a1e',
  'danger-surface-line': 'rgba(176, 58, 46, 0.4)',
  success: '#2f7d4f',
  'ambient-warm': 'rgba(181, 73, 28, 0.06)',
  'ambient-cool': 'rgba(31, 26, 18, 0.04)',
  'grain-opacity': '0.025',
  'grain-blend': 'multiply',
  'scroll-thumb': 'rgba(31, 26, 18, 0.18)',
  'scroll-thumb-hover': 'rgba(31, 26, 18, 0.3)'
}

const block = (selector: string, values: Partial<Record<TokenName, string>>): string => {
  const body = (Object.keys(values) as TokenName[])
    .map((k) => `  ${cssVar(k)}: ${values[k]};`)
    .join('\n')
  return `${selector} {\n${body}\n}\n`
}

/**
 * Both palettes as CSS. Used to generate tokens.css.
 *
 * The atelier block is an attribute selector, which ties with `:root` on
 * specificity — so it must come SECOND, and does. A consumer that wants a
 * different light theme overrides the same variables again after this file.
 */
export function toCss(): string {
  return block(':root', tokens) + '\n' + block("[data-sz-palette='atelier']", atelier)
}
