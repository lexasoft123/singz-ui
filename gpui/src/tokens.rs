//! Native counterparts of `src/tokens/tokens.ts`. RGB values use `rgb`,
//! while translucent values use RGBA ordering with `rgba`.
pub const BG: u32 = 0x12100d;
pub const PANEL: u32 = 0x1b1814;
pub const PANEL_DEEP: u32 = 0x0f0d0a;
pub const TEXT: u32 = 0xf4efe6;
pub const DIM: u32 = 0x9b917e;
pub const FAINT: u32 = 0x6b6355;
pub const ACCENT: u32 = 0xffa028;
pub const ACCENT_DEEP: u32 = 0xff8a1f;
pub const ACCENT_LIFT: u32 = 0xffbe58;
pub const ACCENT_INK: u32 = 0x241705;
pub const SURFACE_RAISED: u32 = 0x1e1a15;
pub const DANGER: u32 = 0xff7a5c;
pub const SUCCESS: u32 = 0x58d68a;
pub const LINE: u32 = 0xfff0d614;
pub const LINE_STRONG: u32 = 0xfff0d633;
pub const ACCENT_SOFT: u32 = 0xffa02821;
pub const ACCENT_LINE: u32 = 0xffa0288c;
pub const CONTROL_FILL: u32 = 0x18141175;
pub const CONTROL_LINE: u32 = 0xfff0dc1a;
pub const CONTROL_RIM: u32 = 0xfff0dc2e;
pub const GLASS_FILL: u32 = 0x1814118c;
pub const GLASS_LINE: u32 = 0xfff0dc0d;
pub const GLASS_RIM: u32 = 0xfff0dc24;
pub const FOOTER_FILL: u32 = 0x0c0a08f5;
pub const FOCUS_RING: u32 = 0xffa028b3;

pub const DISPLAY_400: &str = "SingZ Display 400";
pub const DISPLAY_500: &str = "SingZ Display 500";
pub const DISPLAY_600: &str = "SingZ Display 600";
pub const DISPLAY_650: &str = "SingZ Display 650";
pub const DISPLAY_700: &str = "SingZ Display 700";
pub const DISPLAY_800: &str = "SingZ Display 800";
pub const MONO_400: &str = "SingZ Mono 400";
pub const MONO_500: &str = "SingZ Mono 500";
pub const MONO_700: &str = "SingZ Mono 700";

/// Stable mixer ids, display names and colours from `src/tokens/stems.ts`.
pub const STEMS: [(&str, &str, u32); 7] = [
    ("original", "Full mix", 0xbfb49d),
    ("vocals", "Vocals", 0xff5c65),
    ("drums", "Drums", 0xffc53d),
    ("bass", "Bass", 0x527dff),
    ("guitar", "Guitar", 0xf98424),
    ("piano", "Piano", 0xda81da),
    ("other", "Instruments", 0x27e7bb),
];
pub const CUSTOM_COLORS: [u32; 5] = [0xc7e06a, 0xff9ad5, 0x6fd8ff, 0xe8dcc0, 0xa98cff];
