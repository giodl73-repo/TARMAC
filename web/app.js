const $=id=>document.getElementById(id),number=value=>new Intl.NumberFormat('en-US',{maximumFractionDigits:3}).format(value);
function node(tag,text){const el=document.createElement(tag);if(text!==undefined)el.textContent=text;return el;}
function svg(tag,attrs,text){const el=document.createElementNS('http://www.w3.org/2000/svg',tag);for(const [key,value]of Object.entries(attrs))el.setAttribute(key,value);if(text!==undefined)el.textContent=text;return el;}
let worker,ready=false,timer,revision=0,examples,result;
function status(text){$('status').textContent=text;}
function invalidate(text){revision++;clearTimeout(timer);result=null;$('results').hidden=true;$('download').disabled=true;if(text)status(text);}
function fatal(text){ready=false;worker?.terminate();invalidate(text);for(const id of ['compare','reset','outage','selected'])$(id).disabled=true;}
function formatMetric(value){if(value===null)return 'Not observed';return typeof value==='boolean'?(value?'Connected':'Disconnected'):number(value);}
const metrics=[['enabled_incident_routes','Incident routes','routes'],['incident_ops_per_day','Incident route operations','ops/day'],['degree_centrality','Normalized degree (not service quality)',''],['path_connected_to_hub','Graph path to named hub','']];
function render(current,baseline){
 $('node-context').textContent=`Current airport: ${current.selected_node}; hub: ${current.hub}. Reset comparison uses the same airport/hub pair when both exist in the reset graph.`;
 $('metric-rows').replaceChildren(...metrics.map(([key,title,unit])=>{const row=node('tr'),th=node('th',title);th.scope='row';row.append(th,node('td',baseline?`${formatMetric(baseline.metrics[key])} ${unit}`:'Not observed'),node('td',`${formatMetric(current.metrics[key])} ${unit}`));if(baseline&&current.metrics[key]!==baseline.metrics[key])row.lastChild.append(node('small','Changed from reset graph'));return row;}));
 $('unknowns').replaceChildren(...Object.entries(current.not_assessed).map(([key])=>node('li',`${key.replaceAll('_',' ')}: not assessed (null)`)));
 $('selected').replaceChildren(...current.nodes.map(site=>{const option=node('option',site.name);option.value=site.id;option.selected=site.id===current.selected_node;return option;}));
 const positions=new Map(current.nodes.map((site,index)=>[site.id,{x:100+(index%2)*210,y:70+Math.floor(index/2)*170}]));
 const rows=Math.ceil(current.nodes.length/2);$('diagram').setAttribute('viewBox',`0 0 420 ${Math.max(330,rows*170)}`);
 const shapes=[];for(const link of current.links){const a=positions.get(link.from),b=positions.get(link.to);shapes.push(svg('line',{x1:a.x,y1:a.y,x2:b.x,y2:b.y,stroke:link.enabled?'#17685c':'#899694','stroke-width':link.enabled?5:2,'stroke-dasharray':link.enabled?'none':'7 5'}));shapes.push(svg('text',{x:(a.x+b.x)/2+8,y:(a.y+b.y)/2-10,'font-size':12,fill:'#173c40'},`${number(link.ops_per_day)} ops/day${link.enabled?'':' (disabled)'}`));}
 for(const site of current.nodes){const at=positions.get(site.id);shapes.push(svg('circle',{cx:at.x,cy:at.y,r:22,fill:site.id===current.selected_node?'#17685c':'#e7eee6',stroke:'#17685c','stroke-width':2}));shapes.push(svg('text',{x:at.x,y:at.y+43,'text-anchor':'middle','font-size':13,fill:'#173c40'},site.name));}
 $('diagram').replaceChildren(...shapes);
 $('link-controls').replaceChildren(...current.links.map(link=>{const group=node('div');group.className='link-control';group.append(node('h3',`${link.id}: ${link.from} ↔ ${link.to}`));const enabled=node('input');enabled.type='checkbox';enabled.checked=link.enabled;enabled.id=`enabled-${link.id}`;const toggle=node('label','Enabled ');toggle.htmlFor=enabled.id;toggle.append(enabled);const capacity=node('input');capacity.type='number';capacity.min='0.001';capacity.max='1000000000';capacity.step='any';capacity.value=String(link.ops_per_day);capacity.id=`capacity-${link.id}`;const amount=node('label','Rating (ops/day, Peak) ');amount.htmlFor=capacity.id;amount.append(capacity);const update=()=>{const input=JSON.parse($('input').value),target=input.links.find(row=>row.id===link.id);target.enabled=enabled.checked;target.ops_per_day=Number(capacity.value);$('input').value=JSON.stringify(input,null,2);compare();};enabled.addEventListener('change',update);capacity.addEventListener('change',update);group.append(toggle,amount);return group;}));
 $('results').hidden=false;$('download').disabled=false;
}
function compare(){
 if(!ready)return;invalidate('Comparing locally in Rust WebAssembly…');const input=$('input').value;
 if(new TextEncoder().encode(input).length>65536){status('Input exceeds 64 KiB. Shorten the synthetic graph.');return;}
 let baseline=examples[0];try{const current=JSON.parse(input),selected=current.selected_node;const reset=JSON.parse(examples[0]);if(reset.nodes.some(site=>site.id===selected)&&reset.nodes.some(site=>site.id===current.hub&&site.role==='Hub')){reset.selected_node=selected;reset.hub=current.hub;baseline=JSON.stringify(reset);}else baseline=undefined;}catch{/* The Rust parser reports the authoritative error. */}
 const id=revision;timer=setTimeout(()=>fatal('Calculation timed out. Reload to restart the worker.'),10000);worker.postMessage({type:'evaluate',id,input,baseline});
}
$('input').addEventListener('input',()=>invalidate('Inputs changed. Compare again to refresh results.'));
$('selected').addEventListener('change',()=>{try{const input=JSON.parse($('input').value);input.selected_node=$('selected').value;$('input').value=JSON.stringify(input,null,2);compare();}catch{invalidate('Fix the graph JSON before changing nodes.');}});
$('compare').addEventListener('click',compare);$('reset').addEventListener('click',()=>{$('input').value=examples[0];compare();});$('outage').addEventListener('click',()=>{$('input').value=examples[1];compare();});
$('download').addEventListener('click',()=>{if(!result)return;const url=URL.createObjectURL(new Blob([JSON.stringify(result,null,2)],{type:'application/json'}));const a=node('a');a.href=url;a.download='tarmac-synthetic-comparison.json';a.click();setTimeout(()=>URL.revokeObjectURL(url),1000);});
try{
 examples=await Promise.all(['example.json','outage.json'].map(async path=>{const response=await fetch(path,{signal:AbortSignal.timeout(10000)});if(!response.ok)throw Error(`Fixture download failed (${response.status})`);return response.text();}));$('input').value=examples[0];
 worker=new Worker(new URL('worker.js',import.meta.url),{type:'module'});timer=setTimeout(()=>fatal('Rust WebAssembly loading timed out. Reload to retry.'),15000);worker.onerror=()=>fatal('Unable to start Rust WebAssembly. Reload to retry.');
 worker.onmessage=({data})=>{if(data.type==='ready'){clearTimeout(timer);ready=true;for(const id of ['compare','reset','outage','selected'])$(id).disabled=false;compare();return;}if(data.type==='error'&&data.id===undefined){fatal(`Unable to load Rust WebAssembly: ${data.error}`);return;}if(data.id!==revision)return;clearTimeout(timer);if(data.type==='error'||!data.ok){invalidate(`Cannot compare: ${data.error}`);return;}result={synthetic:true,current:data.result,baseline:data.baseline??null};render(data.result,data.baseline);status('Comparison ready · computed locally in Rust WebAssembly.');};worker.postMessage({type:'init'});
}catch(error){fatal(`Unable to load examples: ${error.message}`);}
