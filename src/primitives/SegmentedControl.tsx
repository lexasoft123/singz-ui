import type { ReactNode } from 'react'
import { cx } from '../util/cx.js'

export interface SegmentOption<T extends string | number> {
  value: T
  label: ReactNode
  disabled?: boolean
  title?: string
}

export interface SegmentedControlProps<T extends string | number> {
  options: SegmentOption<T>[]
  value: T
  onChange: (value: T) => void
  disabled?: boolean
  className?: string
  'aria-label'?: string
}

/**
 * `.mode-seg` — a row of buttons where one is `.on`.
 *
 * The buttons are DIRECT children of `.mode-seg` and nothing may come between
 * them: the Windows smoke test selects `.mode-seg button:nth-child(1|2)` and
 * reads `className.includes('on')`. A wrapper element per option would look
 * harmless and break it.
 */
export function SegmentedControl<T extends string | number>({
  options,
  value,
  onChange,
  disabled = false,
  className,
  'aria-label': ariaLabel
}: SegmentedControlProps<T>): React.JSX.Element {
  return (
    <div className={cx('mode-seg', className)} role="group" aria-label={ariaLabel}>
      {options.map((o) => (
        <button
          key={String(o.value)}
          type="button"
          title={o.title}
          disabled={disabled || o.disabled}
          aria-pressed={o.value === value}
          className={o.value === value ? 'on' : ''}
          onClick={() => onChange(o.value)}
        >
          {o.label}
        </button>
      ))}
    </div>
  )
}
