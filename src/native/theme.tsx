import { createContext, useContext, type PropsWithChildren } from 'react'
import { Platform, type ViewStyle } from 'react-native'
import { tokens } from '../tokens/tokens.js'

export interface NativeTheme {
  readonly bg: string
  readonly panel: string
  readonly panelDeep: string
  readonly text: string
  readonly dim: string
  readonly faint: string
  readonly accent: string
  readonly accentInk: string
  readonly accentSoft: string
  readonly danger: string
  readonly line: string
  readonly lineStrong: string
  readonly glassFill: string
  readonly glassLine: string
  readonly glassRim: string
  readonly controlFill: string
  readonly controlLine: string
  readonly controlRim: string
  readonly footerFill: string
  readonly shadow: string
}

export const nightStudioNativeTheme: NativeTheme = Object.freeze({
  bg: tokens.bg,
  panel: tokens.panel,
  panelDeep: tokens['panel-deep'],
  text: tokens.text,
  dim: tokens.dim,
  faint: tokens.faint,
  accent: tokens.accent,
  accentInk: tokens['accent-ink'],
  accentSoft: tokens['accent-soft'],
  danger: tokens.danger,
  line: tokens.line,
  lineStrong: tokens['line-strong'],
  glassFill: tokens['glass-fill'],
  glassLine: tokens['glass-line'],
  glassRim: tokens['glass-rim'],
  controlFill: tokens['control-fill'],
  controlLine: tokens['control-line'],
  controlRim: tokens['control-rim'],
  footerFill: tokens['footer-fill'],
  shadow: tokens.shadow
})

const NativeThemeContext = createContext<NativeTheme>(nightStudioNativeTheme)

export function NativeThemeProvider({ theme, children }: PropsWithChildren<{ readonly theme: NativeTheme }>): React.JSX.Element {
  return <NativeThemeContext.Provider value={theme}>{children}</NativeThemeContext.Provider>
}

export function useNativeTheme(): NativeTheme {
  return useContext(NativeThemeContext)
}

export type GlassElevation = 'none' | 'surface' | 'dock'

/** The shared no-blur glass recipe used by Player, Training and bottom tabs. */
export function nativeGlassStyle(
  theme: NativeTheme = nightStudioNativeTheme,
  elevation: GlassElevation = 'surface',
  platform: string = Platform.OS
): ViewStyle {
  const shadow = elevation === 'dock'
    ? { shadowOpacity: 0.42, shadowRadius: 15, shadowOffset: { width: 0, height: 12 } }
    : { shadowOpacity: 0.4, shadowRadius: 14, shadowOffset: { width: 0, height: 10 } }
  return {
    backgroundColor: theme.glassFill,
    borderWidth: 1,
    borderColor: theme.glassLine,
    borderTopColor: theme.glassRim,
    borderCurve: 'continuous',
    ...(platform === 'ios' && elevation !== 'none' ? { shadowColor: theme.shadow, ...shadow } : null)
  } as ViewStyle
}
