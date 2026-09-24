import type { ReactNode } from 'react'
import {
  Pressable,
  StyleSheet,
  Text,
  View,
  type StyleProp,
  type ViewProps,
  type ViewStyle
} from 'react-native'
import { nativeGlassStyle, useNativeTheme, type GlassElevation } from './theme.js'

export interface GlassSurfaceProps extends ViewProps {
  readonly radius?: number
  readonly elevation?: GlassElevation
  readonly style?: StyleProp<ViewStyle>
  readonly children?: ReactNode
}

export function GlassSurface({ radius = 24, elevation = 'surface', style, children, ...props }: GlassSurfaceProps): React.JSX.Element {
  const theme = useNativeTheme()
  return <View {...props} style={[nativeGlassStyle(theme, elevation), { borderRadius: radius }, style]}>{children}</View>
}

export function GlassHeader({ title, onBack, backLabel = 'Back', trailing }: {
  readonly title: string
  readonly onBack: () => void
  readonly backLabel?: string
  readonly trailing?: ReactNode
}): React.JSX.Element {
  const theme = useNativeTheme()
  return (
    <GlassSurface radius={31} style={s.header}>
      <Pressable collapsable={false} accessibilityRole="button" accessibilityLabel={backLabel} hitSlop={10} pressRetentionOffset={12} onPress={onBack} style={({ pressed }) => [s.headerAction, pressed && s.pressed]}>
        <Text pointerEvents="none" style={[s.backChevron, { color: theme.text }]}>‹</Text>
      </Pressable>
      <Text pointerEvents="none" accessibilityRole="header" style={[s.headerTitle, { color: theme.text }]}>{title}</Text>
      <View pointerEvents="box-none" style={s.headerTrailing}>{trailing}</View>
    </GlassSurface>
  )
}

export interface ChoiceChipProps {
  readonly label: string
  readonly selected?: boolean
  readonly disabled?: boolean
  readonly onPress: () => void
  readonly style?: StyleProp<ViewStyle>
}

export function ChoiceChip({ label, selected = false, disabled = false, onPress, style }: ChoiceChipProps): React.JSX.Element {
  const theme = useNativeTheme()
  return (
    <Pressable
      accessibilityRole="button"
      accessibilityState={{ selected, disabled }}
      disabled={disabled}
      onPress={onPress}
      style={({ pressed }) => [
        s.chip,
        { backgroundColor: theme.controlFill, borderColor: theme.controlLine, borderTopColor: theme.controlRim },
        selected && { borderColor: theme.accent, borderTopColor: theme.accent, backgroundColor: theme.accentSoft },
        disabled && s.disabled,
        pressed && s.pressed,
        style
      ]}
    >
      <Text style={[s.chipText, { color: selected ? theme.accent : theme.dim }]}>{label}</Text>
    </Pressable>
  )
}

export function PrimaryAction({ label, icon, onPress, disabled = false, style }: {
  readonly label: string
  readonly icon?: ReactNode
  readonly onPress: () => void
  readonly disabled?: boolean
  readonly style?: StyleProp<ViewStyle>
}): React.JSX.Element {
  const theme = useNativeTheme()
  return (
    <Pressable accessibilityRole="button" accessibilityLabel={label} disabled={disabled} onPress={onPress} style={({ pressed }) => [s.primary, { backgroundColor: theme.accent }, disabled && s.disabled, pressed && s.pressed, style]}>
      {icon}
      <Text style={[s.primaryText, { color: theme.accentInk }]}>{label}</Text>
    </Pressable>
  )
}

export function RoundAction({ label, icon, onPress, size = 54, backgroundColor, disabled = false }: {
  readonly label: string
  readonly icon: ReactNode
  readonly onPress: () => void
  readonly size?: number
  readonly backgroundColor?: string
  readonly disabled?: boolean
}): React.JSX.Element {
  const theme = useNativeTheme()
  return (
    <Pressable accessibilityRole="button" accessibilityLabel={label} hitSlop={8} disabled={disabled} onPress={onPress} style={({ pressed }) => ({
      width: size,
      height: size,
      borderRadius: size / 2,
      backgroundColor: pressed ? theme.controlRim : backgroundColor ?? theme.controlFill,
      opacity: disabled ? 0.35 : 1,
      alignItems: 'center',
      justifyContent: 'center',
      borderWidth: 1,
      borderColor: theme.controlLine
    })}>
      {icon}
    </Pressable>
  )
}

