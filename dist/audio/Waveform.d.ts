export interface WaveformProps {
    /** Precomputed envelope, one value per bucket over the whole buffer. */
    peaks: Float32Array;
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
    buffer: AudioBuffer | null;
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
 * Two stacked copies of the same waveform: a resting base layer and a bright
 * "played" layer clipped by the shared `--p` CSS variable, which the host's
 * playhead loop writes. Progress therefore costs no canvas redraws — but a
 * re-clip still damages the layer's whole visible part, so a host should move
 * `--p` on a clock rather than every frame; see audio.css for that contract.
 */
export declare function Waveform({ peaks, buffer, scale, color, viewStart, viewEnd, bucketColors }: WaveformProps): React.JSX.Element;
