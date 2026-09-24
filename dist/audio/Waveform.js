import { jsx as _jsx, jsxs as _jsxs } from "react/jsx-runtime";
import { useLayoutEffect, useRef } from 'react';
import { fitCanvas } from '../hooks/useCanvas2D.js';
/** Below this many visible envelope buckets, draw from raw samples instead. */
const RAW_THRESHOLD_BUCKETS = 600;
/**
 * Each layer's glow: 2px of the lane's `--stem` colour (white where the host
 * sets none) at this share of its alpha — the resting layer a little under
 * the played one, so played reads as played without its edge blooming.
 *
 * It is drawn INTO the bitmap, once per redraw, and never as a CSS filter. A
 * `drop-shadow` on the element moves pixels, so the compositor widens any
 * damage touching the layer to the whole layer — and the host's playhead
 * crosses every lane on every device pixel it moves. As a CSS filter this one
 * glow cost a weak Intel iGPU most of its frame budget while a song played;
 * in the bitmap it costs one filtered draw per redraw. The layers' colour
 * (saturate, brightness) stays in the stylesheet: measured, a canvas filter
 * does not compute those the way the compositor does, and colour moves no
 * pixels, so it costs nothing there.
 */
const GLOW_BLUR_PX = 2;
const GLOW = { base: 0.2, bright: 0.26 };
/**
 * How far the resting layer's canvas reaches past the lane on every side
 * (audio.css gives `.wave-base` the same inset): the CSS glow spilled about
 * 4px beyond the element's box, and a glow drawn into the bitmap would stop
 * dead at its edge. The played layer needs none — its clip-path always
 * clipped its glow at its own box.
 */
const BASE_PAD_PX = 4;
/** One scratch surface, reused: the envelope is drawn here, then stamped
 *  onto the lane with its glow. Redraws are synchronous and one at a time. */
let scratch = null;
function scratchContext(width, height) {
    if (!scratch) {
        scratch = typeof OffscreenCanvas !== 'undefined' ? new OffscreenCanvas(width, height) : document.createElement('canvas');
    }
    // Resizing also clears it.
    scratch.width = width;
    scratch.height = height;
    return scratch.getContext('2d');
}
/** The glow the stylesheet used to write —
 *  `drop-shadow(0 0 2px color-mix(in srgb, var(--stem, #fff) N%, transparent))`
 *  — with `--stem` resolved here, because a canvas filter has no cascade to
 *  read it from. Lengths are in bitmap pixels: the stamp is drawn untransformed. */
