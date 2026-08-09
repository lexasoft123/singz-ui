import type { ReactNode } from 'react'
import { SegmentedControl } from '@singz/ui'

/* The card scaffold paints itself white; this is a dark DS. Surface applies
   the DS's own ground tokens so a theme override still flows through. */
const Surface = ({ children }: { children: ReactNode }): React.JSX.Element => (
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


/* Both of these are real controls from SingZ's metronome popover — the grid
   toggle and the beats-per-bar picker. `value` is fixed rather than stateful
   so the card renders the same way every time. */

export const TwoWay = (): React.JSX.Element => (
  <Surface><SegmentedControl
    aria-label="Grid view"
    value="off"
    onChange={() => {}}
    options={[
      { value: 'off', label: 'Off' },
      { value: 'show', label: 'Show' }
    ]}
  /></Surface>
)

export const ManyOptions = (): React.JSX.Element => (
  <Surface><SegmentedControl
    aria-label="Beats per bar"
    value={4}
    onChange={() => {}}
    options={[
      { value: 2, label: '2' },
      { value: 3, label: '3' },
      { value: 4, label: '4' },
      { value: 6, label: '6' }
    ]}
  /></Surface>
)

/** A single option can be disabled without disabling the group. */
export const WithDisabledOption = (): React.JSX.Element => (
  <Surface><SegmentedControl
    aria-label="Count-in"
    value="off"
    onChange={() => {}}
    options={[
      { value: 'off', label: 'Off' },
      { value: '1', label: '1 bar' },
      { value: '2', label: '2 bars', disabled: true, title: 'Needs a beat grid' }
    ]}
  /></Surface>
)

/** The whole group greys out when the song has no beat grid to follow. */
export const Disabled = (): React.JSX.Element => (
  <Surface><SegmentedControl
    aria-label="Accent"
    value="one"
    onChange={() => {}}
    disabled
    options={[
      { value: 'one', label: 'On the 1' },
      { value: 'off', label: 'Off' }
    ]}
  /></Surface>
)
