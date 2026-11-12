import React from 'react';
export interface PeakMeterProps {
    /** Live level in dBFS; `-Infinity` is silence. */
    db: number;
    peakDb?: number;
    /** 16 bricks / 9 px instead of 40 bricks / 16 px. */
    mini?: boolean;
    clipped?: boolean;
    /** Changing it (a new song, a new input) clears the meter instantly. */
    identity?: object;
    enabled?: boolean;
    testID?: string;
}
/** A tiny Skia surface keeps per-frame ballistics off the JS tree. Needs the
 *  optional peers @shopify/react-native-skia, react-native-reanimated and
 *  react-native-worklets. */
export declare function PeakMeter({ db, peakDb, mini, clipped, identity, enabled, testID }: PeakMeterProps): React.JSX.Element;
