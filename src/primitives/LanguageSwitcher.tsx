import { useCallback, useEffect, useRef, useState, type KeyboardEvent, type ReactNode } from 'react'
import { cx } from '../util/cx.js'
import { useDismissable } from '../hooks/useDismissable.js'

/** The reserved value of the "follow the system" entry. */
export const SYSTEM_LANGUAGE = 'system'

export interface LanguageOption {
  /** A BCP-47 tag, or whatever the host keys its dictionaries by. */
  value: string
  /**
   * The language's OWN name — "English", "简体中文", "Deutsch". Never
   * translated: the one reader who needs this control is the one who cannot
   * read the current language, and the endonym is the only label they are
   * certain to recognise.
   */
  label: string
  /** The same name in the current UI language ("Chinese (Simplified)"), for
   *  everyone else. */
  hint?: string
  /** A two-letter code for the row's left cell — "EN", "ZH". */
  code?: string
  /**
   * A flag, drawn by the host — an inline SVG with the `.lang-flag` class, or
   * an emoji. It takes the row's cell over the code, and the pill shows the
   * flag of the language in use in place of the globe. Host-supplied because
   * a flag is an asset with a palette of its own, and because which flag
   * stands for a language is the host's call, not the kit's.
   */
  flag?: ReactNode
}

export interface LanguageSwitcherProps {
  options: LanguageOption[]
  value: string
  onChange: (value: string) => void
  /**
   * Adds a "follow the system language" entry at the top, with the value
   * `SYSTEM_LANGUAGE`. `resolves` names the option it currently maps to, so
   * the trigger can show the language actually in use rather than the word
   * "System" — and `badge`, when given, marks it as automatic.
   */
  system?: { label: string; hint?: string; resolves: string; badge?: string }
  /** `menu` hangs the rows off a pill; `list` lays them out inline. */
  variant?: 'menu' | 'list'
  /** menu only: which way the rows open. */
  placement?: 'bottom' | 'top'
  /** menu only: which edge of the pill the rows line up with. */
  align?: 'start' | 'end'
  size?: 'md' | 'sm'
  disabled?: boolean
  className?: string
  'aria-label'?: string
}

const Globe = (): React.JSX.Element => (
  <svg className="lang-globe" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.8" strokeLinecap="round" aria-hidden>
    <circle cx="12" cy="12" r="9" />
    <path d="M3 12h18M12 3a14 14 0 0 1 0 18M12 3a14 14 0 0 0 0 18" />
  </svg>
)

const Chevron = (): React.JSX.Element => (
  <svg className="lang-chev" width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.6" strokeLinecap="round" strokeLinejoin="round" aria-hidden>
    <path d="m6 9 6 6 6-6" />
  </svg>
)

const Check = (): React.JSX.Element => (
  <svg className="lang-check" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="3" strokeLinecap="round" strokeLinejoin="round" aria-hidden>
    <path d="M20 6 9 17l-5-5" />
  </svg>
)

/** The "follow the system" row's glyph: a screen, for "whatever this machine says". */
const Screen = (): React.JSX.Element => (
  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.8" strokeLinecap="round" strokeLinejoin="round" aria-hidden>
    <rect x="3" y="4" width="18" height="12" rx="2" />
    <path d="M8 20h8M12 16v4" />
  </svg>
)

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
export function LanguageSwitcher({
  options,
  value,
  onChange,
  system,
  variant = 'menu',
  placement = 'bottom',
  align = 'end',
  size = 'sm',
  disabled = false,
  className,
  'aria-label': ariaLabel
}: LanguageSwitcherProps): React.JSX.Element {
  const rows: LanguageOption[] = system
    ? [{ value: SYSTEM_LANGUAGE, label: system.label, hint: system.hint }, ...options]
    : options
  const selected = Math.max(0, rows.findIndex((r) => r.value === value))

  const [open, setOpen] = useState(false)
  const trigger = useRef<HTMLButtonElement>(null)
  const list = useRef<HTMLDivElement>(null)
  const items = useRef<(HTMLButtonElement | null)[]>([])

  /**
   * Close, and hand focus back to the pill if it was inside the rows. Outside
   * clicks arrive on pointerdown, before focus moves, so this also fires for
   * them — which parks focus on the pill rather than losing it to the body.
   */
  const close = useCallback((): void => {
    setOpen(false)
    if (list.current?.contains(document.activeElement)) trigger.current?.focus()
  }, [])

  useDismissable(list, close, { enabled: variant === 'menu' && open })

  // Opening puts focus on the current row, so the arrows start from it.
  useEffect(() => {
    if (variant === 'menu' && open) items.current[selected]?.focus()
  }, [open, selected, variant])

  const pick = (v: string): void => {
    if (v !== value) onChange(v)
    if (variant === 'menu') close()
  }

  const keys = (e: KeyboardEvent<HTMLDivElement>): void => {
    const focused = items.current.findIndex((el) => el === document.activeElement)
    const n = rows.length
    const go = (i: number): void => { e.preventDefault(); items.current[(i + n) % n]?.focus() }
    switch (e.key) {
      case 'ArrowDown': go(focused + 1); break
      case 'ArrowUp': go(focused - 1); break
      case 'Home': go(0); break
      case 'End': go(n - 1); break
      case 'Tab': if (variant === 'menu') setOpen(false); break
      // Enter and Space land on the focused button's own click handler.
      default: break
    }
  }

  const listbox = (
    <div
      ref={list}
      role="listbox"
      aria-label={ariaLabel}
      data-sz-popover={variant === 'menu' ? '' : undefined}
      className={variant === 'menu' ? cx('lang-menu', placement, align) : cx('lang-list', className)}
      onKeyDown={keys}
    >
      {rows.map((r, i) => {
        const on = i === selected
        return (
          <button
            key={r.value}
            ref={(el) => { items.current[i] = el }}
            type="button"
            role="option"
            aria-selected={on}
            tabIndex={variant === 'list' && !on ? -1 : 0}
            className="lang-item"
            disabled={disabled}
            onClick={() => pick(r.value)}
          >
            <span className="lang-code">
              {r.value === SYSTEM_LANGUAGE ? <Screen /> : (r.flag ?? r.code ?? r.label.slice(0, 2).toUpperCase())}
            </span>
            <span className="lang-text">
              <span className="lang-label">{r.label}</span>
              {r.hint && <span className="lang-hint">{r.hint}</span>}
            </span>
            <Check />
          </button>
        )
      })}
    </div>
  )

  if (variant === 'list') return listbox

  // The pill names the language in USE — for `system`, the one it resolves to.
  const shown = value === SYSTEM_LANGUAGE && system
    ? options.find((o) => o.value === system.resolves) ?? rows[selected]
    : rows[selected]

  return (
    <div className={cx('lang', className)}>
      <button
        ref={trigger}
        type="button"
        className={cx('pill', 'ghost', size === 'sm' && 'small', 'lang-trigger', open && 'on')}
        disabled={disabled}
        aria-haspopup="listbox"
        aria-expanded={open}
        aria-label={ariaLabel}
        onClick={() => setOpen((o) => !o)}
      >
        {shown?.flag ?? <Globe />}
        <span className="lang-current">{shown?.label}</span>
        {value === SYSTEM_LANGUAGE && system?.badge && <span className="lang-badge">{system.badge}</span>}
        <Chevron />
      </button>
      {open && listbox}
    </div>
  )
}
