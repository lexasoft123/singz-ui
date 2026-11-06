"""Generate UIKit's named static faces; outlines are independent of native style matching.
Requires fonttools[woff] in the generation environment, not in consuming apps.
"""
from pathlib import Path
import hashlib,json
from fontTools.ttLib import TTFont
from fontTools.varLib.instancer import instantiateVariableFont
ROOT=Path(__file__).resolve().parents[1]
ASSETS=ROOT/'gpui/assets/fonts'
faces=[]
for source,family,file,weights in [
 ('Bricolage','SingZ Display','bricolage-grotesque-latin-wght-normal.woff2',[400,500,600,650,700,800]),
 ('MartianMono','SingZ Mono','martian-mono-latin-wght-normal.woff2',[400,500,700])]:
 for weight in weights:
  path=ASSETS/'sources'/file
  font=TTFont(path);font.flavor=None;font.recalcTimestamp=False
  axes={a.axisTag:a.defaultValue for a in font['fvar'].axes};axes['wght']=weight
  font=instantiateVariableFont(font,axes,inplace=True)
  family_name=f'{family} {weight}'
  style={400:'Regular',500:'Medium',600:'SemiBold',650:'DemiBold',700:'Bold',800:'ExtraBold'}[weight]
  names=font['name']
  for ident,value in [(1,family_name),(2,style),(4,family_name+' '+style),(6,family.replace(' ','')+'-'+style),(16,family_name),(17,style)]:
   for platform,encoding,lang in [(3,1,0x409),(1,0,0)]:names.setName(value,ident,platform,encoding,lang)
  output=ASSETS/f'{source}{weight}.ttf';font.save(output)
  # DirectWrite FontSet filters requested style properties exactly. The UIKit
  # API selects each weight by its explicit family and normal style. Normalize
  # only Windows metadata; keep CoreText metadata and fallback behavior intact.
  windows=TTFont(output);windows.recalcTimestamp=False
  windows['OS/2'].usWeightClass=400
  windows['OS/2'].fsSelection=(windows['OS/2'].fsSelection & ~(1|32))|64
  windows['head'].macStyle &= ~3
  if 'STAT' in windows:del windows['STAT']
  for ident,value in [(2,'Regular'),(4,family_name+' Regular'),(6,family.replace(' ','')+str(weight)+'-Regular'),(17,'Regular')]:
   for platform,encoding,lang in [(3,1,0x409),(1,0,0)]:windows['name'].setName(value,ident,platform,encoding,lang)
  windows_path=ASSETS/'windows'/output.name;windows_path.parent.mkdir(exist_ok=True);windows.save(windows_path)
  outlines=hashlib.sha256()
  for tag in ['glyf','hmtx','cmap','GPOS','GSUB']:
   if tag in font:outlines.update(tag.encode());outlines.update(font.getTableData(tag))
  faces.append({'file':output.name,'family':family_name,'visualWeight':weight,'nativeWeight':weight,'windowsFile':'windows/'+output.name,'windowsNativeWeight':400,'windowsSha256':hashlib.sha256(windows_path.read_bytes()).hexdigest(),'style':'normal','source':f'sources/{file}','sourceSha256':hashlib.sha256(path.read_bytes()).hexdigest(),'sha256':hashlib.sha256(output.read_bytes()).hexdigest(),'outlineAndLayoutSha256':outlines.hexdigest()})
(ASSETS/'manifest.json').write_text(json.dumps({'schema':1,'purpose':'UIKit-owned static font families; visual weight encoded by family; original CoreText metadata and Windows normal-style aliases','faces':faces},indent=2)+'\n')
print(f'Generated {len(faces)} UIKit font faces')
