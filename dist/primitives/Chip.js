import { jsx as _jsx } from "react/jsx-runtime";
import { forwardRef } from 'react';
import { cx } from '../util/cx.js';
/** A small square toggle — the M / S buttons on a mixer lane, and friends. */
export const Chip = forwardRef(function Chip({ active = false, className, type = 'button', ...rest }, ref) {
    return (_jsx("button", { ref: ref, type: type, "aria-pressed": active, className: cx('chip', className, active && 'active'), ...rest }));
});
