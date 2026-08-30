/** Colour, surface and type tokens. Consumers override these to re-theme. */
export declare const tokens: {
    readonly bg: "#12100d";
    readonly panel: "#1b1814";
    readonly 'panel-deep': "#0f0d0a";
    readonly 'glass-fill': "rgba(24, 20, 17, 0.55)";
    readonly 'glass-line': "rgba(255, 240, 220, 0.05)";
    readonly 'glass-rim': "rgba(255, 240, 220, 0.14)";
    readonly 'control-fill': "rgba(24, 20, 17, 0.46)";
    readonly 'control-line': "rgba(255, 240, 220, 0.10)";
    readonly 'control-rim': "rgba(255, 240, 220, 0.18)";
    readonly 'footer-fill': "rgba(12, 10, 8, 0.96)";
    readonly shadow: "#000000";
    readonly line: "rgba(255, 240, 214, 0.08)";
    readonly 'line-strong': "rgba(255, 240, 214, 0.2)";
    readonly text: "#f4efe6";
    readonly dim: "#9b917e";
    readonly faint: "#6b6355";
    readonly accent: "#ffa028";
    readonly 'accent-deep': "#ff8a1f";
    /** The light end of the primary button's gradient — a highlight above
     *  --accent-deep, not a shade of it. It was a literal in primitives.css
     *  and therefore stayed amber when the atelier palette arrived, giving
     *  that palette a light-amber-to-dark-rust button under near-white ink at
     *  1.54:1. A gradient stop in a themed component is a token. */
    readonly 'accent-lift': "#ffbe58";
    readonly 'accent-soft': "rgba(255, 160, 40, 0.13)";
    /** Text ON an accent fill — dark enough to read against #ffa028. */
    readonly 'accent-ink': "#241705";
    /** The border of anything engaged — a pressed chip, an `on` pill, a
     *  focused field. It was computed inline in three places (and written by
     *  hand as rgba(255,160,40,.55) in SingZ) before it was named. */
    readonly 'accent-line': "rgba(255, 160, 40, 0.55)";
    /** The focus ring. Measured 4.86:1 on --panel and 5.06:1 on --bg. */
    readonly 'focus-ring': "rgba(255, 160, 40, 0.7)";
    /** A surface that floats ABOVE the app — modal cards, popovers. Lighter
     *  than --panel because it sits over a darkened, blurred scrim. */
    readonly 'surface-raised': "#1e1a15";
    readonly danger: "#ff7a5c";
    readonly success: "#58d68a";
    /** Danger as FOREGROUND — a button's text and border, which needs more
     *  lift than --sz-danger has against a dark ground. */
    readonly 'danger-strong': "#ff8a7a";
    readonly 'danger-strong-line': "rgba(255, 138, 122, 0.4)";
    readonly 'danger-strong-wash': "rgba(255, 138, 122, 0.12)";
    /** A danger-tinted raised surface and its text — the toast. Separate
     *  tokens because a light theme needs a light tint here, not a dark one. */
    readonly 'danger-surface': "#2b1c13";
    readonly 'danger-surface-text': "#ffb9a6";
    readonly 'danger-surface-line': "rgba(255, 122, 92, 0.45)";
    readonly 'ambient-warm': "rgba(255, 160, 40, 0.07)";
    readonly 'ambient-cool': "rgba(69, 214, 181, 0.05)";
    readonly 'grain-opacity': "0.05";
    readonly 'grain-blend': "overlay";
    /** The scrollbar thumb, and its hover. Every app that has used this kit
     *  has restyled scrollbars; two of them picked different greys. */
    readonly 'scroll-thumb': "rgba(255, 240, 214, 0.12)";
    readonly 'scroll-thumb-hover': "rgba(255, 240, 214, 0.22)";
    readonly 'font-display': "system-ui, sans-serif";
    readonly 'font-mono': "ui-monospace, 'SF Mono', monospace";
};
export type TokenName = keyof typeof tokens;
/** `--sz-accent` etc. The prefix exists so a consumer's own :root cannot
 *  silently collide with the kit's — a collision should be a rename error,
 *  not a cascade coin-flip decided by stylesheet order. */
export declare const cssVar: (name: TokenName) => string;
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
export declare const atelier: Readonly<Partial<Record<TokenName, string>>>;
/**
 * Both palettes as CSS. Used to generate tokens.css.
 *
 * The atelier block is an attribute selector, which ties with `:root` on
 * specificity — so it must come SECOND, and does. A consumer that wants a
 * different light theme overrides the same variables again after this file.
 */
export declare function toCss(): string;
