function updateTodoCount() {
    const count = document.querySelectorAll("#todo-list li").length;
    let text = "No todos";
    if (count === 1) {
        text = `${count} todo`;
    }
    if (count > 1) {
        text = `${count} todos`;
    }
    document.getElementById("todo-count").innerText = text;

    document.getElementById("todo-hours").innerText = "";
    fetch("/todos/trends")
        .then((res) => res.json())
        .then((trends) => {
            let text2 = "";
            if (trends.hours === 1) {
                text2 = `${trends.hours} hour`;
            }
            if (trends.hours > 1) {
                text2 = `${trends.hours} hours`;
            }
            document.getElementById("todo-hours").innerText = text2;
        })
        .catch((err) => {
            console.error("error in getting trends", err);
        });
}

function formatTags(todoId) {
    let tooltip = document.getElementById(`tooltip-content-${todoId}`);
    try {
        let tags = JSON.parse(tooltip.innerText);
        if (tags.length === 0) {
            tooltip.innerHTML = "<div class='py-1 text-gray-300 italic'>No tags available</div>";
        } else {
            tooltip.innerHTML = tags.map((tag) => `<div>${tag}</div>`).join("");
        }
    } catch (e) {
        console.error("Failed to parse tags:", e);
    }
}

// Expose functions to the global scope
window.updateTodoCount = updateTodoCount;
window.formatTags = formatTags;

// Run on page load
document.addEventListener("DOMContentLoaded", updateTodoCount);
document.body.addEventListener("htmx:afterRequest", updateTodoCount);
