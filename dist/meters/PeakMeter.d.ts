export interface PeakMeterProps {
    /** Live level in dBFS. `-Infinity` is silence. */
    db: number;
    /** Peak level in dBFS; defaults to `db`. */
    peakDb?: number;
    /** 16 bricks / 9px instead of 40 bricks / 16px. */
    mini?: boolean;
    /** Optional bitmap renderer for software compositing; omits decorative glow. */
    renderMode?: 'dom' | 'canvas';
    /** Maximum simulation/paint cadence; ballistics retain their elapsed-time speed. */
    maxFps?: number;
    /** Tints the unlit bricks red. */
    clipped?: boolean;
    /** Changing it (a new song, a new input) clears the meter instantly. */
    identity?: object;
    /** `false` empties the meter and stops all work. Default true. */
    enabled?: boolean;
    /** Height multiplier for TV / desktop scale. Default 1. */
    size?: number;
    className?: string;
    style?: React.CSSProperties;
}
/** Drive the meter without any React render at all. */
export interface PeakMeterHandle {
    push(db: number, peakDb?: number): void;
}
/**
 * The SingZ level meter. Ballistics (`stepPeakMeter`) run in one rAF loop that
 * writes straight to the DOM; React renders the bricks once. The loop stops
 * when the meter is at rest, when the document is hidden and when `enabled`
 * is false, and prefers-reduced-motion keeps the held peak but drops the
 * falling and trailing motion.
 */
export declare const PeakMeter: import("react").ForwardRefExoticComponent<PeakMeterProps & import("react").RefAttributes<PeakMeterHandle>>;
