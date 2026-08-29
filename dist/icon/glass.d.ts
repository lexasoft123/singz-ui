/** The kit's accent, and the two ends of the light it throws. */
export declare const AMBER = "#ffa028";
export declare const HOT = "#ffe0b0";
export declare const DEEP = "#a8500f";
export interface TileOptions {
    pad?: number;
    top?: string;
    mid?: string;
    bottom?: string;
}
export interface TileResult {
    pad: number;
    size: number;
}
export interface LitOptions {
    w?: number;
    fill?: boolean;
    cap?: CanvasLineCap;
    bloom?: number;
    hot?: string;
    accent?: string;
    deep?: string;
}
export type Mark = (ctx: CanvasRenderingContext2D, pad: number, size: number) => void;
/**
 * macOS-style superellipse. Draw it yourself: macOS does NOT mask an .icns,
 * so a plain rounded rect ships looking subtly wrong next to every system
 * icon, and `border-radius` gets the corner curvature wrong in a way people
 * notice without being able to name.
 */
export declare function squircle(ctx: CanvasRenderingContext2D, x: number, y: number, size: number, n?: number): void;
/**
 * The slab. Returns {pad, size} — the mark and the glass both need them.
 *
 * Kept genuinely dark (#241e18 -> #0b0a08) so the mark is the only lit thing
 * in the frame. The instinct is to lift the tile so it "reads better"; it does
 * the opposite, because the accent then has nothing to be brighter than.
 */
export declare function tile(ctx: CanvasRenderingContext2D, S: number, opts?: TileOptions): TileResult;
/**
 * Draw a shape as LIGHT. Three passes, and all three earn their place:
 *
 *   1. a blurred amber bloom   — light escaping through the glass
 *   2. the solid form          — gradient top-lit, hot to deep
 *   3. a sheen offset to the light, blurred
 *
 * Pass 3 is the one people skip and then wonder why the mark looks flat. Keep
 * its offset small: far enough to see as a separate line and the shape reads
 * as doubled, which is a different and worse mistake.
 *
 *   lit(ctx, z, c => { c.arc(x, y, r, 0, 7) }, { w: 0.07 })        // stroked
 *   lit(ctx, z, c => { c.roundRect(x, y, w, h, r) }, { fill: true })
 *
 * `z` is the tile's `size`, so every width is a fraction of the tile and the
 * mark scales with it.
 */
export declare function lit(ctx: CanvasRenderingContext2D, z: number, draw: (ctx: CanvasRenderingContext2D) => void, o?: LitOptions): void;
/** A dark recess — a socket, a hole, anything the light sits inside. */
export declare function bore(ctx: CanvasRenderingContext2D, cx: number, cy: number, r: number): void;
/**
 * The glass, over everything. Call it LAST.
 *
 * Five things, and the second is the one that actually sells it: the broad
 * specular is COOL (#e2eeff) against warm content. That temperature contrast
 * is what the eye reads as a sheet of glass; a warm highlight over a warm
 * tile just looks like a lighter tile.
 */
export declare function glass(ctx: CanvasRenderingContext2D, S: number, pad: number, size: number): void;
/**
 * The whole icon: tile, your mark, glass.
 *
 *   paint(ctx, 1024, (c, pad, size) => { ...your mark... })
 *
 * The mark gets `pad` and `size` so it can position against the tile rather
 * than the canvas — that is what lets the same mark render at 16 and 1024.
 */
export declare function paint(ctx: CanvasRenderingContext2D, S: number, mark: Mark, opts?: TileOptions): TileResult;
