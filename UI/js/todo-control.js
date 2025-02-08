/*document.addEventListener("alpine:init", () => {
    Alpine.data("updateComponent", (type, value, tooltipText, decrement, increment, id) => ({
        showTooltip: false,
        timeout: null,
        type, value, tooltipText, decrement, increment, id,
        html() {
            return `
                <span x-data="{ showTooltip: false, timeout: null }"
                      @mouseenter="timeout = setTimeout(() => showTooltip = true, 500)" 
                      @mouseleave="clearTimeout(timeout); showTooltip = false"
                      class="relative flex items-center justify-center gap-1 
                             text-xs font-thin px-2 py-1 bg-white 
                             border border-gray-500 w-auto cursor-pointer
                             hover:bg-gray-500 hover:text-white rounded-md"
                >

                    <i class="fa-solid fa-minus cursor-pointer"
                       hx-post="/todos/${id}/update-${type}"
                       hx-trigger="click"
                       hx-target="#${type}-${id}"
                       hx-vals='{"change": ${decrement}}'
                       hx-on::after-request="
                            const updated = document.getElementById('${type}-${id}');
                            const mirror = document.getElementById('${type}2-${id}');
                            if (mirror) mirror.innerText = updated.innerText;
                        ">
                    </i>

                    <span id="${type}-${id}">${value}</span>

                    <i class="fa-solid fa-plus cursor-pointer"
                       hx-post="/todos/${id}/update-${type}"
                       hx-trigger="click"
                       hx-target="#${type}-${id}"
                       hx-vals='{"change": ${increment}}'
                       hx-on::after-request="
                            const updated = document.getElementById('${type}-${id}');
                            const mirror = document.getElementById('${type}2-${id}');
                            if (mirror) mirror.innerText = updated.innerText;
                        ">
                    </i>

                    <!-- Tooltip Positioned Below -->
                    <div x-show="showTooltip" 
                         class="absolute top-full left-1/2 -translate-x-1/2 mt-1 
                                w-auto min-w-max text-[10px] text-white bg-gray-500 
                                rounded-md px-3 py-1 opacity-90 shadow-md z-50"
                         x-transition.opacity.duration.200ms>
                        ${tooltipText}
                    </div>

                </span> 
            `;
        }
    }));
});*/

function formatTags(todoId, event) {
    if (event.detail.requestConfig.verb !== "get") {
        return;
    }

    let tooltip = document.getElementById(`tooltip-content-${todoId}`);
    try {
        let tags = JSON.parse(tooltip.innerText);

        if (tags.length === 0) {
            tooltip.innerHTML = `<div class='py-1 text-gray-300 italic'>
                No tags available
            </div>`;
            return;
        }

        tooltip.innerHTML = tags.map(tag => {
            let encodedTag = encodeURIComponent(tag);

            return `
                <div id="tooltip-tag-${encodedTag}" class="tooltip-item">
                    <span class="truncate">${tag}</span>
                    <button hx-delete="/todos/${todoId}/tags/${encodedTag}"
                            hx-trigger="click"
                            hx-swap="none"
                            hx-on::after-request="
                                document
                                .getElementById('tooltip-tag-${encodedTag}')
                                .remove();
                            "
                            class="ml-2 p-0.5 px-1 text-xs bg-gray-200 rounded 
                                   text-red-500 hover:bg-red-300 hover:text-red-700 
                                   transition cursor-pointer">
                        &times;
                    </button>
                </div>
            `;
        }).join("");

        // Ensure HTMX processes newly added delete buttons
        htmx.process(tooltip);

    } catch (e) {
        console.error("Failed to parse tags:", e);
    }
}

window.formatTags = formatTags;
