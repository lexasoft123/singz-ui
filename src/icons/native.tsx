import React from 'react'
import { Image, type ImageStyle, type StyleProp } from 'react-native'
import { iconImages } from './native-images.js'
import type { IconName } from './artwork.js'
export type { IconName } from './artwork.js'
export interface IconProps { name: IconName; size?: number; color?: string; label?: string; style?: StyleProp<ImageStyle> }
/** 4× rasterization of the same vector artwork: no extra native module or animation loop. */
export function Icon({ name, size = 24, color = '#ffa21c', label, style }: IconProps): React.JSX.Element {
 return <Image source={{ uri: iconImages[name] }} style={[{ width: size, height: size, tintColor: color }, style]} accessible={!!label} accessibilityRole={label ? 'image' : undefined} accessibilityLabel={label} />
}
