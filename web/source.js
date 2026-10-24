const $=id=>document.getElementById(id);
const make=(tag,text)=>{const el=document.createElement(tag);el.textContent=text;return el;};
const fields=[['on_time','On-time arrivals'],['delayed','Delayed arrivals (15+ minutes)'],['cancelled','Cancelled flights'],['diverted','Diverted flights'],['flight_operations','Flight operations'],['on_time_percent_hundredths','Published on-time rate'],['delayed_percent_hundredths','Published delayed rate'],['cancelled_percent_hundredths','Published cancelled rate']];
let worker,ready=false,revision=0,timer,canonical;
function state(text){$('source-status').textContent=text;}
function invalidate(text){revision++;clearTimeout(timer);canonical=null;$('source-results').hidden=true;$('source-download').disabled=true;if(text)state(text);}
function stop(text){ready=false;worker?.terminate();invalidate(text);for(const id of ['source-from','source-to','source-compare','source-reset','source-pandemic'])$(id).disabled=true;}
function controls(from,to){$('source-from').value=String(from);$('source-to').value=String(to);$('source-input').value=JSON.stringify({view:'bts-arrival-comparison',from_year:from,to_year:to},null,2);}
function compare(){
 if(!ready)return;invalidate('Comparing frozen source locally in Rust WebAssembly…');
 const input=$('source-input').value;
 if(new TextEncoder().encode(input).length>65536){state('Query exceeds 64 KiB. Reset or shorten the query.');return;}
 const id=revision;worker.postMessage({type:'evaluate',id,input});timer=setTimeout(()=>stop('Source comparison timed out. Reload to retry.'),10000);
}
function render(result){
 $('source-from').value=String(result.from.year);$('source-to').value=String(result.to.year);
 $('source-context').textContent=`January–August ${result.from.year} → January–August ${result.to.year}. All Airports · All Carriers · Arrivals. National reporting-carrier aggregate.`;
 const format=(field,value,change=false)=>field.endsWith('_hundredths')?`${value>0&&change?'+':''}${(value/100).toFixed(2)}${change?' pp':'%'}`:`${value>0&&change?'+':''}${new Intl.NumberFormat('en-US').format(value)}`;
 $('source-rows').replaceChildren(...fields.map(([field,label])=>{const row=document.createElement('tr'),th=make('th',label);th.scope='row';row.append(th,make('td',format(field,result.from[field])),make('td',format(field,result.to[field])),make('td',format(field,result.changes[field],true)));return row;}));
 $('source-from-heading').textContent=String(result.from.year);$('source-to-heading').textContent=String(result.to.year);
 $('source-notes').textContent=result.source.notes.join('\n');
 $('source-custody').textContent=`Captured ${result.source.captured_at}. Original SHA-256: ${result.source.raw_sha256}`;
 $('source-unknowns').replaceChildren(...Object.keys(result.not_assessed).map(key=>make('li',`${key.replaceAll('_',' ')}: not assessed (null)`)));
 $('source-results').hidden=false;$('source-download').disabled=false;state('Source comparison ready. Counts reconcile in both years.');
}
for(const id of ['source-from','source-to'])for(let year=2017;year<=2026;year++){const option=make('option',year);option.value=year;$(id).append(option);}
for(const id of ['source-from','source-to'])$(id).addEventListener('change',()=>{controls(Number($('source-from').value),Number($('source-to').value));compare();});
$('source-input').addEventListener('input',()=>invalidate('Query changed. Compare to refresh.'));
$('source-compare').addEventListener('click',compare);
$('source-reset').addEventListener('click',()=>{controls(2024,2026);compare();});
$('source-pandemic').addEventListener('click',()=>{controls(2019,2020);compare();});
$('source-download').addEventListener('click',()=>{if(!canonical)return;const url=URL.createObjectURL(new Blob([canonical],{type:'application/json'})),a=document.createElement('a');a.href=url;a.download='tarmac-bts-comparison.json';a.click();setTimeout(()=>URL.revokeObjectURL(url),1000);});
controls(2024,2026);
try{
 worker=new Worker(new URL('./worker.js',import.meta.url),{type:'module'});
 timer=setTimeout(()=>stop('Unable to load source Rust WebAssembly. Reload to retry.'),15000);
 worker.onerror=()=>stop('Unable to load source Rust WebAssembly. Reload to retry.');
 worker.onmessage=({data})=>{
  if(!ready&&data.type==='error'){stop(`Unable to load source Rust WebAssembly: ${data.error}`);return;}
  if(data.type==='ready'){clearTimeout(timer);ready=true;for(const id of ['source-from','source-to','source-compare','source-reset','source-pandemic'])$(id).disabled=false;compare();return;}
  if(data.id!==revision)return;clearTimeout(timer);
  if(!data.ok){invalidate(`Cannot compare source: ${data.error}`);return;}
  canonical=data.canonical;render(data.result);
 };
 worker.postMessage({type:'init'});
}catch{stop('Unable to load source Rust WebAssembly. Reload to retry.');}
