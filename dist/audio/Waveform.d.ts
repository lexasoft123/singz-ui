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
}
/**
 * Two stacked copies of the same waveform: a dim base layer and a bright
 * "played" layer clipped by the shared `--p` CSS variable, which the host's
 * playhead loop writes once per frame. Progress therefore costs no canvas
 * redraws at all — see audio.css for that contract.
 */
export declare function Waveform({ peaks, buffer, scale, color, viewStart, viewEnd }: WaveformProps): React.JSX.Element;
