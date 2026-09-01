import { jsx as _jsx } from "react/jsx-runtime";
import { useCallback, useEffect, useRef } from 'react';
import { cx } from '../util/cx.js';
import { useModalLock } from '../hooks/useModalLock.js';
/**
 * The scrim-plus-card shell, written once.
 *
 * SingZ had SEVEN copies of this, each repeating the same
 * `onClick={onClose}` / `onClick={e => e.stopPropagation()}` pair by hand —
 * and only two of the seven handled Escape at all, so whether the key worked
 * depended on which dialog you happened to have open. That inconsistency is
 * the bug this fixes; the deduplication is a side effect.
 */
export function Modal({ onClose, persistent = false, cardClassName, busy = false, children, 'aria-label': ariaLabel }) {
    const cardRef = useRef(null);
    useModalLock(true);
    const dismiss = useCallback(() => {
        if (!persistent && !busy)
            onClose();
    }, [persistent, busy, onClose]);
    useEffect(() => {
        if (persistent)
            return;
        const onKey = (e) => {
            if (e.code !== 'Escape')
                return;
            // A popover open INSIDE the card — a LanguageSwitcher's rows — owns this
            // press: it closes itself and stops here. Both listen on the window in
            // the capture phase, and this one registered first, so without the
            // check Escape would close the whole dialog under an open menu.
            if (e.target?.closest?.('[data-sz-popover]'))
                return;
            // Capture phase + stopPropagation: the app's window-level handler would
            // otherwise ALSO act on this keypress and, for example, leave karaoke
            // while merely closing a dialog on top of it.
            e.stopPropagation();
            dismiss();
        };
        window.addEventListener('keydown', onKey, true);
        return () => window.removeEventListener('keydown', onKey, true);
    }, [persistent, dismiss]);
    return (_jsx("div", { className: "modal-scrim", onClick: persistent ? undefined : dismiss, role: "presentation", children: _jsx("div", { ref: cardRef, className: cx('modal-card', cardClassName), role: "dialog", "aria-modal": "true", "aria-label": ariaLabel, onClick: (e) => e.stopPropagation(), children: children }) }));
}
/** The footer row of buttons. Kept a separate export because the app's E2E
 *  selects `.settings-card .modal-actions .pill`. */
export function ModalActions({ children, className }) {
    return _jsx("div", { className: cx('modal-actions', className), children: children });
}
