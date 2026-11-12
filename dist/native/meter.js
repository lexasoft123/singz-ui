import { jsx as _jsx, jsxs as _jsxs } from "react/jsx-runtime";
import React, { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { AppState, View } from 'react-native';
import { BlurMask, Canvas, Group, RoundedRect } from '@shopify/react-native-skia';
import { useDerivedValue, useFrameCallback, useReducedMotion, useSharedValue } from 'react-native-reanimated';
import { scheduleOnRN } from 'react-native-worklets';
import { tokens } from '../tokens/tokens.js';
import { useNativeTheme } from './theme.js';
import { emptyPeakMeter, meterBrickFill, meterIndex, meterZone, peakMeterAtRest, stepPeakMeter } from '../meters/peak-meter-motion.js';
function Brick({ index, count, size, height, motion, clipped, colors }) {
    const gap = 2;
    const brickWidth = useDerivedValue(() => Math.max(0, (size.value.width - gap * (count - 1)) / count));
    const x = useDerivedValue(() => index * (brickWidth.value + gap));
    const color = colors.zones[meterZone(index, count)];
    const trailOpacity = useDerivedValue(() => motion.value.trail[index]);
    const liveWidth = useDerivedValue(() => brickWidth.value * meterBrickFill(motion.value.liveDb, index, count));
    return _jsxs(Group, { children: [_jsx(RoundedRect, { x: x, y: 0, width: brickWidth, height: height, r: 1, color: clipped ? colors.danger : colors.faint, opacity: clipped ? 0.24 : 0.3 }), _jsx(RoundedRect, { x: x, y: 0, width: brickWidth, height: height, r: 1, color: color, opacity: trailOpacity }), _jsx(RoundedRect, { x: x, y: 0, width: liveWidth, height: height, r: 1, color: color })] });
}
/** A tiny Skia surface keeps per-frame ballistics off the JS tree. Needs the
 *  optional peers @shopify/react-native-skia, react-native-reanimated and
 *  react-native-worklets. */
export function PeakMeter({ db, peakDb = db, mini = false, clipped = false, identity, enabled = true, testID }) {
    const theme = useNativeTheme();
    const colors = useMemo(() => ({
        zones: [tokens.success, tokens['meter-warn'], theme.danger], faint: theme.faint, danger: theme.danger, text: theme.text, panelDeep: theme.panelDeep
    }), [theme]);
    const count = mini ? 16 : 40;
    const height = mini ? 9 : 16;
    const size = useSharedValue({ width: 0, height });
    const [foreground, setForeground] = useState(AppState.currentState === 'active');
    const reduced = useReducedMotion();
    const target = useSharedValue(-Infinity);
    const peakTarget = useSharedValue(-Infinity);
    const motion = useSharedValue(emptyPeakMeter(count));
    const lastTimestamp = useSharedValue(0);
    const accumulatedMs = useSharedValue(0);
    const epoch = useSharedValue(0);
    const pendingStop = useSharedValue(false);
    const latestEpoch = useRef(0);
    const frameRef = useRef(null);
    const stop = useCallback((finishedEpoch) => {
        if (finishedEpoch === latestEpoch.current)
            frameRef.current?.setActive(false);
    }, []);
    const frame = useFrameCallback((info) => {
        const now = info.timestamp;
        const elapsed = lastTimestamp.value > 0 ? Math.min(100, Math.max(0, now - lastTimestamp.value)) : 0;
        lastTimestamp.value = now;
        accumulatedMs.value += elapsed;
        // Coalesce callbacks faster than ProMotion's display cadence.
        if (accumulatedMs.value < 4)
            return;
        const next = stepPeakMeter(motion.value, target.value, accumulatedMs.value, peakTarget.value);
        accumulatedMs.value = 0;
        motion.value = next;
        if (peakMeterAtRest(next) && !pendingStop.value) {
            pendingStop.value = true;
            scheduleOnRN(stop, epoch.value);
        }
    }, false);
    frameRef.current = frame;
    useEffect(() => {
        const subscription = AppState.addEventListener('change', state => setForeground(state === 'active'));
        return () => subscription.remove();
    }, []);
    useEffect(() => { motion.value = emptyPeakMeter(count); }, [identity, count, motion]);
    useEffect(() => {
        latestEpoch.current += 1;
        epoch.value = latestEpoch.current;
        pendingStop.value = false;
        target.value = foreground && enabled ? db : -Infinity;
        peakTarget.value = foreground && enabled ? peakDb : -Infinity;
        if (!foreground || !enabled) {
            frame.setActive(false);
            motion.value = emptyPeakMeter(count);
        }
        else if (reduced) {
            // Reduced motion retains the held peak but omits falling/trailing motion.
            frame.setActive(false);
            const snapshot = stepPeakMeter(motion.value, db, 100, peakDb);
            snapshot.liveDb = Number.isFinite(db) ? Math.max(-60, Math.min(0, db)) : -60;
            snapshot.trail.fill(0);
            if (snapshot.liveDb <= -60)
                snapshot.peakDb = -60;
            motion.value = snapshot;
        }
        else {
            lastTimestamp.value = 0;
            accumulatedMs.value = 0;
            frame.setActive(true);
        }
        return () => frame.setActive(false);
    }, [db, peakDb, foreground, enabled, reduced, identity, count, frame, motion, target, peakTarget, epoch, pendingStop, lastTimestamp, accumulatedMs]);
    const gap = 2;
    const brickWidth = useDerivedValue(() => Math.max(0, (size.value.width - gap * (count - 1)) / count));
    const peakX = useDerivedValue(() => Math.max(0, meterIndex(motion.value.peakDb, count)) * (brickWidth.value + gap));
    const peakOpacity = useDerivedValue(() => motion.value.peakDb > -60 ? 1 : 0);
    const peakColor = useDerivedValue(() => colors.zones[meterZone(Math.max(0, meterIndex(motion.value.peakDb, count)), count)]);
    return _jsx(View, { testID: testID ?? (mini ? 'peak-meter-mini' : 'peak-meter-full'), style: { width: '100%', height, borderRadius: 3, overflow: 'hidden', backgroundColor: colors.panelDeep }, children: _jsxs(Canvas, { onSize: size, style: { width: '100%', height }, pointerEvents: "none", accessible: false, children: [Array.from({ length: count }, (_, index) => _jsx(Brick, { index: index, count: count, size: size, height: height, motion: motion, clipped: clipped, colors: colors }, index)), _jsxs(Group, { opacity: peakOpacity, children: [_jsx(RoundedRect, { x: peakX, y: 0, width: brickWidth, height: height, r: 1, color: peakColor, children: _jsx(BlurMask, { blur: mini ? 2 : 4, style: "outer" }) }), _jsx(RoundedRect, { x: peakX, y: 0, width: brickWidth, height: height, r: 1, color: peakColor }), _jsx(RoundedRect, { x: peakX, y: 0, width: brickWidth, height: mini ? 1 : 2, r: 1, color: colors.text, opacity: 0.45 })] })] }) });
}
