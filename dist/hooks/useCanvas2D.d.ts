/**
 * Size a canvas to its CSS box in DEVICE pixels and hand back a context
 * already scaled, so drawing code can work in CSS units.
 *
 * SingZ had this same six-line preamble copied into four canvases
 * (Waveform, PitchStrip, BeatGrid, TrackStack), each independently capping
 * dpr at 2 — which is not an arbitrary cap: a 3x buffer on a full-width
 * lane stack costs more to rasterise than it visibly buys.
 *
 * Returns null when the element has no layout yet (width or height 0), which
 * happens on the first pass before the parent has sized it. Callers must
 * treat that as "not yet", not as an error.
 */
export declare function fitCanvas(canvas: HTMLCanvasElement, maxDpr?: number): {
    ctx: CanvasRenderingContext2D;
    w: number;
    h: number;
    dpr: number;
} | null;
