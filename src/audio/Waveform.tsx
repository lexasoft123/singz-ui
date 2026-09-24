import { useLayoutEffect, useRef } from 'react'
import { fitCanvas } from '../hooks/useCanvas2D.js'

/** Below this many visible envelope buckets, draw from raw samples instead. */
const RAW_THRESHOLD_BUCKETS = 600

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
const GLOW_BLUR_PX = 2
const GLOW = { base: 0.2, bright: 0.26 } as const
type Layer = keyof typeof GLOW
/**
 * How far the resting layer's canvas reaches past the lane on every side
 * (audio.css gives `.wave-base` the same inset): the CSS glow spilled about
 * 4px beyond the element's box, and a glow drawn into the bitmap would stop
 * dead at its edge. The played layer needs none — its clip-path always
 * clipped its glow at its own box.
 */
const BASE_PAD_PX = 4

/** One scratch surface, reused: the envelope is drawn here, then stamped
 *  onto the lane with its glow. Redraws are synchronous and one at a time. */
let scratch: OffscreenCanvas | HTMLCanvasElement | null = null
function scratchContext(width: number, height: number): OffscreenCanvasRenderingContext2D | CanvasRenderingContext2D | null {
  if (!scratch) {
    scratch = typeof OffscreenCanvas !== 'undefined' ? new OffscreenCanvas(width, height) : document.createElement('canvas')
  }
  // Resizing also clears it.
  scratch.width = width
  scratch.height = height
  return scratch.getContext('2d') as OffscreenCanvasRenderingContext2D | CanvasRenderingContext2D | null
}

/** The glow the stylesheet used to write —
 *  `drop-shadow(0 0 2px color-mix(in srgb, var(--stem, #fff) N%, transparent))`
 *  — with `--stem` resolved here, because a canvas filter has no cascade to
 *  read it from. Lengths are in bitmap pixels: the stamp is drawn untransformed. */
function glowFilter(canvas: HTMLCanvasElement, layer: Layer, dpr: number): string {
  const stem = getComputedStyle(canvas).getPropertyValue('--stem').trim() || '#fff'
  return `drop-shadow(0 0 ${GLOW_BLUR_PX * dpr}px color-mix(in srgb, ${stem} ${Math.round(GLOW[layer] * 100)}%, transparent))`
}

/** What one redraw draws: the envelope and the window onto it. */
interface Envelope {
  peaks: Float32Array
  buffer: AudioBuffer | null
  scale: number
  color: string
  viewStart: number
  viewEnd: number
  bucketColors?: readonly string[]
}

type Fitted = NonNullable<ReturnType<typeof fitCanvas>>

/** Copy the scratch onto a lane canvas through its glow, in bitmap pixels,
 *  `offset` pixels right and down (negative: up and left). */
function stamp(fit: Fitted, source: CanvasImageSource, filter: string, offset = 0): void {
  const { ctx } = fit
  ctx.save()
  ctx.setTransform(1, 0, 0, 1, 0, 0)
  ctx.filter = filter
  ctx.drawImage(source, offset, offset)
  ctx.restore()
}

/** One layer on its own: the envelope drawn onto the scratch and stamped. */
function drawLayer(canvas: HTMLCanvasElement, fit: Fitted, env: Envelope, layer: Layer): void {
  const { ctx, dpr } = fit
  // The resting layer's canvas is larger than the lane by BASE_PAD_PX on each
  // side; the envelope is drawn in the lane's own box inside it.
  const pad = layer === 'base' ? BASE_PAD_PX : 0
  const w = fit.w - 2 * pad
  const h = fit.h - 2 * pad
  if (w <= 0 || h <= 0) return
  const sctx = scratchContext(canvas.width, canvas.height)
  if (!sctx || !('filter' in ctx)) {
    // No second surface or no canvas filters: the envelope without its glow
    // beats no envelope at all.
    ctx.translate(pad, pad)
    drawEnvelope(ctx, w, h, env)
    return
  }
  sctx.setTransform(dpr, 0, 0, dpr, pad * dpr, pad * dpr)
  drawEnvelope(sctx, w, h, env)
  stamp(fit, sctx.canvas, glowFilter(canvas, layer, dpr))
}

