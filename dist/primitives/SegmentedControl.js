import { jsx as _jsx } from "react/jsx-runtime";
import { cx } from '../util/cx.js';
/**
 * `.mode-seg` — a row of buttons where one is `.on`.
 *
 * The buttons are DIRECT children of `.mode-seg` and nothing may come between
 * them: the Windows smoke test selects `.mode-seg button:nth-child(1|2)` and
 * reads `className.includes('on')`. A wrapper element per option would look
 * harmless and break it.
 */
export function SegmentedControl({ options, value, onChange, disabled = false, className, 'aria-label': ariaLabel }) {
    return (_jsx("div", { className: cx('mode-seg', className), role: "group", "aria-label": ariaLabel, children: options.map((o) => (_jsx("button", { type: "button", title: o.title, disabled: disabled || o.disabled, "aria-pressed": o.value === value, className: o.value === value ? 'on' : '', onClick: () => onChange(o.value), children: o.label }, String(o.value)))) }));
}
