import { forwardRef, type ButtonHTMLAttributes, type ReactNode } from 'react'
import { cx } from '../util/cx.js'

export interface ChipProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  /** Pressed state. Also sets aria-pressed — SingZ had this on 5 of its
   *  toggles and missing on the rest; centralising it fixes that by
   *  construction rather than by remembering. */
  active?: boolean
  className?: string
  children?: ReactNode
}

/** A small square toggle — the M / S buttons on a mixer lane, and friends. */
export const Chip = forwardRef<HTMLButtonElement, ChipProps>(function Chip(
  { active = false, className, type = 'button', ...rest },
  ref
) {
  return (
    <button
      ref={ref}
      type={type}
      aria-pressed={active}
      className={cx('chip', className, active && 'active')}
      {...rest}
    />
  )
})
