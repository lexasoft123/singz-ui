import type { ReactNode } from 'react'
import { StatusDot } from '@singz/ui'

/* The splitter-status dot from SingZ's titlebar. `warn` pulses — the host is
   expected to pause that animation behind a modal (see the kit README). */

const Surface = ({ children }: { children: ReactNode }): React.JSX.Element => (
  <div
    style={{
      background: 'var(--sz-bg)',
      color: 'var(--sz-text)',
      fontFamily: 'var(--sz-font-display)',
      padding: 20,
      borderRadius: 10,
      display: 'flex',
      gap: 22,
      alignItems: 'center',
      flexWrap: 'wrap'
    }}
  >
    {children}
  </div>
)

const Row = ({ tone, label }: { tone: 'ok' | 'idle' | 'warn'; label: string }): React.JSX.Element => (
  <span style={{ display: 'inline-flex', gap: 8, alignItems: 'center', fontSize: 13 }}>
    <StatusDot tone={tone} />
    {label}
  </span>
)

export const Tones = (): React.JSX.Element => (
  <Surface>
    <Row tone="ok" label="splitter ready" />
    <Row tone="idle" label="checking splitter…" />
    <Row tone="warn" label="needs setup" />
  </Surface>
)

/** How it actually appears: inside a status chip in the titlebar. */
export const InAChip = (): React.JSX.Element => (
  <Surface>
    <span className="chip-status" style={{ display: 'inline-flex', gap: 8, alignItems: 'center' }}>
      <StatusDot tone="ok" />
      splitter ready
    </span>
  </Surface>
)
