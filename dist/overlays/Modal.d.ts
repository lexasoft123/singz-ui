import { type ReactNode } from 'react';
export interface ModalProps {
    onClose: () => void;
    /**
     * A modal that must be answered rather than dismissed: no Escape, no
     * scrim click. SingZ's first-run wizard is deliberately one of these.
     */
    persistent?: boolean;
    /** Extra class on the card — `.log-card`, `.settings-card`, `.picker-card`
     *  and friends size it. These are also E2E selectors. */
    cardClassName?: string;
    /** Blocks dismissal while a destructive action is in flight. */
    busy?: boolean;
    children?: ReactNode;
    'aria-label'?: string;
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
export declare function Modal({ onClose, persistent, cardClassName, busy, children, 'aria-label': ariaLabel }: ModalProps): React.JSX.Element;
export interface ModalActionsProps {
    children?: ReactNode;
    className?: string;
}
/** The footer row of buttons. Kept a separate export because the app's E2E
 *  selects `.settings-card .modal-actions .pill`. */
export declare function ModalActions({ children, className }: ModalActionsProps): React.JSX.Element;
