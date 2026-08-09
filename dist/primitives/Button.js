import { jsx as _jsx } from "react/jsx-runtime";
import { forwardRef } from 'react';
import { cx } from '../util/cx.js';
/**
 * The pill button. Emits exactly `pill ghost small` etc. — the class names
 * are the contract, not an implementation detail, because the host app's
 * end-to-end tests select on them.
 *
 * `type` defaults to "button": every call site in SingZ passed it explicitly,
 * and a stray submit inside a form is a bug nobody enjoys finding.
 */
export const Button = forwardRef(function Button({ variant = 'ghost', size = 'md', icon = false, className, type = 'button', ...rest }, ref) {
    return (_jsx("button", { ref: ref, type: type, className: cx('pill', variant, size === 'sm' && 'small', icon && 'gear', className), ...rest }));
});
