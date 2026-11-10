"""Reproduce the pinned Linux folder fallback; never used by consuming apps.
Needs fontTools4.68.0. Fetch the pinned public source separately (hash checked):
https://raw.githubusercontent.com/googlefonts/noto-emoji/e20cbc2bbec1926686be9f9bee7d1d2cfa1fea0e/2D/fonts/NotoColorEmoji.ttf
The complete source font is not distributed; its original OFL license is.
"""
from pathlib import Path
import argparse, hashlib, io, json, os, tempfile
from fontTools import subset
from fontTools.ttLib import TTFont

ROOT = Path(__file__).resolve().parents[1]
ASSETS = ROOT / 'gpui/assets/fonts'
SOURCE_SHA = '15671215ab769fdc7162a045d56fd7d7e477c51b04e6b3c761d914d8fdd6cc44'
OUTPUT_SHA = '99e7c6a96c62de5cab386ee2da3812da1e24575224c6bdfb7952d48cba7aa7f1'
LICENSE_SHA = '500bb1ccf43df7bbb522112f9133a52b16e1c35e809632f5d8609b179152de5b'


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source', type=Path, required=True)
    args = parser.parse_args()
    source = args.source.read_bytes()
    if hashlib.sha256(source).hexdigest() != SOURCE_SHA:
        raise RuntimeError('Pinned upstream Noto Color Emoji bytes changed')
    if hashlib.sha256((ASSETS / 'NotoColorEmoji-OFL.txt').read_bytes()).hexdigest() != LICENSE_SHA:
        raise RuntimeError('Pinned upstream OFL license changed')
    font = TTFont(io.BytesIO(source), recalcTimestamp=False)
    options = subset.Options()
    options.name_IDs = ['*']
    options.name_legacy = True
    options.name_languages = ['*']
    options.glyph_names = True
    options.notdef_glyph = True
    options.notdef_outline = True
    options.recalc_timestamp = False
    subsetter = subset.Subsetter(options=options)
    subsetter.populate(unicodes=[0x1F4C1, 0xFE0F])
    subsetter.subset(font)
    output = io.BytesIO()
    font.save(output, reorderTables=True)
    data = output.getvalue()
    if hashlib.sha256(data).hexdigest() != OUTPUT_SHA:
        raise RuntimeError('Subset differs from the reviewed asset; use fontTools4.68.0')
    target = ASSETS / 'NotoColorEmojiFolder.ttf'
    with tempfile.NamedTemporaryFile(dir=ASSETS, prefix='.folder-font-', delete=False) as pending:
        path = Path(pending.name)
        pending.write(data)
    try:
        os.replace(path, target)
    finally:
        path.unlink(missing_ok=True)
    print(f'Regenerated {len(data)}-byte pinned folder subset; no body fonts changed')


if __name__ == '__main__':
    main()
