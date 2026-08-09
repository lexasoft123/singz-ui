import type { ReactNode } from 'react'
import { Chip } from '@singz/ui'

/*
 * The mute/solo chips from a SingZ mixer lane. `active` is the whole API for a
 * toggle — Chip sets aria-pressed from it, so hosts never wire that by hand.
 */

const Surface = ({ children }: { children: ReactNode }): React.JSX.Element => (
  <div
    style={{
      background: 'var(--sz-bg)',
      color: 'var(--sz-text)',
      fontFamily: 'var(--sz-font-display)',
      padding: 20,
      borderRadius: 10,
      display: 'flex',
      gap: 10,
      alignItems: 'center',
      flexWrap: 'wrap'
    }}
  >
    {children}
  </div>
)

export const Resting = (): React.JSX.Element => (
  <Surface>
    <Chip aria-label="Mute">M</Chip>
    <Chip aria-label="Solo">S</Chip>
  </Surface>
)

/** The kit's own pressed treatment: accent tint, accent border, accent text. */
export const Active = (): React.JSX.Element => (
  <Surface>
    <Chip active aria-label="Mute">
      M
    </Chip>
    <Chip aria-label="Solo">S</Chip>
  </Surface>
)

/**
 * A host may colour the pressed state per row instead — SingZ fills it with
 * the lane's own `--stem`. Shown here with inline styles because that rule
 * belongs to the host, not the kit.
 */
export const HostColoured = (): React.JSX.Element => (
  <Surface>
    {[
      ['Vocals', '#ff5c65'],
      ['Drums', '#ffc53d'],
      ['Bass', '#527dff'],
      ['Instruments', '#27e7bb']
    ].map(([label, stem]) => (
      <span key={label} style={{ display: 'inline-flex', gap: 6, alignItems: 'center' }}>
        <Chip
          active
          aria-label={`Mute ${label}`}
          style={{ background: stem, borderColor: stem, color: '#241705' }}
        >
          M
        </Chip>
        <span style={{ fontSize: 12, color: 'var(--sz-dim)' }}>{label}</span>
      </span>
    ))}
  </Surface>
)

/**
 * `disabled` blocks the click and sets the attribute, but the kit ships NO
 * dimming for it — a disabled chip looks exactly like a resting one. Dim it
 * yourself until the kit grows a shared disabled treatment.
 */
export const Disabled = (): React.JSX.Element => (
  <Surface>
    <span style={{ display: 'inline-flex', gap: 6, alignItems: 'center' }}>
      <Chip disabled aria-label="Mute">
        M
      </Chip>
      <span style={{ fontSize: 12, color: 'var(--sz-dim)' }}>as the kit renders it</span>
    </span>
    <span style={{ display: 'inline-flex', gap: 6, alignItems: 'center' }}>
      <Chip disabled aria-label="Mute" style={{ opacity: 0.4, cursor: 'default' }}>
        M
      </Chip>
      <span style={{ fontSize: 12, color: 'var(--sz-dim)' }}>with the host&rsquo;s dimming</span>
    </span>
  </Surface>
)
