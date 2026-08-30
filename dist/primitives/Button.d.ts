import { type ButtonHTMLAttributes, type ReactNode } from 'react';
export type ButtonVariant = 'ghost' | 'primary' | 'danger';
export interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
    variant?: ButtonVariant;
    size?: 'md' | 'sm';
    /** Icon-only, tightened padding — `.pill.gear`. */
    icon?: boolean;
    /**
     * The action is currently engaged — a panel that is open, a mode that is
     * running. Paints `.pill.on` and sets aria-pressed, which makes the pill a
     * toggle to a screen reader as well as to the eye.
     *
     * Omitted, not defaulted to false: a one-shot button that announced
     * itself as an unpressed toggle would be worse than saying nothing.
     */
    active?: boolean;
    /** Appended AFTER the kit's classes, so hosts keep their own hooks:
     *  `mic-toggle`, `update-chip`, `catalog-btn` are all E2E selectors. */
    className?: string;
    children?: ReactNode;
}
/**
 * The pill button. Emits exactly `pill ghost small` etc. — the class names
 * are the contract, not an implementation detail, because the host app's
 * end-to-end tests select on them.
 *
 * `type` defaults to "button": every call site in SingZ passed it explicitly,
 * and a stray submit inside a form is a bug nobody enjoys finding.
 */
export declare const Button: import("react").ForwardRefExoticComponent<ButtonProps & import("react").RefAttributes<HTMLButtonElement>>;
