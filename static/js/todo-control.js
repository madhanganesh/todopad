(()=>{function i(n,a){if(a.detail.requestConfig.verb!=="get")return;let t=document.getElementById(`tooltip-content-${n}`);try{let e=JSON.parse(t.innerText);if(e.length===0){t.innerHTML=`<div class='py-1 text-gray-300 italic'>
                No tags available
            </div>`;return}t.innerHTML=e.map(r=>{let o=encodeURIComponent(r);return`
                <div id="tooltip-tag-${o}" class="tooltip-item">
                    <span class="truncate">${r}</span>
                    <button hx-delete="/todos/${n}/tags/${o}"
                            hx-trigger="click"
                            hx-swap="none"
                            hx-on::after-request="
                                document
                                .getElementById('tooltip-tag-${o}')
                                .remove();
                            "
                            class="ml-2 p-0.5 px-1 text-xs bg-gray-200 rounded 
                                   text-red-500 hover:bg-red-300 hover:text-red-700 
                                   transition cursor-pointer">
                        &times;
                    </button>
                </div>
            `}).join(""),htmx.process(t)}catch(e){console.error("Failed to parse tags:",e)}}window.formatTags=i;})();
