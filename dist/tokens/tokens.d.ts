/** Colour, surface and type tokens. Consumers override these to re-theme. */
export declare const tokens: {
    readonly bg: "#12100d";
    readonly panel: "#1b1814";
    readonly 'panel-deep': "#0f0d0a";
    readonly line: "rgba(255, 240, 214, 0.08)";
    readonly 'line-strong': "rgba(255, 240, 214, 0.2)";
    readonly text: "#f4efe6";
    readonly dim: "#9b917e";
    readonly faint: "#6b6355";
    readonly accent: "#ffa028";
    readonly 'accent-deep': "#ff8a1f";
    readonly 'accent-soft': "rgba(255, 160, 40, 0.13)";
    /** Text ON an accent fill — dark enough to read against #ffa028. */
    readonly 'accent-ink': "#241705";
    readonly danger: "#ff7a5c";
    readonly success: "#58d68a";
    readonly 'font-display': "system-ui, sans-serif";
    readonly 'font-mono': "ui-monospace, 'SF Mono', monospace";
};
export type TokenName = keyof typeof tokens;
/** `--sz-accent` etc. The prefix exists so a consumer's own :root cannot
 *  silently collide with the kit's — a collision should be a rename error,
 *  not a cascade coin-flip decided by stylesheet order. */
export declare const cssVar: (name: TokenName) => string;
/** The whole set as a `:root { … }` block. Used to generate tokens.css. */
export declare function toCss(): string;
