let M=32,S=0,W=`string`,P=1,Y=`Object`,Q=`utf-8`,O=null,V=`number`,$=4,X=`function`,L=Array,Z=Error,T=Float64Array,U=Int32Array,a1=Object,a0=Reflect,_=Uint32Array,R=Uint8Array,N=undefined;var y=((c,d,e)=>{try{a.wasm_bindgen__convert__closures__invoke1_mut_ref__h70542cf1c06c7c94(c,d,x(e))}finally{b[w++]=N}});var q=(a=>{const b=typeof a;if(b==V||b==`boolean`||a==O){return `${a}`};if(b==W){return `"${a}"`};if(b==`symbol`){const b=a.description;if(b==O){return `Symbol`}else{return `Symbol(${b})`}};if(b==X){const b=a.name;if(typeof b==W&&b.length>S){return `Function(${b})`}else{return `Function`}};if(L.isArray(a)){const b=a.length;let c=`[`;if(b>S){c+=q(a[S])};for(let d=P;d<b;d++){c+=`, `+ q(a[d])};c+=`]`;return c};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c.length>P){d=c[P]}else{return toString.call(a)};if(d==Y){try{return `Object(`+ JSON.stringify(a)+ `)`}catch(a){return Y}};if(a instanceof Z){return `${a.name}: ${a.message}\n${a.stack}`};return d});var D=((a,b)=>{const c=C();const d=c.subarray(a/$,a/$+ b);const e=[];for(let a=S;a<d.length;a++){e.push(f(d[a]))};return e});var A=((b,c,d)=>{a.wasm_bindgen__convert__closures__invoke1__hed0edf2a05f7d9cf(b,c,g(d))});var g=(a=>{if(d===b.length)b.push(b.length+ P);const c=d;d=b[c];b[c]=a;return c});var f=(a=>{const b=c(a);e(a);return b});var K=(async(a)=>{if(typeof a===`undefined`){a=new URL(`wangdenticon-yew-wasm-245610845844e3f0_bg.wasm`,import.meta.url)};const b=G();if(typeof a===W||typeof Request===X&&a instanceof Request||typeof URL===X&&a instanceof URL){a=fetch(a)};H(b);const {instance:c,module:d}=await F(await a,b);return I(c,d)});var J=(a=>{const b=G();H(b);const c=new WebAssembly.Module(a);const d=new WebAssembly.Instance(c,b);return I(d,c)});function E(b,c){try{return b.apply(this,c)}catch(b){a.__wbindgen_exn_store(g(b))}}var I=((b,c)=>{a=b.exports;K.__wbindgen_wasm_module=c;m=new T();o=new U();B=new _();i=new R();a.__wbindgen_start();return a});var F=(async(a,b)=>{if(typeof Response===X&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming===X){try{return await WebAssembly.instantiateStreaming(a,b)}catch(b){if(a.headers.get(`Content-Type`)!=`application/wasm`){console.warn(`\`WebAssembly.instantiateStreaming\` failed because your server does not serve wasm with \`application/wasm\` MIME type. Falling back to \`WebAssembly.instantiate\` which is slower. Original error:\\n`,b)}else{throw b}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}});var p=(()=>{if(o.byteLength===S){o=new U(a.memory.buffer)};return o});var l=(a=>a===N||a===O);var c=(a=>b[a]);var H=((a,b)=>{});var G=(()=>{const b={};b.wbg={};b.wbg.__wbindgen_object_drop_ref=(a=>{f(a)});b.wbg.__wbindgen_object_clone_ref=(a=>{const b=c(a);return g(b)});b.wbg.__wbindgen_string_new=((a,b)=>{const c=k(a,b);return g(c)});b.wbg.__wbindgen_cb_drop=(a=>{const b=f(a).original;if(b.cnt--==P){b.a=S;return !0};const c=!1;return c});b.wbg.__wbindgen_number_new=(a=>{const b=a;return g(b)});b.wbg.__wbindgen_number_get=((a,b)=>{const d=c(b);const e=typeof d===V?d:N;n()[a/8+ P]=l(e)?S:e;p()[a/$+ S]=!l(e)});b.wbg.__wbg_new_693216e109162396=(()=>{const a=new Z();return g(a)});b.wbg.__wbg_stack_0ddaca5d1abfb52f=((b,d)=>{const e=c(d).stack;const f=u(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=r;p()[b/$+ P]=g;p()[b/$+ S]=f});b.wbg.__wbg_error_09919627ac0992f5=((b,c)=>{try{console.error(k(b,c))}finally{a.__wbindgen_free(b,c)}});b.wbg.__wbg_warn_921059440157e870=((b,c)=>{var d=D(b,c).slice();a.__wbindgen_free(b,c*$);console.warn(...d)});b.wbg.__wbg_instanceof_Element_1714e50f9bda1d15=(a=>{const b=c(a) instanceof Element;return b});b.wbg.__wbg_namespaceURI_b343a4afa454dd59=((b,d)=>{const e=c(d).namespaceURI;var f=l(e)?S:u(e,a.__wbindgen_malloc,a.__wbindgen_realloc);var g=r;p()[b/$+ P]=g;p()[b/$+ S]=f});b.wbg.__wbg_removeAttribute_2d6e56b2f03aa57e=function(){return E(((a,b,d)=>{c(a).removeAttribute(k(b,d))}),arguments)};b.wbg.__wbg_setAttribute_8cfc462c0dedd03b=function(){return E(((a,b,d,e,f)=>{c(a).setAttribute(k(b,d),k(e,f))}),arguments)};b.wbg.__wbg_instanceof_Window_42f092928baaee84=(a=>{const b=c(a) instanceof Window;return b});b.wbg.__wbg_document_15b2e504fb1556d6=(a=>{const b=c(a).document;return l(b)?S:g(b)});b.wbg.__wbg_location_312161fbd0cf64f0=(a=>{const b=c(a).location;return g(b)});b.wbg.__wbg_history_cb2cdfbe20fef7ad=function(){return E((a=>{const b=c(a).history;return g(b)}),arguments)};b.wbg.__wbg_body_5e6efc7a3c1b65f3=(a=>{const b=c(a).body;return l(b)?S:g(b)});b.wbg.__wbg_createElement_28fc3740fb11defb=function(){return E(((a,b,d)=>{const e=c(a).createElement(k(b,d));return g(e)}),arguments)};b.wbg.__wbg_createElementNS_dd6cca2457c8c16c=function(){return E(((a,b,d,e,f)=>{const h=c(a).createElementNS(b===S?N:k(b,d),k(e,f));return g(h)}),arguments)};b.wbg.__wbg_createTextNode_2ab1e3ebc34e2641=((a,b,d)=>{const e=c(a).createTextNode(k(b,d));return g(e)});b.wbg.__wbg_querySelector_73feab41810011dc=function(){return E(((a,b,d)=>{const e=c(a).querySelector(k(b,d));return l(e)?S:g(e)}),arguments)};b.wbg.__wbg_instanceof_HtmlInputElement_3fad42774bc62388=(a=>{const b=c(a) instanceof HTMLInputElement;return b});b.wbg.__wbg_setchecked_a450b330df6b3fa5=((a,b)=>{c(a).checked=b!==S});b.wbg.__wbg_value_30770021ca38e0db=((b,d)=>{const e=c(d).value;const f=u(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=r;p()[b/$+ P]=g;p()[b/$+ S]=f});b.wbg.__wbg_setvalue_7b7950dacc5eb607=((a,b,d)=>{c(a).value=k(b,d)});b.wbg.__wbg_value_eb32f706ae6bfab2=((b,d)=>{const e=c(d).value;const f=u(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=r;p()[b/$+ P]=g;p()[b/$+ S]=f});b.wbg.__wbg_setvalue_3dd349be116107ce=((a,b,d)=>{c(a).value=k(b,d)});b.wbg.__wbg_addEventListener_ec92ea1297eefdfc=function(){return E(((a,b,d,e,f)=>{c(a).addEventListener(k(b,d),c(e),c(f))}),arguments)};b.wbg.__wbg_removeEventListener_bfe676215a590711=function(){return E(((a,b,d,e,f)=>{c(a).removeEventListener(k(b,d),c(e),f!==S)}),arguments)};b.wbg.__wbg_parentElement_14138ef2ff0b9c88=(a=>{const b=c(a).parentElement;return l(b)?S:g(b)});b.wbg.__wbg_lastChild_2d1fa5efd0e0edcc=(a=>{const b=c(a).lastChild;return l(b)?S:g(b)});b.wbg.__wbg_setnodeValue_59d46f408f89fd0b=((a,b,d)=>{c(a).nodeValue=b===S?N:k(b,d)});b.wbg.__wbg_appendChild_d21bac021b5bbfde=function(){return E(((a,b)=>{const d=c(a).appendChild(c(b));return g(d)}),arguments)};b.wbg.__wbg_insertBefore_26dfd5eb687a3438=function(){return E(((a,b,d)=>{const e=c(a).insertBefore(c(b),c(d));return g(e)}),arguments)};b.wbg.__wbg_removeChild_94b0c126b878241b=function(){return E(((a,b)=>{const d=c(a).removeChild(c(b));return g(d)}),arguments)};b.wbg.__wbg_target_68a5c10e2732a79e=(a=>{const b=c(a).target;return l(b)?S:g(b)});b.wbg.__wbg_cancelBubble_aa216b328c490cb1=(a=>{const b=c(a).cancelBubble;return b});b.wbg.__wbg_href_cae04ee9562fc683=((b,d)=>{const e=c(d).href;const f=u(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=r;p()[b/$+ P]=g;p()[b/$+ S]=f});b.wbg.__wbg_pathname_8ed2fc02f98aeaaf=((b,d)=>{const e=c(d).pathname;const f=u(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=r;p()[b/$+ P]=g;p()[b/$+ S]=f});b.wbg.__wbg_new_d1d1300265e34170=function(){return E(((a,b)=>{const c=new URL(k(a,b));return g(c)}),arguments)};b.wbg.__wbg_pathname_c08f1ef51f6ebba9=function(){return E(((b,d)=>{const e=c(d).pathname;const f=u(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=r;p()[b/$+ P]=g;p()[b/$+ S]=f}),arguments)};b.wbg.__wbg_debug_1dccd22b8a8988e1=((a,b,d,e)=>{console.debug(c(a),c(b),c(d),c(e))});b.wbg.__wbg_error_800b8d466653f7ea=(a=>{console.error(c(a))});b.wbg.__wbg_error_d539c0f5eafe6a31=((a,b,d,e)=>{console.error(c(a),c(b),c(d),c(e))});b.wbg.__wbg_info_17d18b9f8eaab7d9=((a,b,d,e)=>{console.info(c(a),c(b),c(d),c(e))});b.wbg.__wbg_log_f286f3fe4aad906d=((a,b,d,e)=>{console.log(c(a),c(b),c(d),c(e))});b.wbg.__wbg_warn_3d6689f77cb29c86=((a,b,d,e)=>{console.warn(c(a),c(b),c(d),c(e))});b.wbg.__wbg_newnoargs_971e9a5abe185139=((a,b)=>{const c=new Function(k(a,b));return g(c)});b.wbg.__wbg_get_72332cd2bc57924c=function(){return E(((a,b)=>{const d=a0.get(c(a),c(b));return g(d)}),arguments)};b.wbg.__wbg_call_33d7bcddbbfa394a=function(){return E(((a,b)=>{const d=c(a).call(c(b));return g(d)}),arguments)};b.wbg.__wbg_new_e6a9fecc2bf26696=(()=>{const a=new a1();return g(a)});b.wbg.__wbg_self_fd00a1ef86d1b2ed=function(){return E((()=>{const a=self.self;return g(a)}),arguments)};b.wbg.__wbg_window_6f6e346d8bbd61d7=function(){return E((()=>{const a=window.window;return g(a)}),arguments)};b.wbg.__wbg_globalThis_3348936ac49df00a=function(){return E((()=>{const a=globalThis.globalThis;return g(a)}),arguments)};b.wbg.__wbg_global_67175caf56f55ca9=function(){return E((()=>{const a=global.global;return g(a)}),arguments)};b.wbg.__wbindgen_is_undefined=(a=>{const b=c(a)===N;return b});b.wbg.__wbg_valueOf_f83bee79f23e7b05=(a=>{const b=c(a).valueOf();return b});b.wbg.__wbg_is_43eb2f9708e964a9=((a,b)=>{const d=a1.is(c(a),c(b));return d});b.wbg.__wbg_set_2762e698c2f5b7e0=function(){return E(((a,b,d)=>{const e=a0.set(c(a),c(b),c(d));return e}),arguments)};b.wbg.__wbindgen_debug_string=((b,d)=>{const e=q(c(d));const f=u(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=r;p()[b/$+ P]=g;p()[b/$+ S]=f});b.wbg.__wbindgen_throw=((a,b)=>{throw new Z(k(a,b))});b.wbg.__wbindgen_closure_wrapper850=((a,b,c)=>{const d=v(a,b,388,y);return g(d)});b.wbg.__wbindgen_closure_wrapper996=((a,b,c)=>{const d=z(a,b,445,A);return g(d)});return b});var C=(()=>{if(B.byteLength===S){B=new _(a.memory.buffer)};return B});var n=(()=>{if(m.byteLength===S){m=new T(a.memory.buffer)};return m});var z=((b,c,d,e)=>{const f={a:b,b:c,cnt:P,dtor:d};const g=(...b)=>{f.cnt++;try{return e(f.a,f.b,...b)}finally{if(--f.cnt===S){a.__wbindgen_export_2.get(f.dtor)(f.a,f.b);f.a=S}}};g.original=f;return g});var e=(a=>{if(a<36)return;b[a]=d;d=a});var v=((b,c,d,e)=>{const f={a:b,b:c,cnt:P,dtor:d};const g=(...b)=>{f.cnt++;const c=f.a;f.a=S;try{return e(c,f.b,...b)}finally{if(--f.cnt===S){a.__wbindgen_export_2.get(f.dtor)(c,f.b)}else{f.a=c}}};g.original=f;return g});var u=((a,b,c)=>{if(c===N){const c=s.encode(a);const d=b(c.length);j().subarray(d,d+ c.length).set(c);r=c.length;return d};let d=a.length;let e=b(d);const f=j();let g=S;for(;g<d;g++){const b=a.charCodeAt(g);if(b>127)break;f[e+ g]=b};if(g!==d){if(g!==S){a=a.slice(g)};e=c(e,d,d=g+ a.length*3);const b=j().subarray(e+ g,e+ d);const f=t(a,b);g+=f.written};r=g;return e});var j=(()=>{if(i.byteLength===S){i=new R(a.memory.buffer)};return i});var k=((a,b)=>h.decode(j().subarray(a,a+ b)));var x=(a=>{if(w==P)throw new Z(`out of js stack`);b[--w]=a;return w});let a;const b=new L(M).fill(N);b.push(N,O,!0,!1);let d=b.length;const h=new TextDecoder(Q,{ignoreBOM:!0,fatal:!0});h.decode();let i=new R();let m=new T();let o=new U();let r=S;const s=new TextEncoder(Q);const t=typeof s.encodeInto===X?((a,b)=>s.encodeInto(a,b)):((a,b)=>{const c=s.encode(a);b.set(c);return {read:a.length,written:c.length}});let w=M;let B=new _();export default K;export{J as initSync}