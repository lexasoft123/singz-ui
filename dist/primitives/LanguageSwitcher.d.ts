import { type ReactNode } from 'react';
/** The reserved value of the "follow the system" entry. */
export declare const SYSTEM_LANGUAGE = "system";
export interface LanguageOption {
    /** A BCP-47 tag, or whatever the host keys its dictionaries by. */
    value: string;
    /**
     * The language's OWN name — "English", "简体中文", "Deutsch". Never
     * translated: the one reader who needs this control is the one who cannot
     * read the current language, and the endonym is the only label they are
     * certain to recognise.
     */
    label: string;
    /** The same name in the current UI language ("Chinese (Simplified)"), for
     *  everyone else. */
    hint?: string;
    /** A two-letter code for the row's left cell — "EN", "ZH". */
    code?: string;
    /**
     * A flag, drawn by the host — an inline SVG with the `.lang-flag` class, or
     * an emoji. It takes the row's cell over the code, and the pill shows the
     * flag of the language in use in place of the globe. Host-supplied because
     * a flag is an asset with a palette of its own, and because which flag
     * stands for a language is the host's call, not the kit's.
     */
    flag?: ReactNode;
}
export interface LanguageSwitcherProps {
    options: LanguageOption[];
    value: string;
    onChange: (value: string) => void;
    /**
     * Adds a "follow the system language" entry at the top, with the value
     * `SYSTEM_LANGUAGE`. `resolves` names the option it currently maps to, so
     * the trigger can show the language actually in use rather than the word
     * "System" — and `badge`, when given, marks it as automatic.
     */
    system?: {
        label: string;
        hint?: string;
        resolves: string;
        badge?: string;
    };
    /** `menu` hangs the rows off a pill; `list` lays them out inline. */
    variant?: 'menu' | 'list';
    /** menu only: which way the rows open. */
    placement?: 'bottom' | 'top';
    /** menu only: which edge of the pill the rows line up with. */
    align?: 'start' | 'end';
    size?: 'md' | 'sm';
    disabled?: boolean;
    className?: string;
    'aria-label'?: string;
}
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
export declare function LanguageSwitcher({ options, value, onChange, system, variant, placement, align, size, disabled, className, 'aria-label': ariaLabel }: LanguageSwitcherProps): React.JSX.Element;