/**
 * Both layers. They show the same envelope in the same box — the resting
 * canvas merely reaches BASE_PAD_PX further on every side — so it is drawn
 * ONCE, onto the scratch at the resting canvas's size, and stamped twice:
 * where it lies for the resting layer, and moved back by the pad for the
 * played one. That shift is a whole number of bitmap pixels, so the played
 * layer gets the pixels it would have drawn for itself, to the rounding, for
 * one copy instead of a column per pixel of lane width — which is what a
 * redraw costs.
 * A host that pans a zoomed view redraws every lane per step, and on a weak
 * machine drawing each envelope twice was a frame budget several times over.
 *
 * Anything that breaks the shared geometry — a fractional shift at an odd
 * device pixel ratio, or a stylesheet that sizes the two canvases apart —
 * draws each layer on its own as before. Returns whether both layers drew.
 */
function drawLayers(base: HTMLCanvasElement, bright: HTMLCanvasElement, env: Envelope): boolean {
  const fb = fitCanvas(base)
  const fr = fitCanvas(bright)
  if (fb && fr && fb.dpr === fr.dpr && 'filter' in fb.ctx && 'filter' in fr.ctx) {
    const shift = BASE_PAD_PX * fb.dpr
    const w = fr.w
    const h = fr.h
    if (Number.isInteger(shift) && fb.w - 2 * BASE_PAD_PX === w && fb.h - 2 * BASE_PAD_PX === h) {
      const sctx = scratchContext(base.width, base.height)
      if (sctx) {
        sctx.setTransform(fb.dpr, 0, 0, fb.dpr, shift, shift)
        drawEnvelope(sctx, w, h, env)
        stamp(fb, sctx.canvas, glowFilter(base, 'base', fb.dpr))
        stamp(fr, sctx.canvas, glowFilter(bright, 'bright', fr.dpr), -shift)
        return true
      }
    }
  }
  if (fb) drawLayer(base, fb, env, 'base')
  if (fr) drawLayer(bright, fr, env, 'bright')
  return Boolean(fb && fr)
}

/**
 * The played layer's leading edge: the played canvas copied as it stands,
 * which the stylesheet shows only between `--p` and the host's `--p-edge`
 * (see audio.css). A copy, not a third stamp of the scratch: the played bitmap
 * already holds the finished pixels, glow and all, and a same-size draw at the
 * origin reproduces them for one unfiltered operation. Returns whether it drew.
 */
function copyEdge(bright: HTMLCanvasElement, edge: HTMLCanvasElement): boolean {
  const fe = fitCanvas(edge)
  if (!fe || bright.width === 0 || bright.height === 0) return false
  fe.ctx.setTransform(1, 0, 0, 1, 0, 0)
  fe.ctx.drawImage(bright, 0, 0, edge.width, edge.height)
  return true
}

/** The layout a redraw is made for: every canvas's box, and the ratio that
 *  turns them into bitmaps. The same answer twice means the same drawing. */
function drawnFor(base: HTMLCanvasElement, bright: HTMLCanvasElement, edge: HTMLCanvasElement): string {
  const box = (c: HTMLCanvasElement): string => `${c.clientWidth}x${c.clientHeight}`
  return `${box(base)} ${box(bright)} ${box(edge)} ${window.devicePixelRatio || 1}`
}

/** The waveform itself, in CSS units on a context already scaled to them. */
function drawEnvelope(
  ctx: OffscreenCanvasRenderingContext2D | CanvasRenderingContext2D,
  w: number,
  h: number,
  env: Envelope
): void {
  // Per-bucket hues paint solid columns of their own, and no fade.
  if (env.bucketColors && !readsSamples(env)) {
    drawColumns(ctx, w, h, env)
    return
  }
  // The lane's colour is solid down to the midline and fades from there to
  // 0x99 alpha at the bottom — a gradient that only ever moves alpha. So every
  // column goes in as the flat colour, and ONE fill then takes the lower half
  // down, `destination-out` by the missing 0x66: the same colour and alpha at
  // every pixel as a gradient fill per column, to the rounding. Filling each
  // of a lane's thousand-odd columns with the gradient itself sent the
  // gradient along with every rectangle, and a GPU raster process unpacks
  // each one on its own — on a weak machine that, not the drawing, was most
  // of a redraw. `destination-out`, unlike `source-in`, touches only what it
  // covers, so it needs no canvas-sized layer either.
  ctx.fillStyle = env.color
  drawColumns(ctx, w, h, env)
  const mid = h / 2
  const fade = ctx.createLinearGradient(0, mid, 0, h)
  fade.addColorStop(0, 'rgba(0, 0, 0, 0)')
  fade.addColorStop(1, `rgba(0, 0, 0, ${1 - 0x99 / 0xff})`)
  ctx.globalCompositeOperation = 'destination-out'
  ctx.fillStyle = fade
  // On past the box's edges, not to them: a peak over full scale spills out of
  // the lane (into the resting canvas's pad), and the gradient carried its
  // last stop out there too; and at a fractional pad the box's side edges fall
  // between bitmap pixels, where a fill ending on them would fade the first
  // and last columns only in part. Nothing is drawn beside the box.
  ctx.fillRect(-1, mid, w + 2, h)
  ctx.globalCompositeOperation = 'source-over'
}

