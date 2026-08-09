import { jsx as _jsx } from "react/jsx-runtime";
import { cx } from '../util/cx.js';
/** A 7px state dot. `warn` pulses — and the host must pause that animation
 *  while a modal covers the app, or the scrim's backdrop blur re-rasters the
 *  whole window on every frame. SingZ does this via `body.modal-open`. */
export function StatusDot({ tone, className }) {
    return _jsx("span", { className: cx('dot', tone, className) });
}
/** `.linkish` — a text button that reads as a link but is a real button. */
export function LinkButton({ className, type = 'button', ...rest }) {
    return _jsx("button", { type: type, className: cx('linkish', className), ...rest });
}
/** `.badge` — a small uppercase outline tag. */
export function Badge({ children, className }) {
    return _jsx("span", { className: cx('badge', className), children: children });
}
