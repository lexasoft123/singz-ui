import { Waveform } from '@singz/ui'

/*
 * Waveform draws two stacked canvases from a peaks envelope, and reads two
 * variables the HOST owns, not the kit:
 *
 *   --stem  the lane's colour
 *   --p     playback progress as a %, written by the host's rAF loop — the
 *           bright layer is clipped to it, which is why progress costs no
 *           redraw
 *
 * `.wave` is absolutely positioned, so it needs a sized, positioned parent.
 * Both of those are the real integration contract, so the preview shows them
 * rather than hiding them.
 */

/** A song-shaped envelope: verses quieter, choruses louder, silence at the top. */
function makePeaks(n = 900, seed = 7): Float32Array {
  const out = new Float32Array(n)
  let s = seed
  const rnd = (): number => ((s = (s * 1103515245 + 12345) & 0x7fffffff) / 0x7fffffff)
  for (let i = 0; i < n; i++) {
    const t = i / n
    if (t < 0.04) { out[i] = 0.02 * rnd(); continue }        // count-in silence
    const section = 0.45 + 0.4 * Math.abs(Math.sin(t * Math.PI * 2.2))
    const beat = 0.75 + 0.25 * Math.abs(Math.sin(t * Math.PI * 96))
    out[i] = Math.min(1, section * beat * (0.75 + 0.35 * rnd()))
  }
  return out
}

/** Real AudioBuffer — the deep-zoom path reads raw samples from it. */
function makeBuffer(): AudioBuffer {
  const rate = 8000
  const ctx = new OfflineAudioContext(1, rate, rate)
  const buf = ctx.createBuffer(1, rate, rate)
  const ch = buf.getChannelData(0)
  for (let i = 0; i < ch.length; i++) {
    const t = i / rate
    ch[i] = Math.sin(t * 2 * Math.PI * 110) * 0.6 * (0.5 + 0.5 * Math.sin(t * 2 * Math.PI * 3))
  }
  return buf
}

const PEAKS = makePeaks()
const BUFFER = makeBuffer()

const Lane = ({
  color,
  label,
  progress = '38%',
  viewStart = 0,
  viewEnd = 1
}: {
  color: string
  label: string
  progress?: string
  viewStart?: number
  viewEnd?: number
}): React.JSX.Element => (
  <div style={{ marginBottom: 10 }}>
    <div style={{ fontSize: 11, color: 'var(--sz-dim)', marginBottom: 4 }}>{label}</div>
    <div
      style={{
        position: 'relative',
        height: 64,
        ['--stem' as string]: color,
        ['--p' as string]: progress
      }}
    >
      <Waveform
        peaks={PEAKS}
        buffer={BUFFER}
        scale={1}
        color={color}
        viewStart={viewStart}
        viewEnd={viewEnd}
      />
    </div>
  </div>
)

const Surface = ({ children }: { children: React.ReactNode }): React.JSX.Element => (
  <div
    style={{
      background: 'var(--sz-bg)',
      color: 'var(--sz-text)',
      fontFamily: 'var(--sz-font-display)',
      padding: 20,
      borderRadius: 10
    }}
  >
    {children}
  </div>
)

/** One lane per stem, each in its own colour — a SingZ track stack. */
export const StemLanes = (): React.JSX.Element => (
  <Surface>
    <Lane color="#ff5c65" label="Vocals" />
    <Lane color="#ffc53d" label="Drums" />
    <Lane color="#527dff" label="Bass" />
    <Lane color="#27e7bb" label="Instruments" />
  </Surface>
)

/** `--p` clips the bright layer: everything left of it reads as played. */
export const PlayedVsUnplayed = (): React.JSX.Element => (
  <Surface>
    <Lane color="#ffc53d" label="0% — nothing played yet" progress="0%" />
    <Lane color="#ffc53d" label="45% — mid-song" progress="45%" />
    <Lane color="#ffc53d" label="100% — finished" progress="100%" />
  </Surface>
)

/** Zoomed past the raw-sample threshold, the envelope gives way to real samples. */
export const DeepZoom = (): React.JSX.Element => (
  <Surface>
    <Lane color="#527dff" label="Whole song" viewStart={0} viewEnd={1} />
    <Lane color="#527dff" label="Zoomed to 2% of the song" viewStart={0.4} viewEnd={0.42} />
  </Surface>
)