/** Whether this window is narrow enough to draw from raw samples. */
function readsSamples({ peaks, buffer, viewStart, viewEnd }: Envelope): boolean {
  return (viewEnd - viewStart) * peaks.length < RAW_THRESHOLD_BUCKETS && buffer !== null && buffer.length > 0
}

/** One rectangle per pixel column, in whatever fill the context holds. */
function drawColumns(
  ctx: OffscreenCanvasRenderingContext2D | CanvasRenderingContext2D,
  w: number,
  h: number,
  env: Envelope
): void {
  const { peaks, buffer, scale, color, viewStart, viewEnd, bucketColors } = env
  const mid = h / 2
  const amp = mid - 2
  const span = viewEnd - viewStart
  const n = peaks.length

  if (buffer && readsSamples(env)) {
    // Deep zoom: true min/max waveform from the raw samples.
    const ch0 = buffer.getChannelData(0)
    const ch1 = buffer.numberOfChannels > 1 ? buffer.getChannelData(1) : null
    const total = buffer.length
    const s0 = Math.max(0, Math.floor(viewStart * total))
    const s1 = Math.min(total, Math.ceil(viewEnd * total))
    if (s1 <= s0) {
      // View is entirely past this (shorter) track — silence line.
      ctx.fillRect(0, mid - 0.75, w, 1.5)
      return
    }
    const spanS = s1 - s0
    for (let x = 0; x < w; x++) {
      const a = s0 + Math.floor((x / w) * spanS)
      const b = Math.min(s1, Math.max(a + 1, s0 + Math.floor(((x + 1) / w) * spanS)))
      const stride = Math.max(1, Math.floor((b - a) / 256))
      // Per-channel extremes, matching the envelope layer's max-of-channels —
      // averaging L+R made wide-stereo content collapse at this zoom level.
      let mn = 0
      let mx = 0
      for (let i = a; i < b; i += stride) {
        const v0 = ch0[i] as number
        if (v0 > mx) mx = v0
        if (v0 < mn) mn = v0
        if (ch1) {
          const v1 = ch1[i] as number
          if (v1 > mx) mx = v1
          if (v1 < mn) mn = v1
        }
      }
      const top = mid - Math.min(1, mx * scale) * amp
      const bot = mid - Math.max(-1, mn * scale) * amp
      ctx.fillRect(x, top, 0.8, Math.max(1.5, bot - top))
    }
    return
  }

  // Overview: envelope buckets, mirrored around the midline. With
  // `bucketColors` each pixel takes the colour of its loudest bucket — the
  // phone's stem-hued seek bar pattern (each bucket in the loudest lane's
  // hue, so the bar says who is leading). Colour changes are batched: setting
  // fillStyle per pixel would thrash the canvas state for nothing on the
  // long single-hue runs real songs are made of.
  let lastFill: string | null = null
  for (let x = 0; x < w; x++) {
    const f0 = viewStart + (x / w) * span
    const f1 = viewStart + ((x + 1) / w) * span
    const b0 = Math.max(0, Math.floor(f0 * n))
    const b1 = Math.min(n, Math.max(b0 + 1, Math.ceil(f1 * n)))
    let peak = 0
    let peakB = b0
    // `as number`, not a guard: b0..b1 are clamped to peaks.length above, so
    // the read is always in range, and this is a per-pixel inner loop.
    for (let b = b0; b < b1; b++) {
      const v = peaks[b] as number
      if (v > peak) {
        peak = v
        peakB = b
      }
    }
    if (bucketColors) {
      const c = bucketColors[peakB] ?? color
      if (c !== lastFill) {
        ctx.fillStyle = c
        lastFill = c
      }
    }
    const half = Math.max(0.75, peak * amp)
    ctx.fillRect(x, mid - half, 0.8, half * 2)
  }
}