export function SettingsCard({ children, style }: { readonly children: ReactNode; readonly style?: StyleProp<ViewStyle> }): React.JSX.Element {
  return <GlassSurface radius={26} style={[s.settingsCard, style]}>{children}</GlassSurface>
}

export function Hairline(): React.JSX.Element {
  const theme = useNativeTheme()
  return <View style={[s.hairline, { backgroundColor: theme.line }]} />
}

export function SettingsRow({ label, value, expanded = false, onPress }: {
  readonly label: string
  readonly value: string
  readonly expanded?: boolean
  readonly onPress: () => void
}): React.JSX.Element {
  const theme = useNativeTheme()
  return (
    <Pressable accessibilityRole="button" accessibilityState={{ expanded }} onPress={onPress} style={({ pressed }) => [s.settingsRow, pressed && s.pressed]}>
      <Text style={[s.settingsLabel, { color: theme.dim }]}>{label}</Text>
      <View style={s.settingsValueRow}>
        <Text numberOfLines={1} style={[s.settingsValue, { color: theme.text }]}>{value}</Text>
        <Text style={[s.settingsChevron, { color: theme.accent }]}>{expanded ? '⌃' : '›'}</Text>
      </View>
    </Pressable>
  )
}

export function StickyActionFooter({ children, style }: { readonly children: ReactNode; readonly style?: StyleProp<ViewStyle> }): React.JSX.Element {
  const theme = useNativeTheme()
  return <View style={[s.stickyFooter, { backgroundColor: theme.footerFill, borderTopColor: theme.line, shadowColor: theme.shadow }, style]}>{children}</View>
}

export function GlassTabBar({ children, style }: { readonly children: ReactNode; readonly style?: StyleProp<ViewStyle> }): React.JSX.Element {
  return <GlassSurface radius={32} elevation="dock" accessibilityRole="tablist" style={[s.tabBar, style]}>{children}</GlassSurface>
}

export function GlassTab({ label, accessibilityLabel = label, selected, icon, onPress, onLongPress, testID }: {
  readonly label: string
  readonly accessibilityLabel?: string
  readonly selected: boolean
  readonly icon: ReactNode
  readonly onPress: () => void
  readonly onLongPress?: () => void
  readonly testID?: string
}): React.JSX.Element {
  const theme = useNativeTheme()
  const color = selected ? theme.accent : theme.dim
  return (
    <Pressable accessibilityRole="tab" accessibilityLabel={accessibilityLabel} accessibilityState={{ selected }} testID={testID} onPress={onPress} onLongPress={onLongPress} style={({ pressed }) => [s.tab, selected && { backgroundColor: theme.accentSoft, borderWidth: 1, borderColor: theme.controlRim }, pressed && s.pressed]}>
      {icon}
      <Text style={[s.tabLabel, { color }]}>{label}</Text>
    </Pressable>
  )
}

export function FeatureCard({ title, description, glyph, onPress }: {
  readonly title: string
  readonly description: string
  readonly glyph: ReactNode
  readonly onPress: () => void
}): React.JSX.Element {
  const theme = useNativeTheme()
  return (
    <Pressable accessibilityRole="button" onPress={onPress} style={({ pressed }) => [nativeGlassStyle(theme), s.featureCard, pressed && s.pressed]}>
      <View pointerEvents="none" style={s.featureGlyphRail}><View style={s.featureGlyph}>{glyph}</View></View>
      <View style={s.featureCopy}>
        <Text style={[s.featureTitle, { color: theme.text }]}>{title}</Text>
        <Text style={[s.featureDescription, { color: theme.dim }]}>{description}</Text>
      </View>
    </Pressable>
  )
}

export function ListEntry({ title, detail, trailing, onPress }: {
  readonly title: string
  readonly detail: string
  readonly trailing?: ReactNode
  readonly onPress: () => void
}): React.JSX.Element {
  const theme = useNativeTheme()
  return (
    <Pressable accessibilityRole="button" onPress={onPress} style={({ pressed }) => [nativeGlassStyle(theme), s.listEntry, pressed && s.pressed]}>
      <View style={s.listEntryCopy}>
        <Text style={[s.featureTitle, { color: theme.text }]}>{title}</Text>
        <Text style={[s.featureDescription, { color: theme.dim }]}>{detail}</Text>
      </View>
      {trailing ?? <Text style={[s.listChevron, { color: theme.accent }]}>›</Text>}
    </Pressable>
  )
}

