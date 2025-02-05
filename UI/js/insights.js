const ChartManager = (() => {
    let effortChart = null; // Store Chart instance

    function updateEditLink() {
        let insightId = document.getElementById("insight").value;
        let editLink = document.getElementById("edit-insight-link");
        editLink.href = insightId ? `/insights/edit/${insightId}` : "#";
    }

    function resetChartCanvas() {
        let canvasParent = document.getElementById("chart-container");
        let newCanvas = document.createElement("canvas");
        newCanvas.id = "effortChart";
        canvasParent.innerHTML = ""; // Clear previous canvas
        canvasParent.appendChild(newCanvas);
    }

    function updateChart() {
        let period = document.getElementById("period").value;
        let insightId = document.getElementById("insight").value;

        fetch(`/insights/${insightId}/data?period=${period}`)
            .then(response => response.json())
            .then(data => {
                resetChartCanvas();
                let ctx = document.getElementById("effortChart").getContext("2d");

                if (effortChart !== null) {
                    effortChart.destroy();
                }

                let chartContainer = document.getElementById("chart-container");
                let isMobile = window.innerWidth < 768;
                let fontSize = isMobile ? 10 : 14;

                let chartConfig = ChartManager.getChartConfig(data, fontSize, isMobile);
                effortChart = new Chart(ctx, chartConfig);
            })
            .catch(error => console.error("Error updating chart:", error));
    }

    function getChartConfig(data, fontSize, isMobile) {
        let chartType = data.chart_type;
        let labels, datasets, scales, options;

        // ✅ Adjust chart height based on type
        let chartContainer = document.getElementById("chart-container");
        chartContainer.style.height = chartType === "pie" ? "400px" : "500px";

        if (chartType === "pie") {
            labels = Object.keys(data.data_sets);
            datasets = [{
                label: "Distribution",
                data: labels.map(label => data.data_sets[label].reduce((a, b) => a + b, 0)),
                borderWidth: 1,
            }];
            let totalEffort = datasets[0].data.reduce((a, b) => a + b, 0);

            scales = { x: { display: false }, y: { display: false } };
            options = getPieChartOptions(totalEffort, fontSize);
        } else {
            labels = data.labels;
            datasets = Object.keys(data.data_sets).map(key => ({
                label: key,
                data: data.data_sets[key],
                tension: 0.3,
                borderWidth: 3,
            }));

            let maxEffort = Math.max(...datasets.flatMap(d => d.data));
            let stepSize = Math.ceil(maxEffort / 5);

            scales = getBarLineChartScales(fontSize, stepSize, maxEffort);
            options = getBarLineChartOptions(fontSize, isMobile);
        }

        return {
            type: chartType,
            data: { labels: labels, datasets: datasets },
            options: options,
            plugins: [ChartDataLabels],
        };
    }

    function getPieChartOptions(totalEffort, fontSize) {
        return {
            responsive: true,
            maintainAspectRatio: false,
            layout: { padding: { top: 20, bottom: 10 } },
            elements: { arc: { borderWidth: 1, radius: window.innerWidth < 768 ? "40%" : "60%" } },
            plugins: {
                datalabels: {
                    display: true,
                    color: "white",
                    font: { weight: "bold", size: fontSize },
                    formatter: (value) => {
                        let percentage = ((value / totalEffort) * 100).toFixed(1);
                        return `${value} hrs\n(${percentage}%)`;
                    }
                },
                legend: {
                    display: true,
                    position: "bottom",
                    labels: { font: { size: fontSize, weight: "bold" } }
                }
            }
        };
    }

    function getBarLineChartScales(fontSize, stepSize, maxEffort) {
        return {
            x: {
                beginAtZero: true,
                title: { display: true, text: "Time Period", font: { size: fontSize, weight: "bold" } },
                ticks: { font: { size: fontSize } }
            },
            y: {
                beginAtZero: true,
                title: { display: true, text: "Effort (Hours)", font: { size: fontSize, weight: "bold" } },
                ticks: {
                    font: { size: fontSize },
                    stepSize: stepSize,
                    suggestedMax: maxEffort + stepSize
                }
            }
        };
    }

    function getBarLineChartOptions(fontSize, isMobile) {
        return {
            responsive: true,
            maintainAspectRatio: false,
            layout: { padding: { top: 40, left: 10, right: 10, bottom: 20 } },
            scales: getBarLineChartScales(fontSize, 5, 100),
            plugins: {
                datalabels: {
                    display: true,
                    align: "top",
                    color: "black",
                    font: { weight: "thin", size: fontSize },
                    formatter: (value) => `${value}`
                },
                legend: {
                    display: true,
                    position: "bottom",
                    align: "start",
                    labels: {
                        font: { size: fontSize, weight: "bold" },
                        padding: isMobile ? 10 : 20,
                        usePointStyle: true,
                        pointStyle: "line",
                    }
                }
            }
        };
    }

    // ✅ Public API (Expose Functions)
    return {
        updateChart,
        updateEditLink,
        getChartConfig,
    };
})();

// ✅ Attach Events & Expose Functions Globally
document.addEventListener("DOMContentLoaded", () => {
    ChartManager.updateChart();
    ChartManager.updateEditLink();
});

document.getElementById("insight").addEventListener("change", ChartManager.updateEditLink);

// ✅ Recalculate chart when window resizes
window.addEventListener("resize", ChartManager.updateChart);

// ✅ Expose to window (for HTML events)
window.ChartManager = ChartManager;
