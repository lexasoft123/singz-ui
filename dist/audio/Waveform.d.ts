export interface WaveformProps {
    /** Precomputed envelope, one value per bucket over the whole buffer. */
    peaks: Float32Array;
    /** Needed for the deep-zoom path, which reads raw samples. */
    buffer: AudioBuffer;
    scale: number;
    color: string;
    /** Visible window as fractions of the whole buffer. */
    viewStart: number;
    viewEnd: number;
    /**
     * One colour per `peaks` bucket — the stem-hued seek bar pattern the phone
     * shipped: each bucket in its loudest lane's hue, so the bar says who is
     * leading. Overview path only; the deep-zoom raw-sample path keeps the
     * single `color` (raw samples carry no per-bucket hue). Missing entries
     * fall back to `color`.
     */
    bucketColors?: readonly string[];
}
/**
 * Two stacked copies of the same waveform: a dim base layer and a bright
 * "played" layer clipped by the shared `--p` CSS variable, which the host's
 * playhead loop writes once per frame. Progress therefore costs no canvas
 * redraws at all — see audio.css for that contract.
 */
export declare function Waveform({ peaks, buffer, scale, color, viewStart, viewEnd, bucketColors }: WaveformProps): React.JSX.Element;