export interface ReferenceControlsProps {
  readonly volumePercent: number
  readonly volumePosition: number
  readonly volumeMinPercent?: number
  readonly volumeMaxPercent?: number
  readonly testLabel: string
  readonly testing?: boolean
  readonly testIcon?: ReactNode
  readonly onTest: () => void
  readonly onDecrease: () => void
  readonly onIncrease: () => void
  readonly decreaseDisabled?: boolean
  readonly increaseDisabled?: boolean
  readonly hint?: string
  readonly pitchWindow?: { readonly value: number; readonly options: readonly number[]; readonly onChange: (value: number) => void }
  /**
   * The words on the panel, for a host that speaks another language. The kit
   * carries English only as the default; every label here is the host's.
   * `title` and `pitchWindow` are shown upper-cased, as the English is.
   */
  readonly labels?: Partial<ReferenceControlsLabels>
}

export interface ReferenceControlsLabels {
  readonly title: string
  readonly pitchWindow: string
  /** Screen reader: the panel as a whole. */
  readonly volume: string
  /** Screen reader: the panel's value, e.g. "65 percent". */
  readonly volumeValue: (percent: number) => string
  readonly decrease: string
  readonly increase: string
}

const REFERENCE_LABELS: ReferenceControlsLabels = {
  title: 'Reference sound',
  pitchWindow: 'Pitch window',
  volume: 'Reference sound volume',
  volumeValue: (percent) => `${percent} percent`,
  decrease: 'Decrease reference volume',
  increase: 'Increase reference volume'
}

export function ReferenceControls(props: ReferenceControlsProps): React.JSX.Element {
  const theme = useNativeTheme()
  const position = Math.max(0, Math.min(1, props.volumePosition))
  const minPercent = props.volumeMinPercent ?? 0
  const maxPercent = props.volumeMaxPercent ?? 100
  const L = { ...REFERENCE_LABELS, ...props.labels }
  return (
    <GlassSurface radius={25} accessibilityRole="adjustable" accessibilityLabel={L.volume} accessibilityValue={{ min: minPercent, max: maxPercent, now: props.volumePercent, text: L.volumeValue(props.volumePercent) }} style={s.referencePanel}>
      <View style={s.referenceHeader}>
        <View><Text style={[s.utilityLabel, { color: theme.dim }]}>{L.title.toLocaleUpperCase()}</Text><Text style={[s.referenceValue, { color: theme.text }]}>{props.volumePercent}%</Text></View>
        <Pressable accessibilityRole="button" accessibilityLabel={props.testLabel} disabled={props.testing} onPress={props.onTest} style={({ pressed }) => [s.referenceTest, { backgroundColor: theme.accent }, (pressed || props.testing) && s.pressed]}>
          {props.testIcon}<Text style={[s.referenceTestText, { color: theme.accentInk }]}>{props.testing ? 'Playing…' : props.testLabel}</Text>
        </Pressable>
      </View>
      <View style={s.referenceVolumeRow}>
        <RoundAction label={L.decrease} icon={<Text style={[s.referenceStepText, { color: theme.text }]}>−</Text>} onPress={props.onDecrease} size={48} disabled={props.decreaseDisabled} />
        <View style={[s.referenceTrack, { backgroundColor: theme.controlLine }]}><View style={[s.referenceFill, { width: `${Math.round(position * 100)}%`, backgroundColor: theme.accent }]} /></View>
        <RoundAction label={L.increase} icon={<Text style={[s.referenceStepText, { color: theme.text }]}>+</Text>} onPress={props.onIncrease} size={48} disabled={props.increaseDisabled} />
      </View>
      {props.hint && <Text style={[s.referenceHint, { color: theme.dim }]}>{props.hint}</Text>}
      {props.pitchWindow && <><Hairline /><View style={s.pitchWindow}><View><Text style={[s.utilityLabel, { color: theme.dim }]}>{L.pitchWindow.toLocaleUpperCase()}</Text><Text style={[s.pitchWindowValue, { color: theme.text }]}>±{props.pitchWindow.value}¢</Text></View><View style={s.choiceRow}>{props.pitchWindow.options.map((value) => <ChoiceChip key={value} label={`±${value}¢`} selected={props.pitchWindow?.value === value} onPress={() => props.pitchWindow?.onChange(value)} />)}</View></View></>}
    </GlassSurface>
  )
}

