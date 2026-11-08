"""Publish UIKit font assets without a JavaScript runtime."""
from pathlib import Path
import json, shutil
root=Path(__file__).resolve().parents[1]
assets=root/'gpui/assets/fonts';target=root/'dist/fonts';target.mkdir(parents=True,exist_ok=True)
manifest=json.loads((assets/'manifest.json').read_text())
for name in ['Bricolage-OFL.txt','MartianMono-OFL.txt',*[f['file'] for f in manifest['faces']]]:shutil.copyfile(assets/name,target/name)
for face in manifest['faces']:
    output=target/face['windowsFile'];output.parent.mkdir(exist_ok=True);shutil.copyfile(assets/face['windowsFile'],output)
for path in (assets/'sources').glob('*.woff2'):shutil.copyfile(path,target/path.name)
(target/'manifest.json').write_text(json.dumps({**manifest,'faces':[{**f,'source':Path(f['source']).name} for f in manifest['faces']]},indent=2)+'\n')
variable='\n'.join((assets/'sources'/f'{name}-variable.css').read_text().replace('./files/','./fonts/') for name in ['Bricolage','MartianMono'])
static='\n'.join("@font-face { font-family: '%s'; font-style: normal; font-weight: 400; font-display: swap; src: url('./fonts/%s') format('truetype'); }"%(f['family'],f['file']) for f in manifest['faces'])
(root/'dist/fonts.css').write_text('/* GENERATED from canonical UIKit font assets. */\n'+variable+'\n'+static+'\n')
print(f"Distributed {len(manifest['faces'])} native faces, variable subsets and licenses")
