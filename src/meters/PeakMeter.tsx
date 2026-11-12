import { forwardRef, memo, useCallback, useEffect, useImperativeHandle, useRef } from 'react'
import { cx } from '../util/cx.js'
import {
  emptyPeakMeter,
  meterBrickFill,
  meterIndex,
  meterZone,
  peakMeterAtRest,
  stepPeakMeter,
  type PeakMeterMotion
} from './peak-meter-motion.js'

export interface PeakMeterProps {
  /** Live level in dBFS. `-Infinity` is silence. */
  db: number
  /** Peak level in dBFS; defaults to `db`. */
  peakDb?: number
  /** 16 bricks / 9px instead of 40 bricks / 16px. */
  mini?: boolean
  /** Tints the unlit bricks red. */
  clipped?: boolean
  /** Changing it (a new song, a new input) clears the meter instantly. */
  identity?: object
  /** `false` empties the meter and stops all work. Default true. */
  enabled?: boolean
  /** Height multiplier for TV / desktop scale. Default 1. */
  size?: number
  className?: string
  style?: React.CSSProperties
}

/** Drive the meter without any React render at all. */
export interface PeakMeterHandle {
  push(db: number, peakDb?: number): void
}

const MIN_STEP_MS = 4

const Bricks = memo(function Bricks({ count }: { count: number }): React.JSX.Element {
  return (
    <>
      {Array.from({ length: count }, (_, i) => (
        <i key={i} className={`sz-meter-brick z${meterZone(i, count)}`}>
          <b className="sz-meter-trail" />
          <b className="sz-meter-live" />
        </i>
      ))}
      <i className="sz-meter-peak z0" />
    </>
  )
})

/**
 * The SingZ level meter. Ballistics (`stepPeakMeter`) run in one rAF loop that
 * writes straight to the DOM; React renders the bricks once. The loop stops
 * when the meter is at rest, when the document is hidden and when `enabled`
 * is false, and prefers-reduced-motion keeps the held peak but drops the
 * falling and trailing motion.
 */
