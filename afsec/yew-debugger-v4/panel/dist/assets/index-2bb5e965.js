(function(){const e=document.createElement("link").relList;if(e&&e.supports&&e.supports("modulepreload"))return;for(const o of document.querySelectorAll('link[rel="modulepreload"]'))r(o);new MutationObserver(o=>{for(const _ of o)if(_.type==="childList")for(const s of _.addedNodes)s.tagName==="LINK"&&s.rel==="modulepreload"&&r(s)}).observe(document,{childList:!0,subtree:!0});function n(o){const _={};return o.integrity&&(_.integrity=o.integrity),o.referrerPolicy&&(_.referrerPolicy=o.referrerPolicy),o.crossOrigin==="use-credentials"?_.credentials="include":o.crossOrigin==="anonymous"?_.credentials="omit":_.credentials="same-origin",_}function r(o){if(o.ep)return;o.ep=!0;const _=n(o);fetch(o.href,_)}})();let u;const w=new Array(128).fill(void 0);w.push(void 0,null,!0,!1);function c(t){return w[t]}let A=w.length;function j(t){t<132||(w[t]=A,A=t)}function M(t){const e=c(t);return j(t),e}const L=typeof TextDecoder<"u"?new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw Error("TextDecoder not available")}};typeof TextDecoder<"u"&&L.decode();let h=null;function O(){return(h===null||h.byteLength===0)&&(h=new Uint8Array(u.memory.buffer)),h}function f(t,e){return t=t>>>0,L.decode(O().subarray(t,t+e))}function i(t){A===w.length&&w.push(w.length+1);const e=A;return A=w[e],w[e]=t,e}let d=0;const S=typeof TextEncoder<"u"?new TextEncoder("utf-8"):{encode:()=>{throw Error("TextEncoder not available")}},R=typeof S.encodeInto=="function"?function(t,e){return S.encodeInto(t,e)}:function(t,e){const n=S.encode(t);return e.set(n),{read:t.length,written:n.length}};function m(t,e,n){if(n===void 0){const l=S.encode(t),y=e(l.length,1)>>>0;return O().subarray(y,y+l.length).set(l),d=l.length,y}let r=t.length,o=e(r,1)>>>0;const _=O();let s=0;for(;s<r;s++){const l=t.charCodeAt(s);if(l>127)break;_[o+s]=l}if(s!==r){s!==0&&(t=t.slice(s)),o=n(o,r,r=s+t.length*3,1)>>>0;const l=O().subarray(o+s,o+r),y=R(t,l);s+=y.written}return d=s,o}function g(t){return t==null}let p=null;function b(){return(p===null||p.byteLength===0)&&(p=new Int32Array(u.memory.buffer)),p}function v(t){const e=typeof t;if(e=="number"||e=="boolean"||t==null)return`${t}`;if(e=="string")return`"${t}"`;if(e=="symbol"){const o=t.description;return o==null?"Symbol":`Symbol(${o})`}if(e=="function"){const o=t.name;return typeof o=="string"&&o.length>0?`Function(${o})`:"Function"}if(Array.isArray(t)){const o=t.length;let _="[";o>0&&(_+=v(t[0]));for(let s=1;s<o;s++)_+=", "+v(t[s]);return _+="]",_}const n=/\[object ([^\]]+)\]/.exec(toString.call(t));let r;if(n.length>1)r=n[1];else return toString.call(t);if(r=="Object")try{return"Object("+JSON.stringify(t)+")"}catch{return"Object"}return t instanceof Error?`${t.name}: ${t.message}
${t.stack}`:r}function W(t,e,n,r){const o={a:t,b:e,cnt:1,dtor:n},_=(...s)=>{o.cnt++;try{return r(o.a,o.b,...s)}finally{--o.cnt===0&&(u.__wbindgen_export_2.get(o.dtor)(o.a,o.b),o.a=0)}};return _.original=o,_}let T=128;function k(t){if(T==1)throw new Error("out of js stack");return w[--T]=t,T}function C(t,e,n){try{u._dyn_core__ops__function__Fn___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h8bd90b67df59050d(t,e,k(n))}finally{w[T++]=void 0}}function I(t,e,n,r){const o={a:t,b:e,cnt:1,dtor:n},_=(...s)=>{o.cnt++;const l=o.a;o.a=0;try{return r(l,o.b,...s)}finally{--o.cnt===0?u.__wbindgen_export_2.get(o.dtor)(l,o.b):o.a=l}};return _.original=o,_}function U(t,e,n){u._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h067ea33cb8944142(t,e,i(n))}let E=null;function F(){return(E===null||E.byteLength===0)&&(E=new Uint32Array(u.memory.buffer)),E}function x(t,e){t=t>>>0;const r=F().subarray(t/4,t/4+e),o=[];for(let _=0;_<r.length;_++)o.push(M(r[_]));return o}function a(t,e){try{return t.apply(this,e)}catch(n){u.__wbindgen_exn_store(i(n))}}async function B(t,e){if(typeof Response=="function"&&t instanceof Response){if(typeof WebAssembly.instantiateStreaming=="function")try{return await WebAssembly.instantiateStreaming(t,e)}catch(r){if(t.headers.get("Content-Type")!="application/wasm")console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",r);else throw r}const n=await t.arrayBuffer();return await WebAssembly.instantiate(n,e)}else{const n=await WebAssembly.instantiate(t,e);return n instanceof WebAssembly.Instance?{instance:n,module:t}:n}}function P(){const t={};return t.wbg={},t.wbg.__wbindgen_object_drop_ref=function(e){M(e)},t.wbg.__wbindgen_string_new=function(e,n){const r=f(e,n);return i(r)},t.wbg.__wbindgen_object_clone_ref=function(e){const n=c(e);return i(n)},t.wbg.__wbg_subtreeid_e80a1798fee782f9=function(e,n){const r=c(n).__yew_subtree_id;b()[e/4+1]=g(r)?0:r,b()[e/4+0]=!g(r)},t.wbg.__wbg_setsubtreeid_e1fab6b578c800cf=function(e,n){c(e).__yew_subtree_id=n>>>0},t.wbg.__wbg_cachekey_b81c1aacc6a0645c=function(e,n){const r=c(n).__yew_subtree_cache_key;b()[e/4+1]=g(r)?0:r,b()[e/4+0]=!g(r)},t.wbg.__wbg_setcachekey_75bcd45312087529=function(e,n){c(e).__yew_subtree_cache_key=n>>>0},t.wbg.__wbindgen_cb_drop=function(e){const n=M(e).original;return n.cnt--==1?(n.a=0,!0):!1},t.wbg.__wbg_listenerid_6dcf1c62b7b7de58=function(e,n){const r=c(n).__yew_listener_id;b()[e/4+1]=g(r)?0:r,b()[e/4+0]=!g(r)},t.wbg.__wbg_setlistenerid_f2e783343fa0cec1=function(e,n){c(e).__yew_listener_id=n>>>0},t.wbg.__wbg_new_abda76e883ba8a5f=function(){const e=new Error;return i(e)},t.wbg.__wbg_stack_658279fe44541cf6=function(e,n){const r=c(n).stack,o=m(r,u.__wbindgen_malloc,u.__wbindgen_realloc),_=d;b()[e/4+1]=_,b()[e/4+0]=o},t.wbg.__wbg_error_f851667af71bcfc6=function(e,n){let r,o;try{r=e,o=n,console.error(f(e,n))}finally{u.__wbindgen_free(r,o,1)}},t.wbg.__wbg_error_a526fb08a0205972=function(e,n){var r=x(e,n).slice();u.__wbindgen_free(e,n*4),console.error(...r)},t.wbg.__wbg_log_7c3433e130418e14=function(e,n){var r=x(e,n).slice();u.__wbindgen_free(e,n*4),console.log(...r)},t.wbg.__wbg_body_674aec4c1c0910cd=function(e){const n=c(e).body;return g(n)?0:i(n)},t.wbg.__wbg_createElement_4891554b28d3388b=function(){return a(function(e,n,r){const o=c(e).createElement(f(n,r));return i(o)},arguments)},t.wbg.__wbg_createElementNS_119acf9e82482041=function(){return a(function(e,n,r,o,_){const s=c(e).createElementNS(n===0?void 0:f(n,r),f(o,_));return i(s)},arguments)},t.wbg.__wbg_createTextNode_2fd22cd7e543f938=function(e,n,r){const o=c(e).createTextNode(f(n,r));return i(o)},t.wbg.__wbg_instanceof_Window_9029196b662bc42a=function(e){let n;try{n=c(e)instanceof Window}catch{n=!1}return n},t.wbg.__wbg_document_f7ace2b956f30a4f=function(e){const n=c(e).document;return g(n)?0:i(n)},t.wbg.__wbg_instanceof_Element_4622f5da1249a3eb=function(e){let n;try{n=c(e)instanceof Element}catch{n=!1}return n},t.wbg.__wbg_namespaceURI_31718ed49b5343a3=function(e,n){const r=c(n).namespaceURI;var o=g(r)?0:m(r,u.__wbindgen_malloc,u.__wbindgen_realloc),_=d;b()[e/4+1]=_,b()[e/4+0]=o},t.wbg.__wbg_setinnerHTML_b089587252408b67=function(e,n,r){c(e).innerHTML=f(n,r)},t.wbg.__wbg_outerHTML_f7749ceff37b5832=function(e,n){const r=c(n).outerHTML,o=m(r,u.__wbindgen_malloc,u.__wbindgen_realloc),_=d;b()[e/4+1]=_,b()[e/4+0]=o},t.wbg.__wbg_removeAttribute_d8404da431968808=function(){return a(function(e,n,r){c(e).removeAttribute(f(n,r))},arguments)},t.wbg.__wbg_setAttribute_e7e80b478b7b8b2f=function(){return a(function(e,n,r,o,_){c(e).setAttribute(f(n,r),f(o,_))},arguments)},t.wbg.__wbindgen_string_get=function(e,n){const r=c(n),o=typeof r=="string"?r:void 0;var _=g(o)?0:m(o,u.__wbindgen_malloc,u.__wbindgen_realloc),s=d;b()[e/4+1]=s,b()[e/4+0]=_},t.wbg.__wbg_parentNode_9e53f8b17eb98c9d=function(e){const n=c(e).parentNode;return g(n)?0:i(n)},t.wbg.__wbg_parentElement_c75962bc9997ea5f=function(e){const n=c(e).parentElement;return g(n)?0:i(n)},t.wbg.__wbg_childNodes_64dab37cf9d252dd=function(e){const n=c(e).childNodes;return i(n)},t.wbg.__wbg_lastChild_0cee692010bac6c2=function(e){const n=c(e).lastChild;return g(n)?0:i(n)},t.wbg.__wbg_nextSibling_304d9aac7c2774ae=function(e){const n=c(e).nextSibling;return g(n)?0:i(n)},t.wbg.__wbg_setnodeValue_d1c8382910b45e04=function(e,n,r){c(e).nodeValue=n===0?void 0:f(n,r)},t.wbg.__wbg_textContent_c5d9e21ee03c63d4=function(e,n){const r=c(n).textContent;var o=g(r)?0:m(r,u.__wbindgen_malloc,u.__wbindgen_realloc),_=d;b()[e/4+1]=_,b()[e/4+0]=o},t.wbg.__wbg_cloneNode_1f7cce4ea8b708e2=function(){return a(function(e){const n=c(e).cloneNode();return i(n)},arguments)},t.wbg.__wbg_insertBefore_ffa01d4b747c95fc=function(){return a(function(e,n,r){const o=c(e).insertBefore(c(n),c(r));return i(o)},arguments)},t.wbg.__wbg_removeChild_973429f368206138=function(){return a(function(e,n){const r=c(e).removeChild(c(n));return i(r)},arguments)},t.wbg.__wbg_bubbles_63572b91f3885ef1=function(e){return c(e).bubbles},t.wbg.__wbg_cancelBubble_90d1c3aa2a76cbeb=function(e){return c(e).cancelBubble},t.wbg.__wbg_composedPath_cf1bb5b8bcff496f=function(e){const n=c(e).composedPath();return i(n)},t.wbg.__wbg_addEventListener_a5963e26cd7b176b=function(){return a(function(e,n,r,o,_){c(e).addEventListener(f(n,r),c(o),c(_))},arguments)},t.wbg.__wbg_instanceof_ShadowRoot_b64337370f59fe2d=function(e){let n;try{n=c(e)instanceof ShadowRoot}catch{n=!1}return n},t.wbg.__wbg_host_e1c47c33975060d3=function(e){const n=c(e).host;return i(n)},t.wbg.__wbg_value_3c5f08ffc2b7d6f9=function(e,n){const r=c(n).value,o=m(r,u.__wbindgen_malloc,u.__wbindgen_realloc),_=d;b()[e/4+1]=_,b()[e/4+0]=o},t.wbg.__wbg_setvalue_0dc100d4b9908028=function(e,n,r){c(e).value=f(n,r)},t.wbg.__wbg_setchecked_e5a50baea447b8a8=function(e,n){c(e).checked=n!==0},t.wbg.__wbg_value_9423da9d988ee8cf=function(e,n){const r=c(n).value,o=m(r,u.__wbindgen_malloc,u.__wbindgen_realloc),_=d;b()[e/4+1]=_,b()[e/4+0]=o},t.wbg.__wbg_setvalue_1f95e61cbc382f7f=function(e,n,r){c(e).value=f(n,r)},t.wbg.__wbg_postMessage_2f0b8369b84c3c1e=function(){return a(function(e,n){c(e).postMessage(c(n))},arguments)},t.wbg.__wbg_get_44be0491f933a435=function(e,n){const r=c(e)[n>>>0];return i(r)},t.wbg.__wbg_length_fff51ee6522a1a18=function(e){return c(e).length},t.wbg.__wbg_newnoargs_581967eacc0e2604=function(e,n){const r=new Function(f(e,n));return i(r)},t.wbg.__wbg_call_cb65541d95d71282=function(){return a(function(e,n){const r=c(e).call(c(n));return i(r)},arguments)},t.wbg.__wbg_new_b51585de1b234aff=function(){const e=new Object;return i(e)},t.wbg.__wbg_self_1ff1d729e9aae938=function(){return a(function(){const e=self.self;return i(e)},arguments)},t.wbg.__wbg_window_5f4faef6c12b79ec=function(){return a(function(){const e=window.window;return i(e)},arguments)},t.wbg.__wbg_globalThis_1d39714405582d3c=function(){return a(function(){const e=globalThis.globalThis;return i(e)},arguments)},t.wbg.__wbg_global_651f05c6a0944d1c=function(){return a(function(){const e=global.global;return i(e)},arguments)},t.wbg.__wbindgen_is_undefined=function(e){return c(e)===void 0},t.wbg.__wbg_from_d7c216d4616bb368=function(e){const n=Array.from(c(e));return i(n)},t.wbg.__wbg_new0_c0be7df4b6bd481f=function(){return i(new Date)},t.wbg.__wbg_toString_6dc15322a0919dfa=function(e){const n=c(e).toString();return i(n)},t.wbg.__wbg_is_205d914af04a8faa=function(e,n){return Object.is(c(e),c(n))},t.wbg.__wbg_resolve_53698b95aaf7fcf8=function(e){const n=Promise.resolve(c(e));return i(n)},t.wbg.__wbg_then_f7e06ee3c11698eb=function(e,n){const r=c(e).then(c(n));return i(r)},t.wbg.__wbg_parse_670c19d4e984792e=function(){return a(function(e,n){const r=JSON.parse(f(e,n));return i(r)},arguments)},t.wbg.__wbg_set_092e06b0f9d71865=function(){return a(function(e,n,r){return Reflect.set(c(e),c(n),c(r))},arguments)},t.wbg.__wbindgen_debug_string=function(e,n){const r=v(c(n)),o=m(r,u.__wbindgen_malloc,u.__wbindgen_realloc),_=d;b()[e/4+1]=_,b()[e/4+0]=o},t.wbg.__wbindgen_throw=function(e,n){throw new Error(f(e,n))},t.wbg.__wbindgen_closure_wrapper301=function(e,n,r){const o=W(e,n,91,C);return i(o)},t.wbg.__wbindgen_closure_wrapper536=function(e,n,r){const o=I(e,n,183,U);return i(o)},t}function $(t,e){return u=t.exports,N.__wbindgen_wasm_module=e,p=null,E=null,h=null,u.__wbindgen_start(),u}async function N(t){if(u!==void 0)return u;typeof t>"u"&&(t=new URL("/assets/communication_grandchild_with_grandparent_bg-a1e83e48.wasm",self.location));const e=P();(typeof t=="string"||typeof Request=="function"&&t instanceof Request||typeof URL=="function"&&t instanceof URL)&&(t=fetch(t));const{instance:n,module:r}=await B(await t,e);return $(n,r)}await N();
