import { forwardRef, type ButtonHTMLAttributes, type ReactNode } from 'react'
import { cx } from '../util/cx.js'

/*
 * forwardRef, not a bare ref prop: React 19 would allow the latter, but the
 * kit supports React 18.3 too (ChordZ and guitar_helper are both on 18) and
 * there ref-as-prop silently does nothing.
 */

export type ButtonVariant = 'ghost' | 'primary' | 'danger'

export interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: ButtonVariant
  size?: 'md' | 'sm'
  /** Icon-only, tightened padding — `.pill.gear`. */
  icon?: boolean
  /**
   * The action is currently engaged — a panel that is open, a mode that is
   * running. Paints `.pill.on` and sets aria-pressed, which makes the pill a
   * toggle to a screen reader as well as to the eye.
   *
   * Omitted, not defaulted to false: a one-shot button that announced
   * itself as an unpressed toggle would be worse than saying nothing.
   */
  active?: boolean
  /** Appended AFTER the kit's classes, so hosts keep their own hooks:
   *  `mic-toggle`, `update-chip`, `catalog-btn` are all E2E selectors. */
  className?: string
  children?: ReactNode
}

/**
 * The pill button. Emits exactly `pill ghost small` etc. — the class names
 * are the contract, not an implementation detail, because the host app's
 * end-to-end tests select on them.
 *
 * `type` defaults to "button": every call site in SingZ passed it explicitly,
 * and a stray submit inside a form is a bug nobody enjoys finding.
 */
export const Button = forwardRef<HTMLButtonElement, ButtonProps>(function Button(
  { variant = 'ghost', size = 'md', icon = false, active, className, type = 'button', ...rest },
  ref
) {
  return (
    <button
      ref={ref}
      type={type}
      aria-pressed={active}
      className={cx(
        'pill',
        variant,
        size === 'sm' && 'small',
        icon && 'gear',
        active && 'on',
        className
      )}
      {...rest}
    />
  )
})