export interface PitchTargetItem { readonly label: string; readonly state: 'future' | 'active' | 'done' }

export function PitchTarget({ noteName, eyebrow, sequence = [], testID, style }: {
  readonly noteName: string
  readonly eyebrow: string
  readonly sequence?: readonly PitchTargetItem[]
  readonly testID?: string
  readonly style?: StyleProp<ViewStyle>
}): React.JSX.Element {
  const theme = useNativeTheme()
  const match = /^(.*?)(-?\d+)$/.exec(noteName)
  const pitch = match?.[1] ?? noteName
  const octave = match?.[2] ?? ''
  const octaveOffset = pitch.length > 1 ? 76 : 58
  const activeSequenceIndex = sequence.findIndex((item) => item.state === 'active')
  return (
    <View testID={testID} style={[s.targetArea, style]}>
      <Text style={[s.targetEyebrow, { color: theme.accent }]}>{eyebrow}</Text>
      <View accessibilityLabel={`Target note ${noteName}`} style={s.targetLockup}><Text style={[s.targetPitch, { color: theme.text }]}>{pitch}</Text><Text style={[s.targetOctave, { color: theme.accent, marginLeft: octaveOffset }]}>{octave}</Text></View>
      {sequence.length > 1 && <View accessibilityLabel={`Note ${Math.max(0, activeSequenceIndex) + 1} of ${sequence.length}`} style={s.targetSequence}>{sequence.map((item, index) => <View key={`${item.label}-${index}`} style={[s.targetSequenceItem, { backgroundColor: theme.panelDeep, borderColor: theme.controlLine }, item.state === 'done' && { backgroundColor: theme.line, borderColor: theme.lineStrong }, item.state === 'active' && { backgroundColor: theme.accentSoft, borderColor: theme.accent }]}><Text style={[s.targetSequenceText, { color: item.state === 'future' ? theme.dim : theme.text }]}>{item.label}</Text></View>)}</View>}
    </View>
  )
}

export function Countdown({ value, label = 'SING IN', hint }: { readonly value: number; readonly label?: string; readonly hint: string }): React.JSX.Element {
  const theme = useNativeTheme()
  const spokenLabel = `${label.charAt(0)}${label.slice(1).toLowerCase()}`
  return <View accessibilityLabel={`${spokenLabel} ${value}`} style={s.countdown}><Text style={[s.countdownLabel, { color: theme.accent }]}>{label}</Text><Text accessibilityLiveRegion="polite" style={[s.countdownNumber, { color: theme.text, textShadowColor: theme.accent }]}>{value}</Text><Text style={[s.countdownHint, { color: theme.dim }]}>{hint}</Text></View>
}

export interface PitchMeterProps {
  readonly cents: number | null
  readonly pitchWindowCents: number
  readonly detectedNote: string | null
  readonly progress: number
  readonly centered: boolean
  readonly instruction: string
  readonly reading: string
  readonly accessibilityReading?: string
  readonly hint: string
  /** The meter's own words, for a host in another language (English by
   *  default; `flat`, `sharp` and `youAreSinging` are shown upper-cased). */
  readonly labels?: Partial<PitchMeterLabels>
}

export interface PitchMeterLabels {
  readonly flat: string
  readonly sharp: string
  readonly youAreSinging: string
  /** Screen reader: the hold progress, e.g. "Hold C4. 40 percent complete." */
  readonly progress: (instruction: string, percent: number) => string
}

const METER_LABELS: PitchMeterLabels = {
  flat: 'Flat',
  sharp: 'Sharp',
  youAreSinging: 'You are singing',
  progress: (instruction, percent) => `${instruction}. ${percent} percent complete.`
}

