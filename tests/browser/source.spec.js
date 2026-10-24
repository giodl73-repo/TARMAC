import {test,expect} from '@playwright/test';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
async function ready(page){await page.goto('/TARMAC/');await expect(page.locator('#source-status')).toContainText('Source comparison ready');await expect(page.locator('#status')).toContainText('Comparison ready');}
async function download(page){const [file]=await Promise.all([page.waitForEvent('download'),page.locator('#source-download').click()]);return JSON.parse(readFileSync(await file.path(),'utf8'));}

test('actual WASM source queries match four native canonical byte strings',async({page})=>{
 await ready(page);const refs=JSON.parse(readFileSync('.cache/bts-native-results.json','utf8'));
 const actual=await page.evaluate(async refs=>new Promise((resolve,reject)=>{const worker=new Worker('./worker.js',{type:'module'}),values=[];const timer=setTimeout(()=>{worker.terminate();reject(Error('parity timeout'));},10000);worker.onerror=()=>{clearTimeout(timer);worker.terminate();reject(Error('worker error'));};worker.onmessage=({data})=>{if(data.type==='ready')worker.postMessage({type:'evaluate',id:0,input:refs[0].input});else if(data.ok){values.push(data.canonical);if(values.length===refs.length){clearTimeout(timer);worker.terminate();resolve(values);}else worker.postMessage({type:'evaluate',id:values.length,input:refs[values.length].input});}else{clearTimeout(timer);worker.terminate();reject(Error(data.error));}};worker.postMessage({type:'init'});}),refs);
 expect(actual).toEqual(refs.map(r=>r.canonical));await expect(page.locator('#source-rows tr')).toHaveCount(8);
});

test('published outcomes, percentage-point changes and downloads retain limits and graph independence',async({page})=>{
 await ready(page);const graph=await page.locator('#metric-rows').textContent();
 const result=await download(page);expect(result.from.flight_operations).toBe(4714957);expect(result.to.flight_operations).toBe(4728732);expect(result.changes.flight_operations).toBe(13775);expect(result.changes.cancelled).toBe(24173);expect(result.changes.on_time_percent_hundredths).toBe(-78);
 await expect(page.locator('#source-rows')).toContainText('-0.78 pp');expect(result.source.window).toBe('January-August');expect(result.comparability.constant_carrier_cohort).toBe(false);expect(result.comparability.annualized).toBe(false);expect(result.comparability.carrier_counts_after_2021).toBeNull();expect(Object.values(result.not_assessed)).toEqual(Array(7).fill(null));expect(Object.values(result.authority).every(v=>v===false)).toBe(true);
 await page.locator('#source-pandemic').click();await expect(page.locator('#source-context')).toContainText('January–August 2019 → January–August 2020');expect((await download(page)).to.cancelled).toBe(271488);expect(await page.locator('#metric-rows').textContent()).toBe(graph);
 await page.locator('#source-reset').click();await expect(page.locator('#source-context')).toContainText('January–August 2024 → January–August 2026');
});

test('year selectors support same-year zero and reverse signed changes',async({page})=>{
 await ready(page);await page.locator('#source-from').selectOption('2026');await expect(page.locator('#source-context')).toContainText('January–August 2026 → January–August 2026');expect(Object.values((await download(page)).changes).every(v=>v===0)).toBe(true);
 await page.locator('#source-to').selectOption('2024');await expect(page.locator('#source-context')).toContainText('January–August 2026 → January–August 2024');expect((await download(page)).changes.cancelled).toBe(-24173);
});

test('invalid source queries clear stale output and recover without changing source rows',async({page})=>{
 await ready(page);await page.locator('#source-editor summary').click();
 for(const query of ['{',JSON.stringify({view:'bts-arrival-comparison',from_year:2016,to_year:2026}),JSON.stringify({view:'bts-arrival-comparison',from_year:2024,to_year:2026,airport:'ATL'}),JSON.stringify({view:'bts-arrival-comparison',from_year:null,to_year:2026}),'x'.repeat(65537)]){
  await page.locator('#source-input').fill(query);await expect(page.locator('#source-results')).toBeHidden();await expect(page.locator('#source-download')).toBeDisabled();await page.locator('#source-compare').click();await expect(page.locator('#source-status')).toContainText(query.length>65536?'exceeds 64 KiB':'Cannot compare source');
 }
 await page.locator('#source-reset').click();await expect(page.locator('#source-status')).toContainText('Source comparison ready');expect((await download(page)).from.flight_operations).toBe(4714957);
});

test('original hash, normalized ten rows, mobile keyboard and source routes work under prefix',async({page})=>{
 const errors=[];page.on('pageerror',e=>errors.push(e.message));await page.setViewportSize({width:390,height:844});await ready(page);
 await page.locator('#source-compare').focus();await page.keyboard.press('Tab');await expect(page.locator('#source-reset')).toBeFocused();await page.keyboard.press('Enter');await expect(page.locator('#source-status')).toContainText('Source comparison ready');expect(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBe(true);
 const custody=await (await page.request.get('/TARMAC/sources/custody.json')).json();const raw=await (await page.request.get('/TARMAC/sources/transtats-arrivals.html')).body();expect(raw.length).toBe(custody.bytes);expect(createHash('sha256').update(raw).digest('hex')).toBe(custody.sha256);
 const summary=await (await page.request.get('/TARMAC/bts-summary.json')).json();expect(summary.rows).toHaveLength(10);expect(summary.filters.CboSelection).toBe('Arrivals');
 for(const file of ['SOURCE-NOTICE.txt','pkg/tarmac_web.wasm','LICENSE','THIRD-PARTY-LICENSES.txt'])expect((await page.request.get('/TARMAC/'+file)).ok()).toBe(true);
 const provenance=await (await page.request.get('/TARMAC/provenance.json')).json();expect(provenance.tasks).toHaveLength(2);if(process.env.CI||process.env.VERIFY_CLEAN_PROVENANCE)expect(provenance.dirty).toBe(false);expect(errors).toEqual([]);
});

test('source worker failure retains readable evidence links and no stale results',async({page})=>{
 await page.route('**/*.wasm',route=>route.abort());await page.goto('/TARMAC/');await expect(page.locator('#source-status')).toContainText('Unable to load source Rust WebAssembly');await expect(page.locator('#source-results')).toBeHidden();await expect(page.locator('#source-download')).toBeDisabled();await expect(page.getByRole('link',{name:'Original HTML capture'})).toBeVisible();
});
