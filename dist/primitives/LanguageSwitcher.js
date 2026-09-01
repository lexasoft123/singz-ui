import { jsx as _jsx, jsxs as _jsxs } from "react/jsx-runtime";
import { useCallback, useEffect, useRef, useState } from 'react';
import { cx } from '../util/cx.js';
import { useDismissable } from '../hooks/useDismissable.js';
/** The reserved value of the "follow the system" entry. */
export const SYSTEM_LANGUAGE = 'system';
const Globe = () => (_jsxs("svg", { className: "lang-globe", width: "14", height: "14", viewBox: "0 0 24 24", fill: "none", stroke: "currentColor", strokeWidth: "1.8", strokeLinecap: "round", "aria-hidden": true, children: [_jsx("circle", { cx: "12", cy: "12", r: "9" }), _jsx("path", { d: "M3 12h18M12 3a14 14 0 0 1 0 18M12 3a14 14 0 0 0 0 18" })] }));
const Chevron = () => (_jsx("svg", { className: "lang-chev", width: "10", height: "10", viewBox: "0 0 24 24", fill: "none", stroke: "currentColor", strokeWidth: "2.6", strokeLinecap: "round", strokeLinejoin: "round", "aria-hidden": true, children: _jsx("path", { d: "m6 9 6 6 6-6" }) }));
const Check = () => (_jsx("svg", { className: "lang-check", width: "14", height: "14", viewBox: "0 0 24 24", fill: "none", stroke: "currentColor", strokeWidth: "3", strokeLinecap: "round", strokeLinejoin: "round", "aria-hidden": true, children: _jsx("path", { d: "M20 6 9 17l-5-5" }) }));
/** The "follow the system" row's glyph: a screen, for "whatever this machine says". */
const Screen = () => (_jsxs("svg", { width: "14", height: "14", viewBox: "0 0 24 24", fill: "none", stroke: "currentColor", strokeWidth: "1.8", strokeLinecap: "round", strokeLinejoin: "round", "aria-hidden": true, children: [_jsx("rect", { x: "3", y: "4", width: "18", height: "12", rx: "2" }), _jsx("path", { d: "M8 20h8M12 16v4" })] }));
/**
 * Where an app's language is chosen.
 *
 * Two shapes of the same rows: `menu` for a settings row or a rail, `list`
 * for a first-run screen with room to spare. Both are a listbox to the
 * keyboard — arrows move, Enter or Space picks, Escape and Tab leave — and
 * the pill goes `.on` while its rows are open, the way an engaged pill does
 * everywhere else in the kit.
 *
 * The kit carries NO strings. Every label — the languages' own names, the
 * follow-the-system entry, its badge — comes from the host, which is the only
 * party that knows what its dictionaries hold. That is also why there is no
 * default option list: a switcher that offered languages the app does not
 * have would be worse than none.
 */
export function LanguageSwitcher({ options, value, onChange, system, variant = 'menu', placement = 'bottom', align = 'end', size = 'sm', disabled = false, className, 'aria-label': ariaLabel }) {
    const rows = system
        ? [{ value: SYSTEM_LANGUAGE, label: system.label, hint: system.hint }, ...options]
        : options;
    const selected = Math.max(0, rows.findIndex((r) => r.value === value));
    const [open, setOpen] = useState(false);
    const trigger = useRef(null);
    const list = useRef(null);
    const items = useRef([]);
    /**
     * Close, and hand focus back to the pill if it was inside the rows. Outside
     * clicks arrive on pointerdown, before focus moves, so this also fires for
     * them — which parks focus on the pill rather than losing it to the body.
     */
    const close = useCallback(() => {
        setOpen(false);
        if (list.current?.contains(document.activeElement))
            trigger.current?.focus();
    }, []);
    useDismissable(list, close, { enabled: variant === 'menu' && open });
    // Opening puts focus on the current row, so the arrows start from it.
    useEffect(() => {
        if (variant === 'menu' && open)
            items.current[selected]?.focus();
    }, [open, selected, variant]);
    const pick = (v) => {
        if (v !== value)
            onChange(v);
        if (variant === 'menu')
            close();
    };
    const keys = (e) => {
        const focused = items.current.findIndex((el) => el === document.activeElement);
        const n = rows.length;
        const go = (i) => { e.preventDefault(); items.current[(i + n) % n]?.focus(); };
        switch (e.key) {
            case 'ArrowDown':
                go(focused + 1);
                break;
            case 'ArrowUp':
                go(focused - 1);
                break;
            case 'Home':
                go(0);
                break;
            case 'End':
                go(n - 1);
                break;
            case 'Tab':
                if (variant === 'menu')
                    setOpen(false);
                break;
            // Enter and Space land on the focused button's own click handler.
            default: break;
        }
    };
    const listbox = (_jsx("div", { ref: list, role: "listbox", "aria-label": ariaLabel, "data-sz-popover": variant === 'menu' ? '' : undefined, className: variant === 'menu' ? cx('lang-menu', placement, align) : cx('lang-list', className), onKeyDown: keys, children: rows.map((r, i) => {
            const on = i === selected;
            return (_jsxs("button", { ref: (el) => { items.current[i] = el; }, type: "button", role: "option", "aria-selected": on, tabIndex: variant === 'list' && !on ? -1 : 0, className: "lang-item", disabled: disabled, onClick: () => pick(r.value), children: [_jsx("span", { className: "lang-code", children: r.value === SYSTEM_LANGUAGE ? _jsx(Screen, {}) : (r.flag ?? r.code ?? r.label.slice(0, 2).toUpperCase()) }), _jsxs("span", { className: "lang-text", children: [_jsx("span", { className: "lang-label", children: r.label }), r.hint && _jsx("span", { className: "lang-hint", children: r.hint })] }), _jsx(Check, {})] }, r.value));
        }) }));
    if (variant === 'list')
        return listbox;
    // The pill names the language in USE — for `system`, the one it resolves to.
    const shown = value === SYSTEM_LANGUAGE && system
        ? options.find((o) => o.value === system.resolves) ?? rows[selected]
        : rows[selected];
    return (_jsxs("div", { className: cx('lang', className), children: [_jsxs("button", { ref: trigger, type: "button", className: cx('pill', 'ghost', size === 'sm' && 'small', 'lang-trigger', open && 'on'), disabled: disabled, "aria-haspopup": "listbox", "aria-expanded": open, "aria-label": ariaLabel, onClick: () => setOpen((o) => !o), children: [shown?.flag ?? _jsx(Globe, {}), _jsx("span", { className: "lang-current", children: shown?.label }), value === SYSTEM_LANGUAGE && system?.badge && _jsx("span", { className: "lang-badge", children: system.badge }), _jsx(Chevron, {})] }), open && listbox] }));
}