export function PitchMeter(props: PitchMeterProps): React.JSX.Element {
  const theme = useNativeTheme()
  const L = { ...METER_LABELS, ...props.labels }
  const x = props.cents === null ? 50 : Math.max(6, Math.min(94, 50 + props.cents * 0.8))
  return (
    <View style={s.meterWrap}>
      <View style={s.meterLabels}><Text style={[s.meterEdge, { color: theme.dim }]}>{L.flat.toLocaleUpperCase()}</Text><Text style={[s.meterCenterLabel, { color: theme.accent }]}>±{props.pitchWindowCents}¢</Text><Text style={[s.meterEdge, { color: theme.dim }]}>{L.sharp.toLocaleUpperCase()}</Text></View>
      <GlassSurface radius={30} elevation="none" accessibilityLabel={props.accessibilityReading ?? props.reading} style={s.meter}>
        <View style={[s.targetZone, { left: `${50 - props.pitchWindowCents * 0.8}%`, width: `${props.pitchWindowCents * 1.6}%`, backgroundColor: theme.accentSoft }]} />
        <View style={[s.meterCenter, { backgroundColor: theme.accent }]} />
        {props.cents !== null && <View style={[s.pitchMarker, { left: `${x}%`, backgroundColor: props.centered ? theme.accent : theme.text, borderColor: props.centered ? theme.text : theme.lineStrong, shadowColor: theme.accent }]} />}
      </GlassSurface>
      <View style={s.livePitchRow}><View><Text style={[s.livePitchLabel, { color: theme.dim }]}>{L.youAreSinging.toLocaleUpperCase()}</Text><Text accessibilityLiveRegion="polite" style={[s.livePitchNote, { color: theme.text }]}>{props.detectedNote ?? '—'}</Text></View><Text style={[s.meterReading, { color: theme.text }]}>{props.reading}</Text></View>
      <View accessibilityLabel={L.progress(props.instruction, Math.round(props.progress * 100))} style={[s.progressTrack, { backgroundColor: theme.line }]}><View style={[s.progressFill, { width: `${Math.round(Math.max(0, Math.min(1, props.progress)) * 100)}%`, backgroundColor: theme.accent }]} /></View>
      <Text accessibilityLiveRegion="polite" style={[s.meterInstruction, { color: props.centered ? theme.accent : theme.dim }]}>{props.instruction}</Text>
      <Text style={[s.meterHint, { color: theme.dim }]}>{props.hint}</Text>
    </View>
  )
}

export interface TransportItem { readonly accessibilityLabel: string; readonly caption: string; readonly icon: ReactNode; readonly onPress?: () => void }

export function TransportDock({ left, center, right, hint }: { readonly left?: TransportItem; readonly center: TransportItem; readonly right?: TransportItem; readonly hint: string }): React.JSX.Element {
  const theme = useNativeTheme()
  const side = (item?: TransportItem): React.JSX.Element => <View style={s.transportSlot}>{item ? <><RoundAction label={item.accessibilityLabel} icon={item.icon} onPress={item.onPress ?? (() => undefined)} /><Text style={[s.transportLabel, { color: theme.dim }]}>{item.caption}</Text></> : <View style={s.transportSpacer} />}</View>
  return (
    <GlassSurface radius={36} style={s.transportDock}>
      <View style={s.transportRow}>
        {side(left)}
        <View style={s.transportMainSlot}><View accessibilityLabel={center.accessibilityLabel} style={[s.transportMain, { backgroundColor: theme.accent, shadowColor: theme.accent }]}>{center.icon}</View><Text style={[s.transportLabel, { color: theme.dim }]}>{center.caption}</Text></View>
        {side(right)}
      </View>
      <Text style={[s.transportHint, { color: theme.faint }]}>{hint}</Text>
    </GlassSurface>
  )
}