function glowFilter(canvas, layer, dpr) {
    const stem = getComputedStyle(canvas).getPropertyValue('--stem').trim() || '#fff';
    return `drop-shadow(0 0 ${GLOW_BLUR_PX * dpr}px color-mix(in srgb, ${stem} ${Math.round(GLOW[layer] * 100)}%, transparent))`;
}
function drawWave(canvas, peaks, buffer, scale, color, viewStart, viewEnd, layer, bucketColors) {
    const fit = fitCanvas(canvas);
    if (!fit)
        return;
    const { ctx, dpr } = fit;
    // The resting layer's canvas is larger than the lane by BASE_PAD_PX on each
    // side; the envelope is drawn in the lane's own box inside it.
    const pad = layer === 'base' ? BASE_PAD_PX : 0;
    const w = fit.w - 2 * pad;
    const h = fit.h - 2 * pad;
    if (w <= 0 || h <= 0)
        return;
    const sctx = scratchContext(canvas.width, canvas.height);
    if (!sctx || !('filter' in ctx)) {
        // No second surface or no canvas filters: the envelope without its glow
        // beats no envelope at all.
        ctx.translate(pad, pad);
        drawEnvelope(ctx, w, h, peaks, buffer, scale, color, viewStart, viewEnd, bucketColors);
        return;
    }
    sctx.setTransform(dpr, 0, 0, dpr, pad * dpr, pad * dpr);
    drawEnvelope(sctx, w, h, peaks, buffer, scale, color, viewStart, viewEnd, bucketColors);
    ctx.save();
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.filter = glowFilter(canvas, layer, dpr);
    ctx.drawImage(sctx.canvas, 0, 0);
    ctx.restore();
}
/** The waveform itself, in CSS units on a context already scaled to them. */
function drawEnvelope(ctx, w, h, peaks, buffer, scale, color, viewStart, viewEnd, bucketColors) {
    const grad = ctx.createLinearGradient(0, 0, 0, h);
    grad.addColorStop(0, color);
    grad.addColorStop(0.5, color);
    grad.addColorStop(1, color + '99');
    ctx.fillStyle = grad;
    const mid = h / 2;
    const amp = mid - 2;
    const span = viewEnd - viewStart;
    const n = peaks.length;
    if (span * n < RAW_THRESHOLD_BUCKETS && buffer && buffer.length > 0) {
        // Deep zoom: true min/max waveform from the raw samples.
        const ch0 = buffer.getChannelData(0);
        const ch1 = buffer.numberOfChannels > 1 ? buffer.getChannelData(1) : null;
        const total = buffer.length;
        const s0 = Math.max(0, Math.floor(viewStart * total));
        const s1 = Math.min(total, Math.ceil(viewEnd * total));
        if (s1 <= s0) {
            // View is entirely past this (shorter) track — silence line.
            ctx.fillRect(0, mid - 0.75, w, 1.5);
            return;
        }
        const spanS = s1 - s0;
        for (let x = 0; x < w; x++) {
            const a = s0 + Math.floor((x / w) * spanS);
            const b = Math.min(s1, Math.max(a + 1, s0 + Math.floor(((x + 1) / w) * spanS)));
            const stride = Math.max(1, Math.floor((b - a) / 256));
            // Per-channel extremes, matching the envelope layer's max-of-channels —
            // averaging L+R made wide-stereo content collapse at this zoom level.
            let mn = 0;
            let mx = 0;
            for (let i = a; i < b; i += stride) {
                const v0 = ch0[i];
                if (v0 > mx)
                    mx = v0;
                if (v0 < mn)
                    mn = v0;
                if (ch1) {
                    const v1 = ch1[i];
                    if (v1 > mx)
                        mx = v1;
                    if (v1 < mn)
                        mn = v1;
                }
            }
            const top = mid - Math.min(1, mx * scale) * amp;
            const bot = mid - Math.max(-1, mn * scale) * amp;
            ctx.fillRect(x, top, 0.8, Math.max(1.5, bot - top));
        }
        return;
    }
    // Overview: envelope buckets, mirrored around the midline. With
    // `bucketColors` each pixel takes the colour of its loudest bucket — the
    // phone's stem-hued seek bar pattern (each bucket in the loudest lane's
    // hue, so the bar says who is leading). Colour changes are batched: setting
    // fillStyle per pixel would thrash the canvas state for nothing on the
    // long single-hue runs real songs are made of.
    let lastFill = null;
    for (let x = 0; x < w; x++) {
        const f0 = viewStart + (x / w) * span;
        const f1 = viewStart + ((x + 1) / w) * span;
        const b0 = Math.max(0, Math.floor(f0 * n));
        const b1 = Math.min(n, Math.max(b0 + 1, Math.ceil(f1 * n)));
        let peak = 0;
        let peakB = b0;
        // `as number`, not a guard: b0..b1 are clamped to peaks.length above, so
        // the read is always in range, and this is a per-pixel inner loop.
        for (let b = b0; b < b1; b++) {
            const v = peaks[b];
            if (v > peak) {
                peak = v;
                peakB = b;
            }
        }
        if (bucketColors) {
            const c = bucketColors[peakB] ?? color;
            if (c !== lastFill) {
                ctx.fillStyle = c;
                lastFill = c;
            }
        }
        const half = Math.max(0.75, peak * amp);
        ctx.fillRect(x, mid - half, 0.8, half * 2);
    }
}
/**
 * Two stacked copies of the same waveform: a resting base layer and a bright
 * "played" layer clipped by the shared `--p` CSS variable, which the host's
 * playhead loop writes. Progress therefore costs no canvas redraws — but a
 * re-clip still damages the layer's whole visible part, so a host should move
 * `--p` on a clock rather than every frame; see audio.css for that contract.
 */
export function Waveform({ peaks, buffer, scale, color, viewStart, viewEnd, bucketColors }) {
    const baseRef = useRef(null);
    const brightRef = useRef(null);
    const wrapRef = useRef(null);
    useLayoutEffect(() => {
        const redraw = () => {
            if (baseRef.current)
                drawWave(baseRef.current, peaks, buffer, scale, color, viewStart, viewEnd, 'base', bucketColors);
            if (brightRef.current)
                drawWave(brightRef.current, peaks, buffer, scale, color, viewStart, viewEnd, 'bright', bucketColors);
        };
        redraw();
        const ro = new ResizeObserver(redraw);
        if (wrapRef.current)
            ro.observe(wrapRef.current);
        return () => ro.disconnect();
    }, [peaks, buffer, scale, color, viewStart, viewEnd, bucketColors]);
    return (_jsxs("div", { className: "wave", ref: wrapRef, children: [_jsx("canvas", { ref: baseRef, className: "wave-base" }), _jsx("canvas", { ref: brightRef, className: "wave-bright" })] }));
}
