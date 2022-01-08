var app=function(){"use strict";function t(){}function e(t){return t()}function n(){return Object.create(null)}function o(t){t.forEach(e)}function s(t){return"function"==typeof t}function l(t,e){return t!=t?e==e:t!==e||t&&"object"==typeof t||"function"==typeof t}function a(e,n,o){e.$$.on_destroy.push(function(e,...n){if(null==e)return t;const o=e.subscribe(...n);return o.unsubscribe?()=>o.unsubscribe():o}(n,o))}function i(t,e){t.appendChild(e)}function r(t,e,n){t.insertBefore(e,n||null)}function c(t){t.parentNode.removeChild(t)}function u(t){return document.createElement(t)}function f(t){return document.createTextNode(t)}function d(){return f(" ")}function p(t,e,n,o){return t.addEventListener(e,n,o),()=>t.removeEventListener(e,n,o)}function m(t){return function(e){return e.preventDefault(),t.call(this,e)}}function g(t,e,n){null==n?t.removeAttribute(e):t.getAttribute(e)!==n&&t.setAttribute(e,n)}function h(t,e){e=""+e,t.wholeText!==e&&(t.data=e)}function $(t,e){t.value=null==e?"":e}function v(t,e,n){t.classList[n?"add":"remove"](e)}let y;function w(t){y=t}function b(){if(!y)throw new Error("Function called outside component initialization");return y}function k(t){b().$$.on_mount.push(t)}function x(){const t=b();return(e,n)=>{const o=t.$$.callbacks[e];if(o){const s=function(t,e,n=!1){const o=document.createEvent("CustomEvent");return o.initCustomEvent(t,n,!1,e),o}(e,n);o.slice().forEach((e=>{e.call(t,s)}))}}}const j=[],S=[],_=[],q=[],C=Promise.resolve();let O=!1;function E(t){_.push(t)}const T=new Set;let L=0;function M(){const t=y;do{for(;L<j.length;){const t=j[L];L++,w(t),P(t.$$)}for(w(null),j.length=0,L=0;S.length;)S.pop()();for(let t=0;t<_.length;t+=1){const e=_[t];T.has(e)||(T.add(e),e())}_.length=0}while(j.length);for(;q.length;)q.pop()();O=!1,T.clear(),w(t)}function P(t){if(null!==t.fragment){t.update(),o(t.before_update);const e=t.dirty;t.dirty=[-1],t.fragment&&t.fragment.p(t.ctx,e),t.after_update.forEach(E)}}const A=new Set;let N;function I(){N={r:0,c:[],p:N}}function z(){N.r||o(N.c),N=N.p}function J(t,e){t&&t.i&&(A.delete(t),t.i(e))}function B(t,e,n,o){if(t&&t.o){if(A.has(t))return;A.add(t),N.c.push((()=>{A.delete(t),o&&(n&&t.d(1),o())})),t.o(e)}}function D(t,e){t.d(1),e.delete(t.key)}function H(t){t&&t.c()}function U(t,n,l,a){const{fragment:i,on_mount:r,on_destroy:c,after_update:u}=t.$$;i&&i.m(n,l),a||E((()=>{const n=r.map(e).filter(s);c?c.push(...n):o(n),t.$$.on_mount=[]})),u.forEach(E)}function F(t,e){const n=t.$$;null!==n.fragment&&(o(n.on_destroy),n.fragment&&n.fragment.d(e),n.on_destroy=n.fragment=null,n.ctx=[])}function G(t,e){-1===t.$$.dirty[0]&&(j.push(t),O||(O=!0,C.then(M)),t.$$.dirty.fill(0)),t.$$.dirty[e/31|0]|=1<<e%31}function W(e,s,l,a,i,r,u,f=[-1]){const d=y;w(e);const p=e.$$={fragment:null,ctx:null,props:r,update:t,not_equal:i,bound:n(),on_mount:[],on_destroy:[],on_disconnect:[],before_update:[],after_update:[],context:new Map(s.context||(d?d.$$.context:[])),callbacks:n(),dirty:f,skip_bound:!1,root:s.target||d.$$.root};u&&u(p.root);let m=!1;if(p.ctx=l?l(e,s.props||{},((t,n,...o)=>{const s=o.length?o[0]:n;return p.ctx&&i(p.ctx[t],p.ctx[t]=s)&&(!p.skip_bound&&p.bound[t]&&p.bound[t](s),m&&G(e,t)),n})):[],p.update(),m=!0,o(p.before_update),p.fragment=!!a&&a(p.ctx),s.target){if(s.hydrate){const t=function(t){return Array.from(t.childNodes)}(s.target);p.fragment&&p.fragment.l(t),t.forEach(c)}else p.fragment&&p.fragment.c();s.intro&&J(e.$$.fragment),U(e,s.target,s.anchor,s.customElement),M()}w(d)}class K{$destroy(){F(this,1),this.$destroy=t}$on(t,e){const n=this.$$.callbacks[t]||(this.$$.callbacks[t]=[]);return n.push(e),()=>{const t=n.indexOf(e);-1!==t&&n.splice(t,1)}}$set(t){var e;this.$$set&&(e=t,0!==Object.keys(e).length)&&(this.$$.skip_bound=!0,this.$$set(t),this.$$.skip_bound=!1)}}const Q=[];function R(e,n=t){let o;const s=new Set;function a(t){if(l(e,t)&&(e=t,o)){const t=!Q.length;for(const t of s)t[1](),Q.push(t,e);if(t){for(let t=0;t<Q.length;t+=2)Q[t][0](Q[t+1]);Q.length=0}}}return{set:a,update:function(t){a(t(e))},subscribe:function(l,i=t){const r=[l,i];return s.add(r),1===s.size&&(o=n(a)||t),l(e),()=>{s.delete(r),0===s.size&&(o(),o=null)}}}}const V={state:"loggedout",username:null,userid:null,authtoken:null},X=R(V),Y={subscribe:X.subscribe,appStarted:()=>{const t=localStorage.getItem("todo-auth-state");if(t){let e=JSON.parse(t);e={...e,state:"loggedin"},X.set(e)}},setLoggedOff:()=>{X.set(V),localStorage.removeItem("todo-auth-state")},setSignup:()=>{const t={...V,state:"signup"};X.set(t)},setLogin:()=>{const t={...V,state:"login"};X.set(t)},setLoggedIn:(t,e,n)=>{X.set({state:"loggedin",username:t,userid:e,authtoken:n}),localStorage.setItem("todo-auth-state",JSON.stringify({username:t,userid:e,authtoken:n}))}};function Z(e){let n,s,l,a,f,h,$;return{c(){n=u("li"),s=u("a"),s.textContent="Login",l=d(),a=u("li"),f=u("a"),f.textContent="Signup",g(s,"href","/"),g(s,"class","button svelte-jpnq0b"),g(n,"class","svelte-jpnq0b"),g(f,"href","/"),g(f,"class","button svelte-jpnq0b"),g(a,"class","svelte-jpnq0b")},m(t,o){r(t,n,o),i(n,s),r(t,l,o),r(t,a,o),i(a,f),h||($=[p(s,"click",m(e[2])),p(f,"click",m(e[3]))],h=!0)},p:t,d(t){t&&c(n),t&&c(l),t&&c(a),h=!1,o($)}}}function tt(t){let e,n,o,s,l,a,$,v,y,w=t[0].username+"";return{c(){e=u("li"),n=f(w),o=d(),s=u("li"),s.innerHTML='<a href="#" class="svelte-jpnq0b">About</a>',l=d(),a=u("li"),$=u("a"),$.textContent="Logout",g(e,"class","svelte-jpnq0b"),g(s,"class","svelte-jpnq0b"),g($,"href","/"),g($,"class","svelte-jpnq0b"),g(a,"class","svelte-jpnq0b")},m(c,u){r(c,e,u),i(e,n),r(c,o,u),r(c,s,u),r(c,l,u),r(c,a,u),i(a,$),v||(y=p($,"click",m(t[1])),v=!0)},p(t,e){1&e&&w!==(w=t[0].username+"")&&h(n,w)},d(t){t&&c(e),t&&c(o),t&&c(s),t&&c(l),t&&c(a),v=!1,y()}}}function et(e){let n,o,s,l,a;function f(t,e){return"loggedin"===t[0].state?tt:Z}let p=f(e),m=p(e);return{c(){n=u("header"),o=u("h1"),o.innerHTML='<span><span class="fas fa-tasks svelte-jpnq0b" aria-hidden="true"></span> todo-pad</span>',s=d(),l=u("nav"),a=u("ul"),m.c(),g(o,"class","svelte-jpnq0b"),g(a,"class","svelte-jpnq0b"),g(l,"class","svelte-jpnq0b"),g(n,"class","svelte-jpnq0b")},m(t,e){r(t,n,e),i(n,o),i(n,s),i(n,l),i(l,a),m.m(a,null)},p(t,[e]){p===(p=f(t))&&m?m.p(t,e):(m.d(1),m=p(t),m&&(m.c(),m.m(a,null)))},i:t,o:t,d(t){t&&c(n),m.d()}}}function nt(t,e,n){let o;a(t,Y,(t=>n(0,o=t)));return[o,()=>Y.setLoggedOff(),()=>Y.setLogin(),()=>Y.setSignup()]}class ot extends K{constructor(t){super(),W(this,t,nt,et,l,{})}}async function st(t){const e={method:"GET",headers:{"Content-Type":"application/json",Authorization:`Bearer ${t}`,Accept:"application/json",Origin:"http://127.0.0.1:5000"}},n=await fetch("/todo?pending=true",e);if(!n.ok){const t=await n.text();throw console.log(t),t}return await n.json()}function lt(t){let e,n;return{c(){e=u("p"),n=f(t[4]),g(e,"class","error-message svelte-qk26dr")},m(t,o){r(t,e,o),i(e,n)},p(t,e){16&e&&h(n,t[4])},d(t){t&&c(e)}}}function at(e){let n,o,s,l,a,m,$,y,w=e[6]&&lt(e);return{c(){n=u("div"),o=u("label"),s=f(e[1]),l=d(),a=u("input"),m=d(),w&&w.c(),g(o,"for",e[0]),g(o,"class","svelte-qk26dr"),g(a,"type",e[3]),g(a,"id",e[0]),a.value=e[2],g(a,"class","svelte-qk26dr"),v(a,"invalid",e[6]),g(n,"class","form-control svelte-qk26dr")},m(t,c){r(t,n,c),i(n,o),i(o,s),i(n,l),i(n,a),e[11](a),i(n,m),w&&w.m(n,null),$||(y=p(a,"input",e[7]),$=!0)},p(t,[e]){2&e&&h(s,t[1]),1&e&&g(o,"for",t[0]),8&e&&g(a,"type",t[3]),1&e&&g(a,"id",t[0]),4&e&&a.value!==t[2]&&(a.value=t[2]),64&e&&v(a,"invalid",t[6]),t[6]?w?w.p(t,e):(w=lt(t),w.c(),w.m(n,null)):w&&(w.d(1),w=null)},i:t,o:t,d(t){t&&c(n),e[11](null),w&&w.d(),$=!1,y()}}}function it(t,e,n){let o;const s=x();let l,{id:a}=e,{label:i}=e,{value:r}=e,{type:c}=e,{autofocus:u=0}=e,{valid:f=!0}=e,{validityMessage:d=""}=e;k((()=>{u&&l.focus()}));let p=!1;return t.$$set=t=>{"id"in t&&n(0,a=t.id),"label"in t&&n(1,i=t.label),"value"in t&&n(2,r=t.value),"type"in t&&n(3,c=t.type),"autofocus"in t&&n(8,u=t.autofocus),"valid"in t&&n(9,f=t.valid),"validityMessage"in t&&n(4,d=t.validityMessage)},t.$$.update=()=>{1552&t.$$.dirty&&n(6,o=!f&&d&&p)},[a,i,r,c,d,l,o,function(t){n(10,p=!0),s("input",t)},u,f,p,function(t){S[t?"unshift":"push"]((()=>{l=t,n(5,l)}))}]}class rt extends K{constructor(t){super(),W(this,t,it,at,l,{id:0,label:1,value:2,type:3,autofocus:8,valid:9,validityMessage:4})}}function ct(t){return 0===t.trim().length}function ut(t){return!/^((?!\.)[\w\-_.]*[^.])(@\w+)(\.\w+(\.\w+)?[^.\W])$/.test(t)}function ft(t){let e,n;return e=new rt({props:{id:"name",label:"Name",type:"text",value:t[1],valid:!ct(t[1]),validityMessage:"Please enter your name",autofocus:"true"}}),e.$on("input",t[9]),{c(){H(e.$$.fragment)},m(t,o){U(e,t,o),n=!0},p(t,n){const o={};2&n&&(o.value=t[1]),2&n&&(o.valid=!ct(t[1])),e.$set(o)},i(t){n||(J(e.$$.fragment,t),n=!0)},o(t){B(e.$$.fragment,t),n=!1},d(t){F(e,t)}}}function dt(t){let e,n;return{c(){e=u("p"),n=f(t[4]),g(e,"class","error svelte-yelmpf")},m(t,o){r(t,e,o),i(e,n)},p(t,e){16&e&&h(n,t[4])},d(t){t&&c(e)}}}function pt(t){let e,n,s,l,a,h,$,v,y,w,b,k,x,j,S,_,q,C,O=t[0]&&ft(t);a=new rt({props:{id:"email",label:"Email",type:"email",value:t[2],valid:!ut(t[2]),validityMessage:"Please enter valid email",autofocus:!t[0]}}),a.$on("input",t[10]),$=new rt({props:{id:"password",label:"Password",type:"password",value:t[3],valid:!ct(t[3]),validityMessage:"Please enter your password"}}),$.$on("input",t[11]);let E=t[4]&&dt(t);return{c(){e=u("form"),n=u("h2"),n.textContent=`${t[6]}`,s=d(),O&&O.c(),l=d(),H(a.$$.fragment),h=d(),H($.$$.fragment),v=d(),y=u("div"),w=u("button"),b=f(t[6]),x=d(),j=u("button"),j.textContent="Cancel",S=d(),E&&E.c(),g(n,"class","svelte-yelmpf"),w.disabled=k=!t[5](),g(w,"class","svelte-yelmpf"),g(j,"class","svelte-yelmpf"),g(y,"class","controls svelte-yelmpf"),g(e,"class","svelte-yelmpf")},m(o,c){var u;r(o,e,c),i(e,n),i(e,s),O&&O.m(e,null),i(e,l),U(a,e,null),i(e,h),U($,e,null),i(e,v),i(e,y),i(y,w),i(w,b),i(y,x),i(y,j),i(y,S),E&&E.m(y,null),_=!0,q||(C=[p(w,"click",(u=m(t[8]),function(t){return t.stopPropagation(),u.call(this,t)})),p(j,"click",t[7])],q=!0)},p(t,[n]){t[0]?O?(O.p(t,n),1&n&&J(O,1)):(O=ft(t),O.c(),J(O,1),O.m(e,l)):O&&(I(),B(O,1,1,(()=>{O=null})),z());const o={};4&n&&(o.value=t[2]),4&n&&(o.valid=!ut(t[2])),1&n&&(o.autofocus=!t[0]),a.$set(o);const s={};8&n&&(s.value=t[3]),8&n&&(s.valid=!ct(t[3])),$.$set(s),(!_||32&n&&k!==(k=!t[5]()))&&(w.disabled=k),t[4]?E?E.p(t,n):(E=dt(t),E.c(),E.m(y,null)):E&&(E.d(1),E=null)},i(t){_||(J(O),J(a.$$.fragment,t),J($.$$.fragment,t),_=!0)},o(t){B(O),B(a.$$.fragment,t),B($.$$.fragment,t),_=!1},d(t){t&&c(e),O&&O.d(),F(a),F($),E&&E.d(),q=!1,o(C)}}}function mt(t,e,n){let o,{signup:s=!1}=e,l=s?"Signup":"Login",a="",i="",r="",c=null;return t.$$set=t=>{"signup"in t&&n(0,s=t.signup)},t.$$.update=()=>{15&t.$$.dirty&&n(5,o=()=>{let t=!ct(r)&&!ut(i);return s&&(t=t&&!ct(a)),t})},[s,a,i,r,c,o,l,function(){Y.setLoggedOff()},async function(){if(n(4,c=null),o())try{let t;t=s?await async function(t,e,n){const o={method:"POST",body:JSON.stringify({name:t,email:e,password:n}),headers:{"Content-Type":"application/json"}},s=await fetch("/signup",o);if(!s.ok){const t=await s.text();if(console.log(t),409===s.status)throw"Email already exists. Please login if you have registered already.";throw t}return await s.json()}(a,i,r):await async function(t,e){const n={method:"POST",body:JSON.stringify({email:t,password:e}),headers:{"Content-Type":"application/json"}},o=await fetch("/login",n);if(!o.ok){const t=await o.text();if(console.log(t),404===o.status)throw"Email not found. Please login with valid credentials.";if(403===o.status)throw"Invalid Credentials. Please login with valid credentials.";throw t}return await o.json()}(i,r),Y.setLoggedIn(t.name,t.userid,t.token)}catch(t){n(4,c=t)}else n(4,c="invalid form")},t=>n(1,a=t.detail.target.value),t=>n(2,i=t.detail.target.value),t=>n(3,r=t.detail.target.value)]}class gt extends K{constructor(t){super(),W(this,t,mt,pt,l,{signup:0})}}const ht=R([]),$t={subscribe:ht.subscribe,set:t=>{ht.set(t)},add:t=>{ht.update((e=>[...e,t]))},toggleTodoDone:t=>{ht.update((e=>{let n={...e.find((e=>e.id===t))};n.done=!n.done;const o=e.findIndex((e=>e.id==t));let s=[...e];return s[o]=n,s}))},deleteTodo:t=>{ht.update((e=>e.filter((e=>e.id!==t))))}};function vt(t,e,n){const o=t.slice();return o[15]=e[n],o}function yt(t){let e,n;return{c(){e=u("p"),n=f(t[2]),g(e,"class","error svelte-pfwesm")},m(t,o){r(t,e,o),i(e,n)},p(t,e){4&e&&h(n,t[2])},d(t){t&&c(e)}}}function wt(t){let e;return{c(){e=u("span"),g(e,"class","fas fa-check-square svelte-pfwesm")},m(t,n){r(t,e,n)},d(t){t&&c(e)}}}function bt(t){let e;return{c(){e=u("span"),g(e,"class","fas fa-square svelte-pfwesm")},m(t,n){r(t,e,n)},d(t){t&&c(e)}}}function kt(t){let e,n,o,s;function l(){return t[10](t[15])}return{c(){e=u("span"),n=u("span"),g(n,"class","fas fa-trash svelte-pfwesm"),g(e,"class","controls svelte-pfwesm")},m(t,a){r(t,e,a),i(e,n),o||(s=p(n,"click",l),o=!0)},p(e,n){t=e},d(t){t&&c(e),o=!1,s()}}}function xt(t,e){let n,s,l,a,m,$,v,y,w,b=e[15].title+"";function k(t,e){return t[15].done?wt:bt}let x=k(e),j=x(e);function S(){return e[9](e[15])}let _=e[15].id===e[1]&&kt(e);function q(){return e[12](e[15])}function C(){return e[13](e[15])}return{key:t,first:null,c(){n=u("li"),s=u("span"),j.c(),l=d(),a=u("span"),m=f(b),$=d(),_&&_.c(),v=d(),g(s,"class","completed"),g(a,"class","title"),g(n,"class","svelte-pfwesm"),this.first=n},m(t,o){r(t,n,o),i(n,s),j.m(s,null),i(n,l),i(n,a),i(a,m),i(n,$),_&&_.m(n,null),i(n,v),y||(w=[p(s,"click",S),p(n,"mouseleave",e[11]),p(n,"mouseover",q),p(n,"focus",C)],y=!0)},p(t,o){x!==(x=k(e=t))&&(j.d(1),j=x(e),j&&(j.c(),j.m(s,null))),8&o&&b!==(b=e[15].title+"")&&h(m,b),e[15].id===e[1]?_?_.p(e,o):(_=kt(e),_.c(),_.m(n,v)):_&&(_.d(1),_=null)},d(t){t&&c(n),j.d(),_&&_.d(),y=!1,o(w)}}}function jt(e){let n,s,l,a,f,m,h,v,y,w=[],b=new Map,k=e[2]&&yt(e),x=e[3];const j=t=>t[15].id;for(let t=0;t<x.length;t+=1){let n=vt(e,x,t),o=j(n);b.set(o,w[t]=xt(o,n))}return{c(){n=u("div"),s=u("div"),l=u("input"),a=d(),k&&k.c(),f=d(),m=u("div"),h=u("ul");for(let t=0;t<w.length;t+=1)w[t].c();g(l,"placeholder","enter todo"),g(l,"type","text"),l.autofocus=!0,g(l,"class","svelte-pfwesm"),g(s,"class","todoinput svelte-pfwesm"),g(h,"class","svelte-pfwesm"),g(m,"class","todos svelte-pfwesm"),g(n,"class","todoarea svelte-pfwesm")},m(t,o){r(t,n,o),i(n,s),i(s,l),$(l,e[0]),i(n,a),k&&k.m(n,null),i(n,f),i(n,m),i(m,h);for(let t=0;t<w.length;t+=1)w[t].m(h,null);l.focus(),v||(y=[p(l,"input",e[8]),p(l,"keypress",e[4])],v=!0)},p(t,[e]){1&e&&l.value!==t[0]&&$(l,t[0]),t[2]?k?k.p(t,e):(k=yt(t),k.c(),k.m(n,f)):k&&(k.d(1),k=null),234&e&&(x=t[3],w=function(t,e,n,o,s,l,a,i,r,c,u,f){let d=t.length,p=l.length,m=d;const g={};for(;m--;)g[t[m].key]=m;const h=[],$=new Map,v=new Map;for(m=p;m--;){const t=f(s,l,m),i=n(t);let r=a.get(i);r?o&&r.p(t,e):(r=c(i,t),r.c()),$.set(i,h[m]=r),i in g&&v.set(i,Math.abs(m-g[i]))}const y=new Set,w=new Set;function b(t){J(t,1),t.m(i,u),a.set(t.key,t),u=t.first,p--}for(;d&&p;){const e=h[p-1],n=t[d-1],o=e.key,s=n.key;e===n?(u=e.first,d--,p--):$.has(s)?!a.has(o)||y.has(o)?b(e):w.has(s)?d--:v.get(o)>v.get(s)?(w.add(o),b(e)):(y.add(s),d--):(r(n,a),d--)}for(;d--;){const e=t[d];$.has(e.key)||r(e,a)}for(;p;)b(h[p-1]);return h}(w,e,j,1,t,x,b,h,D,xt,null,vt))},i:t,o:t,d(t){t&&c(n),k&&k.d();for(let t=0;t<w.length;t+=1)w[t].d();v=!1,o(y)}}}function St(t,e,n){let o,s;a(t,Y,(t=>n(14,o=t))),a(t,$t,(t=>n(3,s=t)));let l="",i=null,r=null;function c(t){n(1,i=t)}async function u(t){try{const e={...s.find((e=>e.id===t))};e.done=!e.done,await async function(t,e){const n={method:"PUT",headers:{"Content-Type":"application/json",Authorization:`Bearer ${t}`,Accept:"application/json",Origin:"http://127.0.0.1:5000"},body:JSON.stringify(e)},o=`/todo/${e.id}`,s=await fetch(o,n);if(!s.ok){const t=await s.text();throw console.log(t),t}}(o.authtoken,e);const n=await st(o.authtoken);$t.set(n)}catch(t){n(2,r=t)}}function f(t){$t.deleteTodo(t)}k((async()=>{try{const t=await st(o.authtoken);$t.set(t)}catch(t){n(2,r=t)}}));return[l,i,r,s,async function(t){if(13===t.keyCode&&(t.preventDefault(),0!==l.length)){try{const t=await async function(t,e){const n={method:"POST",headers:{"Content-Type":"application/json",Authorization:`Bearer ${t}`,Accept:"application/json",Origin:"http://127.0.0.1:5000"},body:JSON.stringify(e)},o=await fetch("/todo",n);if(!o.ok){const t=await o.text();throw console.log(t),t}return await o.json()}(o.authtoken,{title:l,effort:1,due:new Date,done:!1});$t.add(t)}catch(t){n(2,r=t)}n(0,l="")}},c,u,f,function(){l=this.value,n(0,l)},t=>u(t.id),t=>f(t.id),()=>n(1,i=null),t=>c(t.id),t=>c(t.id)]}class _t extends K{constructor(t){super(),W(this,t,St,jt,l,{})}}function qt(e){let n;return{c(){n=u("h1"),n.textContent="Unknown state"},m(t,e){r(t,n,e)},i:t,o:t,d(t){t&&c(n)}}}function Ct(t){let e,n;return e=new _t({}),{c(){H(e.$$.fragment)},m(t,o){U(e,t,o),n=!0},i(t){n||(J(e.$$.fragment,t),n=!0)},o(t){B(e.$$.fragment,t),n=!1},d(t){F(e,t)}}}function Ot(t){let e,n;return e=new gt({}),{c(){H(e.$$.fragment)},m(t,o){U(e,t,o),n=!0},i(t){n||(J(e.$$.fragment,t),n=!0)},o(t){B(e.$$.fragment,t),n=!1},d(t){F(e,t)}}}function Et(t){let e,n;return e=new gt({props:{signup:!0}}),{c(){H(e.$$.fragment)},m(t,o){U(e,t,o),n=!0},i(t){n||(J(e.$$.fragment,t),n=!0)},o(t){B(e.$$.fragment,t),n=!1},d(t){F(e,t)}}}function Tt(e){let n;return{c(){n=u("h1"),n.textContent="Signin please"},m(t,e){r(t,n,e)},i:t,o:t,d(t){t&&c(n)}}}function Lt(t){let e,n,o,s;const l=[Tt,Et,Ot,Ct,qt],a=[];function i(t,e){return"loggedout"===t[0].state?0:"signup"===t[0].state?1:"login"===t[0].state?2:"loggedin"===t[0].state?3:4}return n=i(t),o=a[n]=l[n](t),{c(){e=u("div"),o.c(),g(e,"class","main svelte-1vfsecp")},m(t,o){r(t,e,o),a[n].m(e,null),s=!0},p(t,[s]){let r=n;n=i(t),n!==r&&(I(),B(a[r],1,1,(()=>{a[r]=null})),z(),o=a[n],o||(o=a[n]=l[n](t),o.c()),J(o,1),o.m(e,null))},i(t){s||(J(o),s=!0)},o(t){B(o),s=!1},d(t){t&&c(e),a[n].d()}}}function Mt(t,e,n){let o;return a(t,Y,(t=>n(0,o=t))),[o]}class Pt extends K{constructor(t){super(),W(this,t,Mt,Lt,l,{})}}function At(e){let n,o,s,l,a;return o=new ot({}),l=new Pt({}),{c(){n=u("div"),H(o.$$.fragment),s=d(),H(l.$$.fragment)},m(t,e){r(t,n,e),U(o,n,null),i(n,s),U(l,n,null),a=!0},p:t,i(t){a||(J(o.$$.fragment,t),J(l.$$.fragment,t),a=!0)},o(t){B(o.$$.fragment,t),B(l.$$.fragment,t),a=!1},d(t){t&&c(n),F(o),F(l)}}}function Nt(t){return Y.appStarted(),[]}return new class extends K{constructor(t){super(),W(this,t,Nt,At,l,{})}}({target:document.body})}();
//# sourceMappingURL=bundle.js.map