export const PeakMeter = forwardRef<PeakMeterHandle, PeakMeterProps>(function PeakMeter(
  { db, peakDb = db, mini = false, clipped = false, identity, enabled = true, size = 1, className, style },
  ref
) {
  const count = mini ? 16 : 40
  const root = useRef<HTMLDivElement>(null)
  const sim = useRef({
    count,
    motion: emptyPeakMeter(count) as PeakMeterMotion,
    want: -Infinity,
    wantPeak: -Infinity,
    target: -Infinity,
    peakTarget: -Infinity,
    enabled,
    reduced: false,
    hidden: false,
    raf: 0,
    last: 0,
    acc: 0,
    painted: { trail: [] as number[], live: [] as number[], peakIndex: -2, peakZone: -1 }
  })

  const paint = useCallback(() => {
    const el = root.current
    if (!el) return
    const s = sim.current
    const { motion, count: n, painted } = s
    const bricks = el.children
    for (let i = 0; i < n; i++) {
      const brick = bricks[i]
      if (!brick) break
      const fill = meterBrickFill(motion.liveDb, i, n)
      const trail = motion.trail[i] ?? 0
      if (painted.live[i] !== fill) {
        painted.live[i] = fill
        ;(brick.lastElementChild as HTMLElement).style.transform = `scaleX(${fill})`
      }
      if (painted.trail[i] !== trail) {
        painted.trail[i] = trail
        ;(brick.firstElementChild as HTMLElement).style.opacity = String(trail)
      }
    }
    const marker = bricks[n] as HTMLElement | undefined
    if (!marker) return
    const index = Math.max(0, meterIndex(motion.peakDb, n))
    const shown = motion.peakDb > -60
    const key = shown ? index : -1
    if (painted.peakIndex !== key) {
      painted.peakIndex = key
      marker.style.opacity = shown ? '1' : '0'
      marker.style.transform = `translateX(calc(${index} * (100% + var(--sz-meter-gap))))`
      const zone = meterZone(index, n)
      if (painted.peakZone !== zone) {
        painted.peakZone = zone
        marker.className = `sz-meter-peak z${zone}`
      }
    }
  }, [])

  const reset = useCallback(() => {
    const s = sim.current
    s.motion = emptyPeakMeter(s.count)
    paint()
  }, [paint])

  const stop = useCallback(() => {
    const s = sim.current
    if (s.raf) cancelAnimationFrame(s.raf)
    s.raf = 0
  }, [])

  const frame = useCallback((now: number) => {
    const s = sim.current
    s.raf = 0
    const elapsed = s.last > 0 ? Math.min(100, Math.max(0, now - s.last)) : 0
    s.last = now
    s.acc += elapsed
    if (s.acc >= MIN_STEP_MS) {
      s.motion = stepPeakMeter(s.motion, s.target, s.acc, s.peakTarget)
      s.acc = 0
      paint()
      if (peakMeterAtRest(s.motion)) return
    }
    s.raf = requestAnimationFrame(frame)
  }, [paint])

  /** Re-evaluate what the meter should be doing after any input changed. */
  const sync = useCallback(() => {
    const s = sim.current
    const live = s.enabled && !s.hidden
    s.target = live ? s.want : -Infinity
    s.peakTarget = live ? s.wantPeak : -Infinity
    if (!live) {
      stop()
      reset()
    } else if (s.reduced) {
      // Hold the peak, omit falling and trailing.
      stop()
      const snapshot = stepPeakMeter(s.motion, s.target, 100, s.peakTarget)
      snapshot.liveDb = Number.isFinite(s.target) ? Math.max(-60, Math.min(0, s.target)) : -60
      snapshot.trail.fill(0)
      if (snapshot.liveDb <= -60) snapshot.peakDb = -60
      s.motion = snapshot
      paint()
    } else if (!s.raf) {
      // Nothing to animate: an empty meter fed silence stays dark and idle.
      if (peakMeterAtRest(s.motion) && !(s.target > -60) && !(s.peakTarget > -60)) return
      s.last = 0
      s.acc = 0
      s.raf = requestAnimationFrame(frame)
    }
  }, [frame, paint, reset, stop])

  const push = useCallback((nextDb: number, nextPeak: number = nextDb) => {
    const s = sim.current
    s.want = nextDb
    s.wantPeak = nextPeak
    sync()
  }, [sync])

  useImperativeHandle(ref, () => ({ push }), [push])

  // Environment: document visibility and reduced motion.
  useEffect(() => {
    const s = sim.current
    s.hidden = document.hidden
    const motionQuery = typeof matchMedia === 'function' ? matchMedia('(prefers-reduced-motion: reduce)') : null
    s.reduced = motionQuery?.matches ?? false
    const onVisibility = (): void => { s.hidden = document.hidden; sync() }
    const onMotion = (): void => { s.reduced = motionQuery?.matches ?? false; sync() }
    document.addEventListener('visibilitychange', onVisibility)
    motionQuery?.addEventListener?.('change', onMotion)
    sync()
    return () => {
      document.removeEventListener('visibilitychange', onVisibility)
      motionQuery?.removeEventListener?.('change', onMotion)
      stop()
    }
  }, [stop, sync])

  // A new song / size starts from an empty meter.
  useEffect(() => {
    const s = sim.current
    s.count = count
    s.painted = { trail: [], live: [], peakIndex: -2, peakZone: -1 }
    reset()
    sync()
  }, [identity, count, reset, sync])

  // New values only touch refs; the rAF loop does the rest.
  useEffect(() => {
    const s = sim.current
    s.enabled = enabled
    s.want = db
    s.wantPeak = peakDb
    sync()
  }, [db, peakDb, enabled, sync])

  const vars = { '--sz-meter-count': count, '--sz-meter-scale': size, ...style } as React.CSSProperties
  return (
    <div ref={root} className={cx('sz-meter', mini && 'mini', clipped && 'clipped', className)} style={vars} aria-hidden="true">
      <Bricks count={count} />
    </div>
  )
})
