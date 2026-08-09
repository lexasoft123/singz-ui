import type { ReactNode } from 'react'
import { Badge } from '@singz/ui'

/* Small uppercase outline tags. In SingZ these mark what a project contains
   and where its lyrics came from. */

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

export const Tags = (): React.JSX.Element => (
  <Surface>
    <Badge>stems</Badge>
    <Badge>lyrics</Badge>
    <Badge>synced</Badge>
  </Surface>
)

/** On a project row, which is where they are actually read. */
export const OnAProjectRow = (): React.JSX.Element => (
  <Surface>
    <span
      style={{
        display: 'inline-flex',
        gap: 12,
        alignItems: 'center',
        width: '100%',
        justifyContent: 'space-between'
      }}
    >
      <span style={{ fontWeight: 600 }}>Sixteen Tons</span>
      <span style={{ display: 'inline-flex', gap: 8, alignItems: 'center' }}>
        <Badge>stems</Badge>
        <Badge>lyrics</Badge>
        <span style={{ fontSize: 12, color: 'var(--sz-faint)' }}>9 Aug</span>
      </span>
    </span>
  </Surface>
)
