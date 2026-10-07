import { jsx as _jsx, jsxs as _jsxs } from "react/jsx-runtime";
import { cx } from '../util/cx.js';
/** Centered status or feedback, with a full-width optional second line. */
export function Banner({ tone = 'info', icon, heading, children, footer, className, ...rest }) {
    return (_jsxs("div", { role: tone === 'danger' ? 'alert' : 'status', ...rest, className: cx('sz-banner', `sz-banner--${tone}`, className), children: [_jsxs("div", { className: "sz-banner__content", children: [icon && _jsx("span", { className: "sz-banner__icon", "aria-hidden": "true", children: icon }), heading && _jsx("strong", { className: "sz-banner__heading", children: heading }), children && _jsx("span", { className: "sz-banner__detail", children: children })] }), footer && _jsx("div", { className: "sz-banner__footer", children: footer })] }));
}
