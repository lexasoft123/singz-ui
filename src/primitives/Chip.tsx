import { forwardRef, type ButtonHTMLAttributes, type ReactNode } from 'react'
import { cx } from '../util/cx.js'

export interface ChipProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  /** Pressed state. Also sets aria-pressed — SingZ had this on 5 of its
   *  toggles and missing on the rest; centralising it fixes that by
   *  construction rather than by remembering.
   *
   *  Leave it OFF for a chip that is not a toggle. It used to default to
   *  false, which meant a one-shot chip — ChordZ's transpose and capo
   *  steppers — was announced as an unpressed toggle, so those had to be
   *  written as raw <button className="chip"> to stay honest. */
  active?: boolean
  /** Room for a word rather than a letter — `.chip.wide`. */
  wide?: boolean
  className?: string
  children?: ReactNode
}

/** A small square toggle — the M / S buttons on a mixer lane, and friends. */
export const Chip = forwardRef<HTMLButtonElement, ChipProps>(function Chip(
  { active, wide = false, className, type = 'button', ...rest },
  ref
) {
  return (
    <button
      ref={ref}
      type={type}
      aria-pressed={active}
      className={cx('chip', className, wide && 'wide', active && 'active')}
      {...rest}
    />
  )
})
