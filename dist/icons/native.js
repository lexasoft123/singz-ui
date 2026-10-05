import { jsx as _jsx } from "react/jsx-runtime";
import React from 'react';
import { Image } from 'react-native';
import { iconImages } from './native-images.js';
/** 4× rasterization of the same vector artwork: no extra native module or animation loop. */
export function Icon({ name, size = 24, color = '#ffa21c', label, style }) {
    return _jsx(Image, { source: { uri: iconImages[name] }, style: [{ width: size, height: size, tintColor: color }, style], accessible: !!label, accessibilityRole: label ? 'image' : undefined, accessibilityLabel: label });
}
