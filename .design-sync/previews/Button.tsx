import type { ReactNode } from 'react'
import { Button } from '@singz/ui'

/*
 * Compositions taken from SingZ's own titlebar and transport, so the labels
 * are the ones a singer actually sees rather than "Button 1".
 *
 * Surface exists because the preview card's scaffold paints itself white and
 * this is a dark design system — ghost buttons are near-white text and would
 * be invisible on it. It uses the DS's own tokens, so a theme override
 * (ChordZ's light palette) flows straight through instead of being pinned.
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
      gap: 12,
      alignItems: 'center',
      flexWrap: 'wrap'
    }}
  >
    {children}
  </div>
)

export const Variants = (): React.JSX.Element => (
  <Surface>
    <Button variant="ghost">Open…</Button>
    <Button variant="primary">Split into stems</Button>
    <Button variant="ghost" className="danger">
      Delete
    </Button>
  </Surface>
)

export const Sizes = (): React.JSX.Element => (
  <Surface>
    <Button variant="ghost">Save project</Button>
    <Button variant="ghost" size="sm">
      Log
    </Button>
  </Surface>
)

/** Icon-only: tighter padding, and the svg is laid out as a block. */
export const IconOnly = (): React.JSX.Element => (
  <Surface>
    <Button variant="ghost" size="sm" icon aria-label="Settings">
      <svg width="13" height="13" viewBox="0 0 16 16" fill="currentColor" aria-hidden>
        <path d="M9.4 1.05c-.41-1.4-2.4-1.4-2.81 0l-.1.34a1.46 1.46 0 0 1-2.1.87l-.31-.17c-1.28-.7-2.69.71-1.99 1.99l.17.31a1.46 1.46 0 0 1-.87 2.1l-.34.1c-1.4.42-1.4 2.4 0 2.82l.34.1a1.46 1.46 0 0 1 .87 2.1l-.17.31c-.7 1.29.71 2.69 1.99 1.99l.31-.17a1.46 1.46 0 0 1 2.1.87l.1.34c.42 1.4 2.4 1.4 2.82 0l.1-.34a1.46 1.46 0 0 1 2.1-.87l.31.17c1.29.7 2.69-.7 1.99-1.99l-.17-.31a1.46 1.46 0 0 1 .87-2.1l.34-.1c1.4-.42 1.4-2.4 0-2.82l-.34-.1a1.46 1.46 0 0 1-.87-2.1l.17-.31c.7-1.28-.7-2.69-1.99-1.99l-.31.17a1.46 1.46 0 0 1-2.1-.87l-.1-.34zM8 10.93a2.93 2.93 0 1 1 0-5.86 2.93 2.93 0 0 1 0 5.86z" />
      </svg>
    </Button>
    <Button variant="ghost" size="sm">
      Stem files
    </Button>
  </Surface>
)

/**
 * `primary` is the ONLY variant with a disabled treatment of its own
 * (`opacity: .45`). A disabled `ghost` is pixel-identical to a live one — dim
 * it yourself until the kit grows a shared rule.
 */
export const Disabled = (): React.JSX.Element => (
  <Surface>
    <Button variant="primary" disabled>
      Splitting…
    </Button>
    <Button variant="ghost" disabled>
      Re-split (kit as-is)
    </Button>
    <Button variant="ghost" disabled style={{ opacity: 0.45, cursor: 'default' }}>
      Re-split (host-dimmed)
    </Button>
  </Surface>
)
