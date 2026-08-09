import { cx } from '../util/cx.js'

export type DotTone = 'ok' | 'idle' | 'warn'

export interface StatusDotProps {
  tone: DotTone
  className?: string
}

/** A 7px state dot. `warn` pulses — and the host must pause that animation
 *  while a modal covers the app, or the scrim's backdrop blur re-rasters the
 *  whole window on every frame. SingZ does this via `body.modal-open`. */
export function StatusDot({ tone, className }: StatusDotProps): React.JSX.Element {
  return <span className={cx('dot', tone, className)} />
}

export interface LinkButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  className?: string
}

/** `.linkish` — a text button that reads as a link but is a real button. */
export function LinkButton({ className, type = 'button', ...rest }: LinkButtonProps): React.JSX.Element {
  return <button type={type} className={cx('linkish', className)} {...rest} />
}

export interface BadgeProps {
  children?: React.ReactNode
  className?: string
}

/** `.badge` — a small uppercase outline tag. */
export function Badge({ children, className }: BadgeProps): React.JSX.Element {
  return <span className={cx('badge', className)}>{children}</span>
}
