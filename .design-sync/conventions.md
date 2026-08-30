# @singz/ui — how to build with it

The design system behind **SingZ**, a desktop app for singers: it splits a song into
stems, shows synced lyrics, and matches pitch. The look is a *night studio* — warm
tungsten darks, one amber accent, pill-shaped controls, and stem lanes colour-coded
by instrument. Everything here is the app's real shipped code, not a mock-up.

## Ground rules

**The surface is dark, and the kit paints it.** `:where(body)` sets
`--sz-bg` / `--sz-text` / `--sz-font-display`. Zero specificity, so any rule you write
wins — but if you put components on a light panel of your own, give that panel a
`color` too or ghost buttons go near-white on white.

**Colour comes from tokens, never literals.** Every value is a `--sz-*` custom
property. Two palettes ship: night-studio at `:root`, and a light paper "atelier"
under `data-sz-palette="atelier"` on `<html>`. Override the variables for anything
else. If something stays dark after you re-theme, that's a bug in the kit, not in
your override.

**Class names are plain, and deliberately so.** `pill`, `chip`, `mode-seg`,
`modal-card`, `badge`, `linkish`. No `sz-` prefix. They read like ordinary CSS, and
they collide with ordinary CSS — scope your own rules if you're mixing in another kit.

## The vocabulary

| Want | Use | Notes |
|---|---|---|
| An action | `<Button variant="ghost" \| "primary" \| "danger" size="md" \| "sm">` | One `primary` per view. `danger` is an outline, not a fill. |
| An action that is engaged | `<Button active>` | The open panel, the running mode. Paints `.pill.on`, sets aria-pressed. |
| A toggle in a dense row | `<Chip active>` | 26×24. Mute/solo scale. `wide` when it holds a word. |
| A label over a group | `class="eyebrow"` | A type style, not a component. Put it on the element you already have. |
| A scrub bar or a level | `<input type="range" class="slider seek \| slider">` | `seek` fills from the host's `--p`. |
| A small either/or | `<SegmentedControl>` | Pill-bordered, amber on the chosen segment. |
| A dialog | `<Modal>` + `.modal-title` + `.modal-body` + `<ModalActions>` | See below. |
| A tag | `<Badge>` | Uppercase, letter-spaced, outline. Nouns, lowercase input. `caps={false}` when the case means something. |
| A live/idle indicator | `<StatusDot tone>` | 6px. Pair with a label; the dot alone isn't accessible. |
| An inline action inside prose | `<LinkButton>` | Reads as a link, *is* a `<button>`. |
| Audio | `<Waveform>` | Needs a sized, positioned parent and host-set `--stem` / `--p`. |
| Frameless-window chrome | `<WindowButtons api={…}>` | You inject the five window calls. |

## Things that will bite you

**`disabled` now paints.** As of 1.4.0 `.pill`, `.chip`, `.linkish`, `.round-ghost`
and `.slider` all dim to `opacity: .45` and stop lifting on hover; `.mode-seg button`
keeps its own long-standing `.4`. Host rules that were dimming these by hand are now
redundant rather than wrong. (Previews captured before 1.4.0 show the old
undimmed rendering beside a host-dimmed one.)

**`Modal` is the shell, not the sizing.** Width comes from `cardClassName`
(`.settings-card`, `.picker-card`, `.confirm-card`), and those classes belong to the
host app. Without one you get the default card. `persistent` removes Escape and
scrim-click, for dialogs that must be answered. `busy` blocks dismissal while
something is in flight and paints nothing — dim the actions yourself.

**`Waveform` and `WindowButtons` are positioned.** `.wave` is absolute; give it a
parent with a height. `.win-controls` is `position: fixed`, so anywhere but the real
window titlebar it escapes to the viewport corner — put a `transform: translateZ(0)`
on the box that should contain it.

**The kit paints the room.** `body::before` / `body::after` carry two ambient lamps
and a film grain, at `:where()` specificity, and `.app` is raised to `z-index: 1` so
the grain grades everything. If you draw your own `body::before`, you replace the
lamps — that is a choice, not a collision. To switch the room off, set
`--sz-ambient-*` to `transparent` and `--sz-grain-opacity` to `0`.

**Motion is gated for you.** As of 1.4.1 `primitives.css` ends with a
`prefers-reduced-motion` `!important` sweep covering every animation and
transition on the page — the kit's four keyframes and the host's own. Do NOT
add a second copy in the app: two `!important` sweeps make the rule
un-overridable, and there is no way back out for the one animation you may
legitimately need. Motion the sweep cannot reach — a `requestAnimationFrame`
loop, a scripted scroll — is still yours to gate in JS.

**Hosts outrank the kit on state colour.** SingZ fills a pressed `Chip` with that
lane's `--stem` rather than the amber accent. That's expected: write a more specific
rule and it wins.

## Voice

User-visible copy is sentence-case, friendly, and states costs up front — sizes in
MB, time in minutes. "Install it once and every future split uses it," not
"Dependency required." Buttons are verbs. Nothing shouts.
