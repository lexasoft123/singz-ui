import { jsx as _jsx, jsxs as _jsxs, Fragment as _Fragment } from "react/jsx-runtime";
import { forwardRef, memo, useCallback, useEffect, useImperativeHandle, useRef } from 'react';
import { paintMeterCanvas } from './paint-canvas.js';
import { cx } from '../util/cx.js';
import { emptyPeakMeter, meterBrickFill, meterIndex, meterZone, peakMeterAtRest, stepPeakMeter } from './peak-meter-motion.js';
const MIN_STEP_MS = 4;
const Bricks = memo(function Bricks({ count }) {
    return (_jsxs(_Fragment, { children: [Array.from({ length: count }, (_, i) => (_jsxs("i", { className: `sz-meter-brick z${meterZone(i, count)}`, children: [_jsx("b", { className: "sz-meter-trail" }), _jsx("b", { className: "sz-meter-live" })] }, i))), _jsx("i", { className: "sz-meter-peak z0" })] }));
});
/**
 * The SingZ level meter. Ballistics (`stepPeakMeter`) run in one rAF loop that
 * writes straight to the DOM; React renders the bricks once. The loop stops
 * when the meter is at rest, when the document is hidden and when `enabled`
 * is false, and prefers-reduced-motion keeps the held peak but drops the
 * falling and trailing motion.
 */