const s = StyleSheet.create({
  pressed: { opacity: 0.65 },
  disabled: { opacity: 0.35 },
  header: { minHeight: 62, flexDirection: 'row', marginBottom: 4, paddingHorizontal: 10, alignItems: 'center' },
  headerAction: { width: 48, minHeight: 48, alignItems: 'center', justifyContent: 'center', zIndex: 20, elevation: 4 },
  backChevron: { fontSize: 30, lineHeight: 32, fontWeight: '600', marginTop: -4 },
  headerTitle: { flex: 1, fontSize: 20, fontWeight: '900', letterSpacing: -0.4, textAlign: 'center' },
  headerTrailing: { width: 48, minHeight: 48, alignItems: 'center', justifyContent: 'center' },
  chip: { minHeight: 42, minWidth: 42, paddingHorizontal: 13, borderRadius: 15, borderCurve: 'continuous', borderWidth: 1, alignItems: 'center', justifyContent: 'center' },
  chipText: { fontSize: 13, fontWeight: '700', textTransform: 'capitalize' },
  primary: { minHeight: 54, borderRadius: 18, paddingHorizontal: 22, flexDirection: 'row', gap: 9, alignItems: 'center', justifyContent: 'center' },
  primaryText: { fontSize: 16, fontWeight: '900' },
  settingsCard: { paddingHorizontal: 16, paddingVertical: 3 },
  hairline: { height: StyleSheet.hairlineWidth },
  settingsRow: { minHeight: 62, flexDirection: 'row', alignItems: 'center', justifyContent: 'space-between', gap: 16 },
  settingsLabel: { fontSize: 13, fontWeight: '700' },
  settingsValueRow: { flex: 1, flexDirection: 'row', alignItems: 'center', justifyContent: 'flex-end', gap: 9 },
  settingsValue: { flexShrink: 1, fontSize: 16, fontWeight: '900' },
  settingsChevron: { fontSize: 26, lineHeight: 28 },
  stickyFooter: { position: 'absolute', left: 0, right: 0, bottom: 0, paddingHorizontal: 20, paddingTop: 14, paddingBottom: 12, borderTopWidth: StyleSheet.hairlineWidth, shadowOpacity: 0.55, shadowRadius: 18, shadowOffset: { width: 0, height: -8 } },
  tabBar: { minHeight: 64, flexDirection: 'row', paddingVertical: 8, paddingHorizontal: 12, gap: 12 },
  tab: { flex: 1, minHeight: 48, borderRadius: 24, borderCurve: 'continuous', alignItems: 'center', justifyContent: 'center', gap: 2 },
  tabLabel: { fontSize: 11, fontWeight: '800', letterSpacing: 0.4 },
  featureCard: { minHeight: 112, borderRadius: 24, borderCurve: 'continuous', paddingVertical: 17, paddingLeft: 17, paddingRight: 78, justifyContent: 'center' },
  featureGlyphRail: { position: 'absolute', top: 0, right: 14, bottom: 0, width: 54, alignItems: 'center', justifyContent: 'center' },
  featureGlyph: { width: 42, height: 42, alignItems: 'center', justifyContent: 'center' },
  featureCopy: { width: '100%' },
  featureTitle: { fontSize: 18, fontWeight: '800' },
  featureDescription: { fontSize: 13, lineHeight: 19, marginTop: 3 },
  listEntry: { minHeight: 80, borderRadius: 22, borderCurve: 'continuous', paddingVertical: 16, paddingLeft: 16, paddingRight: 12, flexDirection: 'row', alignItems: 'center', justifyContent: 'space-between', gap: 12 },
  listEntryCopy: { flex: 1, minWidth: 0 },
  listChevron: { fontSize: 32 },
  referencePanel: { padding: 16, gap: 12 },
  referenceHeader: { minHeight: 54, flexDirection: 'row', alignItems: 'center', justifyContent: 'space-between', gap: 12 },
  utilityLabel: { fontSize: 12, lineHeight: 16, fontWeight: '900', letterSpacing: 1.2 },
  referenceValue: { fontSize: 28, lineHeight: 34, fontWeight: '900', letterSpacing: -0.7 },
  referenceTest: { minWidth: 132, minHeight: 52, borderRadius: 18, paddingHorizontal: 16, flexDirection: 'row', alignItems: 'center', justifyContent: 'center', gap: 10 },
  referenceTestText: { fontSize: 15, lineHeight: 20, fontWeight: '900' },
  referenceVolumeRow: { minHeight: 48, flexDirection: 'row', alignItems: 'center', gap: 12 },
  referenceStepText: { fontSize: 28, lineHeight: 31, fontWeight: '700' },
  referenceTrack: { flex: 1, height: 12, borderRadius: 6, overflow: 'hidden' },
  referenceFill: { height: '100%', borderRadius: 6 },
  referenceHint: { fontSize: 12, lineHeight: 17, textAlign: 'center' },
  pitchWindow: { gap: 12 },
  pitchWindowValue: { fontSize: 24, lineHeight: 30, fontWeight: '900', letterSpacing: -0.5 },
  choiceRow: { flexDirection: 'row', flexWrap: 'wrap', gap: 8 },
  targetArea: { width: '100%', height: 222, alignItems: 'center', justifyContent: 'flex-start', paddingTop: 18 },
  targetEyebrow: { fontSize: 11, fontWeight: '900', letterSpacing: 2 },
  targetLockup: { position: 'relative', width: '100%', height: 181, alignItems: 'center', justifyContent: 'center' },
  targetPitch: { position: 'absolute', left: 0, right: 0, fontSize: 166, lineHeight: 181, fontWeight: '900', letterSpacing: -8, textAlign: 'center' },
  targetOctave: { position: 'absolute', left: '50%', top: 25, fontSize: 52, lineHeight: 60, fontWeight: '900' },
  targetSequence: { position: 'absolute', left: 0, right: 0, bottom: 0, minHeight: 34, flexDirection: 'row', alignItems: 'center', justifyContent: 'center', gap: 7 },
  targetSequenceItem: { minWidth: 54, height: 32, borderRadius: 16, borderWidth: 1, alignItems: 'center', justifyContent: 'center', paddingHorizontal: 9 },
  targetSequenceText: { fontSize: 14, lineHeight: 18, fontWeight: '800' },
  countdown: { flex: 1, width: '100%', minHeight: 156, alignItems: 'center', justifyContent: 'center', paddingBottom: 8 },
  countdownLabel: { fontSize: 13, lineHeight: 18, fontWeight: '900', letterSpacing: 3.2 },
  countdownNumber: { fontSize: 112, lineHeight: 120, fontWeight: '900', fontVariant: ['tabular-nums'], textAlign: 'center', textShadowRadius: 24 },
  countdownHint: { fontSize: 15, lineHeight: 22, fontWeight: '700', textAlign: 'center' },
  meterWrap: { flex: 1, width: '100%', justifyContent: 'center', gap: 7, paddingBottom: 4 },
  meterLabels: { flexDirection: 'row', alignItems: 'center', justifyContent: 'space-between', paddingHorizontal: 8 },
  meterEdge: { fontSize: 10, fontWeight: '900', letterSpacing: 1.2 },
  meterCenterLabel: { fontSize: 10, fontWeight: '900', letterSpacing: 1.2 },
  meter: { width: '100%', height: 112, overflow: 'hidden' },
  targetZone: { position: 'absolute', top: 0, bottom: 0 },
  meterCenter: { position: 'absolute', left: '50%', top: 0, bottom: 0, width: 2 },
  pitchMarker: { position: 'absolute', top: 41, width: 44, height: 44, marginLeft: -22, borderRadius: 22, borderWidth: 4, shadowOpacity: 0.75, shadowRadius: 14 },
  livePitchRow: { minHeight: 58, flexDirection: 'row', alignItems: 'center', justifyContent: 'space-between', paddingHorizontal: 10 },
  livePitchLabel: { fontSize: 9, lineHeight: 12, fontWeight: '900', letterSpacing: 1.4 },
  livePitchNote: { fontSize: 38, lineHeight: 42, fontWeight: '900', letterSpacing: -1 },
  meterReading: { flex: 1, fontSize: 17, lineHeight: 24, fontWeight: '900', textAlign: 'right', marginLeft: 16 },
  progressTrack: { width: '100%', height: 8, borderRadius: 4, overflow: 'hidden' },
  progressFill: { height: '100%', borderRadius: 4 },
  meterInstruction: { fontSize: 18, lineHeight: 24, fontWeight: '900', textAlign: 'center', paddingHorizontal: 12 },
  meterHint: { fontSize: 12, lineHeight: 17, textAlign: 'center', paddingHorizontal: 12 },
  transportDock: { width: '100%', minHeight: 126, marginTop: 8, paddingHorizontal: 18, paddingTop: 11, paddingBottom: 9, justifyContent: 'center' },
  transportRow: { minHeight: 76, flexDirection: 'row', alignItems: 'center', justifyContent: 'space-between' },
  transportSlot: { width: 76, alignItems: 'center', justifyContent: 'center', gap: 3 },
  transportMainSlot: { width: 76, alignItems: 'center', justifyContent: 'center', gap: 3 },
  transportSpacer: { width: 54, height: 54 },
  transportMain: { width: 68, height: 68, borderRadius: 34, alignItems: 'center', justifyContent: 'center', shadowOpacity: 0.24, shadowRadius: 12 },
  transportLabel: { fontSize: 10, lineHeight: 12, fontWeight: '800', letterSpacing: 0.5, textTransform: 'uppercase' },
  transportHint: { fontSize: 10.5, lineHeight: 14, fontWeight: '700', letterSpacing: 0.2, textAlign: 'center' }
})
