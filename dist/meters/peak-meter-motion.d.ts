export declare const METER_FLOOR_DB = -60;
export declare const PEAK_HOLD_MS = 1100;
export interface PeakMeterMotion {
    liveDb: number;
    peakDb: number;
    holdMs: number;
    trail: number[];
}
export declare function meterIndex(db: number, count: number): number;
/** Fractional fill preserves real movement inside a single dB segment. */
export declare function meterBrickFill(db: number, index: number, count: number): number;
/** One level palette for the live bricks, their afterglow and the held peak. */
export declare function meterZone(index: number, count: number): number;
export declare function emptyPeakMeter(count: number): PeakMeterMotion;
/** Pure ballistics shared by the UI-thread renderer and regression tests. */
export declare function stepPeakMeter(previous: PeakMeterMotion, inputDb: number, elapsedMs: number, peakInputDb?: number, attackMs?: number): PeakMeterMotion;
export declare function peakMeterAtRest(state: PeakMeterMotion): boolean;
