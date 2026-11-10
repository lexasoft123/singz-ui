"""Verify the owned folder asset, safe reproduction and no-Node distribution.
Run with the same fontTools environment used for generation:
  python scripts/verify-folder-fonts.py --source /path/to/pinned/NotoColorEmoji.ttf
This never installs fonts, runs Node, generates body fonts, or launches an app.
"""
from pathlib import Path
import argparse
import hashlib
import importlib.util
import io
import json
import shutil
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

from fontTools.ttLib import TTFont

ROOT = Path(__file__).resolve().parents[1]
SOURCE = None
sys.dont_write_bytecode = True


def digest(data):
    return hashlib.sha256(data).hexdigest()


class FolderFontTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        if SOURCE is None:
            raise RuntimeError('Pass --source with the pinned upstream font')
        cls.source = SOURCE.read_bytes()
        cls.asset_dir = ROOT / 'gpui/assets/fonts'
        cls.metadata = json.loads((cls.asset_dir / 'NotoColorEmojiFolder.json').read_text())
        if digest(cls.source) != cls.metadata['sourceSha256']:
            raise RuntimeError('Upstream source does not match the reviewed metadata')

    def isolated(self):
        temporary = tempfile.TemporaryDirectory(prefix='singz-uikit-folder-font-')
        self.addCleanup(temporary.cleanup)
        root = Path(temporary.name)
        shutil.copytree(self.asset_dir, root / 'gpui/assets/fonts')
        (root / 'scripts').mkdir()
        for name in ('subset-folder-emoji.py', 'dist-fonts.py'):
            shutil.copyfile(ROOT / 'scripts' / name, root / 'scripts' / name)
        return root

    def generator(self, root):
        spec = importlib.util.spec_from_file_location('singz_folder_generator', root / 'scripts/subset-folder-emoji.py')
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        return module

    def test_owned_subset_preserves_pinned_bitmap_metrics_names_and_coverage(self):
        data = (self.asset_dir / self.metadata['file']).read_bytes()
        self.assertEqual(len(data), self.metadata['bytes'])
        self.assertEqual(digest(data), self.metadata['sha256'])
        self.assertEqual(digest((self.asset_dir / self.metadata['licenseFile']).read_bytes()), self.metadata['licenseSha256'])
        self.assertEqual(self.metadata['platforms'], ['linux'])
        font = TTFont(io.BytesIO(data), recalcTimestamp=False)
        original = TTFont(io.BytesIO(self.source), recalcTimestamp=False)
        self.assertEqual(set(font.getBestCmap()), {0x1F4C1})
        self.assertEqual(font.getGlyphOrder(), ['.notdef', 'u1F4C1'])
        self.assertEqual(font['name'].getDebugName(1), self.metadata['family'])
        self.assertEqual(font['name'].getDebugName(6), 'NotoColorEmoji')
        self.assertTrue({'CBDT', 'CBLC'}.issubset(font.keys()))
        glyph = font.getBestCmap()[0x1F4C1]
        original_glyph = original.getBestCmap()[0x1F4C1]
        image = font['CBDT'].strikeData[0][glyph].imageData
        self.assertEqual(image, original['CBDT'].strikeData[0][original_glyph].imageData)
        self.assertEqual(digest(image), self.metadata['folderBitmapSha256'])
        self.assertEqual(font['hmtx'][glyph], original['hmtx'][original_glyph])
        self.assertEqual(font['head'].unitsPerEm, original['head'].unitsPerEm)

    def test_real_reproduction_is_byte_identical(self):
        root = self.isolated()
        target = root / 'gpui/assets/fonts' / self.metadata['file']
        target.write_bytes(b'Previous asset must be replaced only by reviewed bytes')
        module = self.generator(root)
        with patch.object(sys, 'argv', ['subset-folder-emoji.py', '--source', str(SOURCE)]):
            module.main()
        self.assertEqual(target.read_bytes(), (self.asset_dir / self.metadata['file']).read_bytes())
        self.assertFalse(list(target.parent.glob('.folder-font-*')))

    def test_unrecognized_source_preserves_installed_asset(self):
        root = self.isolated()
        target = root / 'gpui/assets/fonts' / self.metadata['file']
        before = target.read_bytes()
        wrong = root / 'wrong-source.ttf'
        wrong.write_bytes(b'Foreign font')
        module = self.generator(root)
        with patch.object(sys, 'argv', ['subset-folder-emoji.py', '--source', str(wrong)]):
            with self.assertRaisesRegex(RuntimeError, 'upstream.*changed'):
                module.main()
        self.assertEqual(target.read_bytes(), before)

    def test_output_pin_failure_preserves_installed_asset(self):
        root = self.isolated()
        target = root / 'gpui/assets/fonts' / self.metadata['file']
        before = target.read_bytes()
        module = self.generator(root)
        # Fault-inject only the expected output pin in this disposable module:
        # real fontTools output must not publish when qualification disagrees.
        with patch.object(module, 'OUTPUT_SHA', '0' * 64):
            with patch.object(sys, 'argv', ['subset-folder-emoji.py', '--source', str(SOURCE)]):
                with self.assertRaisesRegex(RuntimeError, 'Subset differs'):
                    module.main()
        self.assertEqual(target.read_bytes(), before)
        self.assertFalse(list(target.parent.glob('.folder-font-*')))

    def test_license_failure_preserves_installed_asset(self):
        root = self.isolated()
        target = root / 'gpui/assets/fonts' / self.metadata['file']
        before = target.read_bytes()
        (target.parent / self.metadata['licenseFile']).write_text('Unreviewed replacement')
        module = self.generator(root)
        with patch.object(sys, 'argv', ['subset-folder-emoji.py', '--source', str(SOURCE)]):
            with self.assertRaisesRegex(RuntimeError, 'OFL license changed'):
                module.main()
        self.assertEqual(target.read_bytes(), before)

    def test_python_distribution_preserves_body_faces_and_ships_owned_fallback(self):
        root = self.isolated()
        assets = root / 'gpui/assets/fonts'
        before = {str(p.relative_to(assets)): digest(p.read_bytes()) for p in assets.rglob('*') if p.is_file()}
        manifest = json.loads((assets / 'manifest.json').read_text())
        self.assertEqual(manifest['fallbacks'], [self.metadata])
        for face in manifest['faces']:
            self.assertEqual(digest((assets / face['file']).read_bytes()), face['sha256'])
            self.assertEqual(digest((assets / face['windowsFile']).read_bytes()), face['windowsSha256'])
        result = subprocess.run([sys.executable, str(root / 'scripts/dist-fonts.py')], capture_output=True, text=True)
        self.assertEqual(result.returncode, 0, result.stderr)
        target = root / 'dist/fonts'
        distributed = json.loads((target / 'manifest.json').read_text())
        self.assertEqual(distributed['faces'], [{**face, 'source': Path(face['source']).name} for face in manifest['faces']])
        self.assertEqual(distributed['fallbacks'], [self.metadata])
        for name in (self.metadata['file'], self.metadata['licenseFile'], self.metadata['metadataFile']):
            self.assertEqual((target / name).read_bytes(), (assets / name).read_bytes())
        for face in manifest['faces']:
            for name in (face['file'], face['windowsFile']):
                self.assertEqual((target / name).read_bytes(), (assets / name).read_bytes())
        after = {str(p.relative_to(assets)): digest(p.read_bytes()) for p in assets.rglob('*') if p.is_file()}
        self.assertEqual(before, after)
        self.assertNotIn('Noto Color Emoji', (root / 'dist/fonts.css').read_text())


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source', type=Path, required=True)
    args = parser.parse_args()
    SOURCE = args.source.resolve()
    unittest.main(argv=[sys.argv[0]], verbosity=2)
