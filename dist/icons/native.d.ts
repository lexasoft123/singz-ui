import React from 'react';
import { type ImageStyle, type StyleProp } from 'react-native';
import type { IconName } from './artwork.js';
export type { IconName } from './artwork.js';
export interface IconProps {
    name: IconName;
    size?: number;
    color?: string;
    label?: string;
    style?: StyleProp<ImageStyle>;
}
/** 4× rasterization of the same vector artwork: no extra native module or animation loop. */
export declare function Icon({ name, size, color, label, style }: IconProps): React.JSX.Element;
