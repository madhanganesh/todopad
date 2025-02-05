const TagManager = (() => {
    let tags = [];
    let allTags = [];
    let selectedIndex = -1;

    function init() {
        const tagInput = document.getElementById("tag-input");
        const tagContainer = document.getElementById("tag-container");
        const tagsHidden = document.getElementById("tags-hidden");
        const tagSuggestions = document.getElementById("tag-suggestions");

        tags = tagsHidden.value ? tagsHidden.value.split(",").map(t => t.trim()).filter(t => t.length > 0) : [];

        renderTags();
        fetchTags();

        tagInput.addEventListener("input", () => filterSuggestions(tagInput.value));
        tagInput.addEventListener("keydown", (event) => handleKeyDown(event, tagInput));

        document.addEventListener("click", (event) => {
            if (!tagContainer.contains(event.target) && !tagSuggestions.contains(event.target)) {
                tagSuggestions.classList.add("hidden");
            }
        });
    }

    function renderTags() {
        const tagContainer = document.getElementById("tag-container");
        const tagInput = document.getElementById("tag-input");
        const tagsHidden = document.getElementById("tags-hidden");

        tagContainer.innerHTML = "";
        tags.forEach(tag => {
            let chip = document.createElement("div");
            chip.className = "bg-gray-200 text-sm px-2 py-1 m-1 rounded flex items-center";
            chip.innerHTML = `${tag} <span class="ml-2 cursor-pointer text-red-500" onclick="TagManager.removeTag('${tag}')">&times;</span>`;
            tagContainer.appendChild(chip);
        });
        tagContainer.appendChild(tagInput);
        tagsHidden.value = tags.join(",");
    }

    function removeTag(tag) {
        tags = tags.filter(t => t !== tag);
        renderTags();
    }

    function filterSuggestions(query) {
        const tagInput = document.getElementById("tag-input");
        const tagSuggestions = document.getElementById("tag-suggestions");

        let matches = allTags.filter(tag => tag.toLowerCase().includes(query.toLowerCase()));
        showSuggestions(matches, tagInput, tagSuggestions);
    }

    function showSuggestions(matches, tagInput, tagSuggestions) {
        tagSuggestions.innerHTML = "";
        selectedIndex = -1;

        if (matches.length === 0) {
            tagSuggestions.classList.add("hidden");
            return;
        }

        tagSuggestions.classList.remove("hidden");
        tagSuggestions.style.width = `${tagInput.offsetWidth}px`;

        matches.forEach((tag, index) => {
            let suggestion = document.createElement("li");
            suggestion.className = "px-2 py-1 cursor-pointer hover:bg-gray-100";
            suggestion.textContent = tag;
            suggestion.setAttribute("data-index", index);

            suggestion.addEventListener("click", () => selectTag(tag));
            tagSuggestions.appendChild(suggestion);
        });
    }

    function handleKeyDown(event, tagInput) {
        const tagSuggestions = document.getElementById("tag-suggestions");
        let suggestions = tagSuggestions.querySelectorAll("li");

        if (event.key === "ArrowDown" && suggestions.length > 0) {
            event.preventDefault();
            if (selectedIndex < suggestions.length - 1) {
                selectedIndex++;
            }
            updateSuggestionHighlight(suggestions);
        } else if (event.key === "ArrowUp" && suggestions.length > 0) {
            event.preventDefault();
            if (selectedIndex > 0) {
                selectedIndex--;
            }
            updateSuggestionHighlight(suggestions);
        } else if (event.key === "Enter") {
            event.preventDefault();
            if (selectedIndex >= 0 && selectedIndex < suggestions.length) {
                selectTag(suggestions[selectedIndex].textContent);
            } else {
                addNewTag(tagInput.value.trim());
            }
            tagInput.value = "";
            tagSuggestions.classList.add("hidden");
            tagInput.focus();
        } else if (event.key === "Backspace" && tagInput.value === "") {
            tags.pop();
            renderTags();
        }
    }

    function updateSuggestionHighlight(suggestions) {
        suggestions.forEach((s, i) => {
            if (i === selectedIndex) {
                s.classList.add("bg-gray-300");
            } else {
                s.classList.remove("bg-gray-300");
            }
        });
    }

    function selectTag(tag) {
        if (!tags.includes(tag)) {
            tags.push(tag);
            renderTags();
        }
        document.getElementById("tag-input").value = "";
        document.getElementById("tag-suggestions").classList.add("hidden");
    }

    function addNewTag(tag) {
        if (tag && !tags.includes(tag)) {
            tags.push(tag);
            renderTags();
        }
    }

    function fetchTags() {
        fetch("/tags")
            .then(response => response.json())
            .then(data => { allTags = data.tags || []; })
            .catch(error => console.error("Error fetching tags:", error));
    }

    return {
        init,
        removeTag
    };
})();

// ✅ Initialize the TagManager when the page loads
document.addEventListener("DOMContentLoaded", () => TagManager.init());