export interface WaveformProps {
  /** Precomputed envelope, one value per bucket over the whole buffer. */
  peaks: Float32Array
  /**
   * Needed for the deep-zoom path, which reads raw samples — and NULL when
   * the caller no longer holds them.
   *
   * A host that decodes a whole song per lane may want that memory back once
   * something else owns playback, keeping only `peaks`. Passing null says so,
   * and the deep-zoom path falls back to the envelope: coarser at close zoom,
   * correct at every scale, and drawn without the decoded audio. There is no
   * empty-buffer stand-in to pass instead — Chromium refuses to construct a
   * zero-length AudioBuffer, and a one-sample one draws a flat line.
   */
  buffer: AudioBuffer | null
  scale: number
  color: string
  /** Visible window as fractions of the whole buffer. */
  viewStart: number
  viewEnd: number
  /**
   * One colour per `peaks` bucket — the stem-hued seek bar pattern the phone
   * shipped: each bucket in its loudest lane's hue, so the bar says who is
   * leading. Overview path only; the deep-zoom raw-sample path keeps the
   * single `color` (raw samples carry no per-bucket hue). Missing entries
   * fall back to `color`.
   */
  bucketColors?: readonly string[]
}

/**
 * Stacked copies of the same waveform: a resting base layer, and a bright
 * "played" layer clipped by the shared `--p` CSS variable, which the host's
 * playhead loop writes. Progress therefore costs no canvas redraws — but a
 * re-clip still damages the layer's whole visible part, so a host should move
 * `--p` on a clock rather than every frame. The third canvas is the played
 * layer again, shown only from `--p` to `--p-edge`: a host that writes
 * `--p-edge` every frame gets a played edge exactly at its playhead while the
 * big re-clip stays on the clock; see audio.css for that contract.
 */
export function Waveform({
  peaks,
  buffer,
  scale,
  color,
  viewStart,
  viewEnd,
  bucketColors
}: WaveformProps): React.JSX.Element {
  const baseRef = useRef<HTMLCanvasElement>(null)
  const brightRef = useRef<HTMLCanvasElement>(null)
  const edgeRef = useRef<HTMLCanvasElement>(null)
  const wrapRef = useRef<HTMLDivElement>(null)
  // The current redraw, the layout it last drew for, and the size observer.
  // The observer lives as long as the element and calls whichever redraw is
  // current. It used to be re-created with every new view, and an observer
  // reports the element's size once as soon as it starts watching — so every
  // pan or zoom drew each lane twice, the second time for a size nothing had
  // changed.
  const redrawRef = useRef<() => void>(() => undefined)
  const drawnRef = useRef('')
  const observerRef = useRef<ResizeObserver | null>(null)

  useLayoutEffect(() => {
    const env: Envelope = { peaks, buffer, scale, color, viewStart, viewEnd, bucketColors }
    const redraw = (): void => {
      const base = baseRef.current
      const bright = brightRef.current
      const edge = edgeRef.current
      if (!base || !bright || !edge) return
      const drew = drawLayers(base, bright, env)
      drawnRef.current = copyEdge(bright, edge) && drew ? drawnFor(base, bright, edge) : ''
    }
    redrawRef.current = redraw
    redraw()
    // ...and have it look again. This drawing may be for a size the observer
    // never reports — a parent's layout effect resizes the lane before the
    // frame, or the lane is shown in the frame it was hidden in — and would
    // stand until the next change. Its next report now checks this drawing
    // instead, which costs nothing when the size is the one just drawn.
    const ro = observerRef.current
    const wrap = wrapRef.current
    if (ro && wrap) {
      ro.unobserve(wrap)
      ro.observe(wrap)
    }
  }, [peaks, buffer, scale, color, viewStart, viewEnd, bucketColors])

  // A new size redraws; a report of the size last drawn for does not.
  useLayoutEffect(() => {
    const wrap = wrapRef.current
    if (!wrap) return
    const ro = new ResizeObserver(() => {
      const base = baseRef.current
      const bright = brightRef.current
      const edge = edgeRef.current
      if (base && bright && edge && drawnRef.current === drawnFor(base, bright, edge)) return
      redrawRef.current()
    })
    ro.observe(wrap)
    observerRef.current = ro
    return () => {
      ro.disconnect()
      observerRef.current = null
    }
  }, [])

  return (
    <div className="wave" ref={wrapRef}>
      <canvas ref={baseRef} className="wave-base" />
      <canvas ref={brightRef} className="wave-bright" />
      {/* Carries `wave-bright` too, so every rule written for the played
          layer — a host's included — styles its edge alike. */}
      <canvas ref={edgeRef} className="wave-bright wave-edge" />
    </div>
  )
}
