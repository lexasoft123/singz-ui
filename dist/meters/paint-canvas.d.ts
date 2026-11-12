import { type PeakMeterMotion } from './peak-meter-motion.js';
export interface MeterPalette {
    zones: readonly string[];
    faint: string;
    danger: string;
    text: string;
}
/** One bitmap, no per-brick layers. Quantize damage to bitmap pixels and alpha bytes. */
export declare function paintMeterCanvas(ctx: CanvasRenderingContext2D, motion: PeakMeterMotion, count: number, width: number, height: number, gap: number, radius: number, palette: MeterPalette, clipped: boolean, previous: string): string;