export const PeakMeter = forwardRef(function PeakMeter({ db, peakDb = db, mini = false, clipped = false, identity, enabled = true, size = 1, renderMode = 'dom', maxFps = 0, paintOnReceive = false, attackMs = 20, onPaint, className, style }, ref) {
    const count = mini ? 16 : 40;
    const root = useRef(null);
    const canvas = useRef(null);
    const bitmap = useRef({ width: 0, height: 0, ratio: 1, gap: 2, key: '', palette: null });
    const sim = useRef({
        count,
        motion: emptyPeakMeter(count),
        want: -Infinity,
        wantPeak: -Infinity,
        target: -Infinity,
        peakTarget: -Infinity,
        enabled,
        maxFps,
        attackMs,
        paintOnReceive,
        onPaint,
        readingId: undefined,
        liveShape: '',
        nextPaint: 0,
        lastPixelPaint: -Infinity,
        reduced: false,
        hidden: false,
        raf: 0,
        last: 0,
        acc: 0,
        painted: { trail: [], live: [], peakIndex: -2, peakZone: -1 }
    });
    const paint = useCallback((frameTime) => {
        const el = root.current;
        if (!el)
            return;
        const s = sim.current;
        const { motion, count: n, painted } = s;
        const surface = canvas.current;
        if (surface) {
            const b = bitmap.current, ctx = surface.getContext('2d'), previousKey = bitmap.current.key;
            if (ctx && b.width > 0 && b.height > 0 && b.palette)
                b.key = paintMeterCanvas(ctx, motion, n, b.width, b.height, b.gap, b.ratio, b.palette, surface.dataset.clipped === 'true', b.key, s.onPaint ? shape => {
                    if (shape === s.liveShape)
                        return;
                    s.liveShape = shape;
                    if (s.readingId !== undefined && s.enabled && !s.hidden) {
                        s.onPaint?.(s.readingId, frameTime ?? performance.now());
                        s.readingId = undefined;
                    }
                } : undefined);
            if (b.key !== previousKey)
                s.lastPixelPaint = frameTime ?? performance.now();
            return;
        }
        let changed = false;
        const bricks = el.children;
        for (let i = 0; i < n; i++) {
            const brick = bricks[i];
            if (!brick)
                break;
            const fill = meterBrickFill(motion.liveDb, i, n);
            const trail = motion.trail[i] ?? 0;
            if (painted.live[i] !== fill) {
                changed = true;
                painted.live[i] = fill;
                brick.lastElementChild.style.transform = `scaleX(${fill})`;
            }
            if (painted.trail[i] !== trail) {
                changed = true;
                painted.trail[i] = trail;
                brick.firstElementChild.style.opacity = String(trail);
            }
        }
        const marker = bricks[n];
        if (!marker)
            return;
        const index = Math.max(0, meterIndex(motion.peakDb, n));
        const shown = motion.peakDb > -60;
        const key = shown ? index : -1;
        if (painted.peakIndex !== key) {
            changed = true;
            painted.peakIndex = key;
            marker.style.opacity = shown ? '1' : '0';
            marker.style.transform = `translateX(calc(${index} * (100% + var(--sz-meter-gap))))`;
            const zone = meterZone(index, n);
            if (painted.peakZone !== zone) {
                painted.peakZone = zone;
                marker.className = `sz-meter-peak z${zone}`;
            }
        }
        if (changed)
            s.lastPixelPaint = frameTime ?? performance.now();
    }, []);
    const reset = useCallback(() => {
        const s = sim.current;
        s.motion = emptyPeakMeter(s.count);
        paint();
    }, [paint]);
    const stop = useCallback(() => {
        const s = sim.current;
        if (s.raf)
            cancelAnimationFrame(s.raf);
        s.raf = 0;
    }, []);
    const frame = useCallback((now) => {
        const s = sim.current;
        s.raf = 0;
        const elapsed = s.last > 0 ? Math.min(100, Math.max(0, now - s.last)) : (s.attackMs === 0 ? MIN_STEP_MS : 0);
        s.last = now;
        s.acc += elapsed;
        if (s.acc >= MIN_STEP_MS && now + .5 >= s.nextPaint) {
            s.motion = stepPeakMeter(s.motion, s.target, s.acc, s.peakTarget, s.attackMs);
            s.acc = 0;
            const period = s.maxFps > 0 ? 1000 / s.maxFps : 0;
            s.nextPaint = period ? Math.max(s.nextPaint + period, now) : now;
            paint(now);
            if (peakMeterAtRest(s.motion))
                return;
        }
        s.raf = requestAnimationFrame(frame);
    }, [paint]);
    /** Re-evaluate what the meter should be doing after any input changed. */
    const sync = useCallback(() => {
        const s = sim.current;
        const live = s.enabled && !s.hidden;
        s.target = live ? s.want : -Infinity;
        s.peakTarget = live ? s.wantPeak : -Infinity;
        if (!live) {
            stop();
            reset();
        }
        else if (s.reduced) {
            // Hold the peak, omit falling and trailing.
            stop();
            const snapshot = stepPeakMeter(s.motion, s.target, 100, s.peakTarget, s.attackMs);
            snapshot.liveDb = Number.isFinite(s.target) ? Math.max(-60, Math.min(0, s.target)) : -60;
            snapshot.trail.fill(0);
            if (snapshot.liveDb <= -60)
                snapshot.peakDb = -60;
            s.motion = snapshot;
            paint();
        }
        else if (!s.raf) {
            // Nothing to animate: an empty meter fed silence stays dark and idle.
            if (peakMeterAtRest(s.motion) && !(s.target > -60) && !(s.peakTarget > -60))
                return;
            s.last = 0;
            s.acc = 0;
            s.nextPaint = s.paintOnReceive && s.maxFps > 0 ? Math.max(0, s.lastPixelPaint + 1000 / s.maxFps) : 0;
            s.raf = requestAnimationFrame(frame);
        }
    }, [frame, paint, reset, stop]);
    const push = useCallback((nextDb, nextPeak = nextDb, readingId) => {
        const s = sim.current;
        s.readingId = readingId;
        s.want = nextDb;
        s.wantPeak = nextPeak;
        sync();
        const now = performance.now();
        // A reply often lands immediately after rAF. Draw before the compositor's
        // next presentation if the cap permits; retain the existing ballistic step.
        if (s.paintOnReceive && s.enabled && !s.hidden && !document.body.classList.contains('modal-open') && !s.reduced && now + .5 >= s.lastPixelPaint + (s.maxFps > 0 ? 1000 / s.maxFps : 0)) {
            stop();
            s.nextPaint = now;
            s.acc = Math.max(s.acc, MIN_STEP_MS);
            frame(now);
        }
    }, [sync, stop, frame]);
    useImperativeHandle(ref, () => ({ push }), [push]);
    // Size and palette are measured only on layout/theme changes, never per frame.
    useEffect(() => {
        const el = root.current, surface = canvas.current;
        if (!el || !surface)
            return;
        const resize = () => {
            const rect = el.getBoundingClientRect(), css = getComputedStyle(el);
            const b = bitmap.current, dpr = window.devicePixelRatio || 1;
            b.width = Math.max(0, Math.round(rect.width * dpr));
            b.height = Math.max(0, Math.round(rect.height * dpr));
            b.ratio = rect.width / (el.clientWidth || rect.width || 1) * dpr;
            b.gap = (parseFloat(css.getPropertyValue('--sz-meter-gap')) || 2) * b.ratio;
            b.palette = { zones: ['--sz-success', '--sz-meter-warn', '--sz-danger'].map(key => css.getPropertyValue(key).trim()),
                faint: css.getPropertyValue('--sz-faint').trim(), danger: css.getPropertyValue('--sz-danger').trim(), text: css.getPropertyValue('--sz-text').trim() };
            if (surface.width !== b.width)
                surface.width = b.width;
            if (surface.height !== b.height)
                surface.height = b.height;
            b.key = '';
            paint();
        };
        const observer = typeof ResizeObserver === 'function' ? new ResizeObserver(resize) : null;
        const theme = new MutationObserver(resize);
        observer?.observe(el);
        theme.observe(document.body, { attributes: true, attributeFilter: ['class', 'style'] });
        window.addEventListener('resize', resize);
        resize();
        return () => { observer?.disconnect(); theme.disconnect(); window.removeEventListener('resize', resize); };
    }, [renderMode, count, size, paint]);
    useEffect(() => { bitmap.current.key = ''; paint(); }, [clipped, paint]);
    // Environment: document visibility and reduced motion.
    useEffect(() => {
        const s = sim.current;
        s.hidden = document.hidden;
        const motionQuery = typeof matchMedia === 'function' ? matchMedia('(prefers-reduced-motion: reduce)') : null;
        s.reduced = motionQuery?.matches ?? false;
        const onVisibility = () => { s.hidden = document.hidden; sync(); };
        const onMotion = () => { s.reduced = motionQuery?.matches ?? false; sync(); };
        document.addEventListener('visibilitychange', onVisibility);
        motionQuery?.addEventListener?.('change', onMotion);
        sync();
        return () => {
            document.removeEventListener('visibilitychange', onVisibility);
            motionQuery?.removeEventListener?.('change', onMotion);
            stop();
        };
    }, [stop, sync]);
    // A new song / size starts from an empty meter.
    useEffect(() => {
        const s = sim.current;
        s.count = count;
        s.painted = { trail: [], live: [], peakIndex: -2, peakZone: -1 };
        reset();
        sync();
    }, [identity, count, renderMode, reset, sync]);
    // New values only touch refs; the rAF loop does the rest.
    useEffect(() => {
        const s = sim.current;
        s.enabled = enabled;
        s.attackMs = Number.isFinite(attackMs) && attackMs >= 0 ? attackMs : 20;
        s.paintOnReceive = paintOnReceive;
        s.onPaint = onPaint;
        s.maxFps = Number.isFinite(maxFps) && maxFps > 0 ? maxFps : 0;
        s.want = db;
        s.wantPeak = peakDb;
        sync();
    }, [db, peakDb, enabled, maxFps, paintOnReceive, attackMs, onPaint, sync]);
    const vars = { '--sz-meter-count': count, '--sz-meter-scale': size, ...style };
    return (_jsx("div", { ref: root, className: cx('sz-meter', mini && 'mini', clipped && 'clipped', className), style: vars, "aria-hidden": "true", children: renderMode === 'canvas' ? _jsx("canvas", { ref: canvas, className: "sz-meter-canvas", "data-clipped": String(clipped), style: { width: '100%', height: '100%', display: 'block' } }) : _jsx(Bricks, { count: count }) }));
});
