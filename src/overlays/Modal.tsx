import { useCallback, useEffect, useRef, type ReactNode } from 'react'
import { cx } from '../util/cx.js'
import { useModalLock } from '../hooks/useModalLock.js'

export interface ModalProps {
  onClose: () => void
  /**
   * A modal that must be answered rather than dismissed: no Escape, no
   * scrim click. SingZ's first-run wizard is deliberately one of these.
   */
  persistent?: boolean
  /** Extra class on the card — `.log-card`, `.settings-card`, `.picker-card`
   *  and friends size it. These are also E2E selectors. */
  cardClassName?: string
  /** Blocks dismissal while a destructive action is in flight. */
  busy?: boolean
  children?: ReactNode
  'aria-label'?: string
}

/**
 * The scrim-plus-card shell, written once.
 *
 * SingZ had SEVEN copies of this, each repeating the same
 * `onClick={onClose}` / `onClick={e => e.stopPropagation()}` pair by hand —
 * and only two of the seven handled Escape at all, so whether the key worked
 * depended on which dialog you happened to have open. That inconsistency is
 * the bug this fixes; the deduplication is a side effect.
 */
export function Modal({
  onClose,
  persistent = false,
  cardClassName,
  busy = false,
  children,
  'aria-label': ariaLabel
}: ModalProps): React.JSX.Element {
  const cardRef = useRef<HTMLDivElement>(null)
  useModalLock(true)

  const dismiss = useCallback(() => {
    if (!persistent && !busy) onClose()
  }, [persistent, busy, onClose])

  useEffect(() => {
    if (persistent) return
    const onKey = (e: KeyboardEvent): void => {
      if (e.code !== 'Escape') return
      // Capture phase + stopPropagation: the app's window-level handler would
      // otherwise ALSO act on this keypress and, for example, leave karaoke
      // while merely closing a dialog on top of it.
      e.stopPropagation()
      dismiss()
    }
    window.addEventListener('keydown', onKey, true)
    return () => window.removeEventListener('keydown', onKey, true)
  }, [persistent, dismiss])

  return (
    <div
      className="modal-scrim"
      onClick={persistent ? undefined : dismiss}
      role="presentation"
    >
      <div
        ref={cardRef}
        className={cx('modal-card', cardClassName)}
        role="dialog"
        aria-modal="true"
        aria-label={ariaLabel}
        onClick={(e) => e.stopPropagation()}
      >
        {children}
      </div>
    </div>
  )
}

export interface ModalActionsProps {
  children?: ReactNode
  className?: string
}

/** The footer row of buttons. Kept a separate export because the app's E2E
 *  selects `.settings-card .modal-actions .pill`. */
export function ModalActions({ children, className }: ModalActionsProps): React.JSX.Element {
  return <div className={cx('modal-actions', className)}>{children}</div>
}
