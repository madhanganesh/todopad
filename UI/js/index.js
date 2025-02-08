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


// Expose functions to the global scope
window.updateTodoCount = updateTodoCount;

// Run on page load
document.addEventListener("DOMContentLoaded", updateTodoCount);
document.body.addEventListener("htmx:afterRequest", updateTodoCount);
