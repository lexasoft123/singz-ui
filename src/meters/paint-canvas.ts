import { meterBrickFill, meterIndex, meterZone, type PeakMeterMotion } from './peak-meter-motion.js'

export interface MeterPalette { zones: readonly string[]; faint: string; danger: string; text: string }
/** One bitmap, no per-brick layers. Quantize damage to bitmap pixels and alpha bytes. */
export function paintMeterCanvas(ctx: CanvasRenderingContext2D, motion: PeakMeterMotion, count: number,
  width: number, height: number, gap: number, radius: number, palette: MeterPalette, clipped: boolean, previous: string): string {
  const brick = Math.max(0, (width - gap * (count - 1)) / count)
  const fills = Array.from({ length: count }, (_, i) => Math.round(meterBrickFill(motion.liveDb, i, count) * brick))
  const trail = motion.trail.map(value => Math.round(value * 255))
  const peak = motion.peakDb > -60 ? Math.max(0, meterIndex(motion.peakDb, count)) : -1
  const key = [width, height, gap, radius, clipped, peak, ...palette.zones, palette.faint, palette.danger, palette.text, ...fills, ...trail].join('|')
  if (key === previous) return previous
  ctx.clearRect(0, 0, width, height)
  ctx.save()
  ctx.beginPath()
  for (let i = 0; i < count; ++i) ctx.roundRect(i * (brick + gap), 0, brick, height, radius)
  ctx.fillStyle = clipped ? palette.danger : palette.faint
  ctx.globalAlpha = clipped ? .24 : .3
  ctx.fill()
  ctx.clip()
  for (let i = 0; i < count; ++i) {
    ctx.fillStyle = palette.zones[meterZone(i, count)] ?? palette.faint
    const energy = trail[i] ?? 0, fill = fills[i] ?? 0
    if (energy) { ctx.globalAlpha = energy / 255; ctx.fillRect(i * (brick + gap), 0, brick, height) }
    if (fill) { ctx.globalAlpha = 1; ctx.fillRect(i * (brick + gap), 0, fill, height) }
  }
  ctx.restore()
  if (peak >= 0) {
    ctx.globalAlpha = 1; ctx.fillStyle = palette.zones[meterZone(peak, count)] ?? palette.faint
    ctx.beginPath(); ctx.roundRect(peak * (brick + gap), 0, brick, height, radius); ctx.fill()
    ctx.globalAlpha = .45; ctx.fillStyle = palette.text
    ctx.fillRect(peak * (brick + gap), 0, brick, Math.max(1, radius * (count === 16 ? 1 : 2)))
    ctx.globalAlpha = 1
  }
  return key
}
