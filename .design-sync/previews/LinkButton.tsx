import type { ReactNode } from 'react'
import { LinkButton } from '@singz/ui'

/* Reads as a link, is a real <button>. These are the lyric-panel actions from
   SingZ — the row under a matched lyrics source. */

const Surface = ({ children }: { children: ReactNode }): React.JSX.Element => (
  <div
    style={{
      background: 'var(--sz-bg)',
      color: 'var(--sz-text)',
      fontFamily: 'var(--sz-font-display)',
      padding: 20,
      borderRadius: 10,
      display: 'flex',
      gap: 14,
      alignItems: 'center',
      flexWrap: 'wrap'
    }}
  >
    {children}
  </div>
)

export const Actions = (): React.JSX.Element => (
  <Surface>
    <LinkButton>Refine timing</LinkButton>
    <LinkButton>Change…</LinkButton>
    <LinkButton>Precise</LinkButton>
  </Surface>
)

/** In context: a source credit line with its actions trailing. */
export const InASourceLine = (): React.JSX.Element => (
  <Surface>
    <span style={{ display: 'inline-flex', gap: 10, alignItems: 'center', fontSize: 12 }}>
      <span className="badge">synced</span>
      <span style={{ color: 'var(--sz-dim)' }}>Rammstein — Puppe</span>
      <LinkButton>Refine timing</LinkButton>
      <LinkButton>Change…</LinkButton>
    </span>
  </Surface>
)

/**
 * As with Chip, `disabled` is functional only — `.linkish` carries no dimming,
 * so a dead action still reads as live accent text. SingZ dims its own at
 * `.src-link:disabled { opacity: .4; cursor: default }`; do the same.
 */
export const Disabled = (): React.JSX.Element => (
  <Surface>
    <LinkButton disabled>Refine timing (kit as-is)</LinkButton>
    <LinkButton disabled style={{ opacity: 0.4, cursor: 'default' }}>
      Refine timing (host-dimmed)
    </LinkButton>
  </Surface>
)
