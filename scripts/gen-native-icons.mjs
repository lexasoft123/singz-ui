/** Regenerate transparent native icons from the canonical vector artwork. */
import { writeFile } from 'node:fs/promises'
import { chromium } from 'playwright-core'
import { iconArtwork, iconSvg } from '../dist/icons/artwork.js'

const browser = await chromium.launch({
  // Set ICON_BROWSER_PATH for a locally installed Chrome; otherwise use
  // Playwright's managed Chromium (npx playwright install chromium).
  ...(process.env.ICON_BROWSER_PATH ? { executablePath: process.env.ICON_BROWSER_PATH } : {}),
  headless: true
})
try {
  const page = await browser.newPage({ viewport: { width: 112, height: 112 }, deviceScaleFactor: 1 })
  const images = {}
  for (const name of Object.keys(iconArtwork)) {
    await page.setContent('<style>body{margin:0;background:transparent}</style>' + iconSvg(name, 'white'))
    images[name] = 'data:image/png;base64,' + (await page.screenshot({ omitBackground: true })).toString('base64')
  }
  await writeFile(new URL('../src/icons/native-images.ts', import.meta.url),
    '/** Generated 112px transparent renditions of artwork.ts. */\nexport const iconImages = ' + JSON.stringify(images, null, 2) + ' as const\n')
} finally {
  await browser.close()
}
