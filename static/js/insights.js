(()=>{var l=(()=>{let c=null;function p(){let t=document.getElementById("insight").value,e=document.getElementById("edit-insight-link");e.href=t?`/insights/edit/${t}`:"#"}function f(){let t=document.getElementById("chart-container"),e=document.createElement("canvas");e.id="effortChart",t.innerHTML="",t.appendChild(e)}function m(){let t=document.getElementById("period").value,e=document.getElementById("insight").value;fetch(`/insights/${e}/data?period=${t}`).then(n=>n.json()).then(n=>{f();let r=document.getElementById("effortChart").getContext("2d");c!==null&&c.destroy();let o=document.getElementById("chart-container"),a=window.innerWidth<768,h=a?10:14,s=l.getChartConfig(n,h,a);c=new Chart(r,s)}).catch(n=>console.error("Error updating chart:",n))}function b(t,e,n){let r=t.chart_type,o,a,h,s,E=document.getElementById("chart-container");if(E.style.height=r==="pie"?"400px":"500px",r==="pie"){o=Object.keys(t.data_sets),a=[{label:"Distribution",data:o.map(d=>t.data_sets[d].reduce((i,w)=>i+w,0)),borderWidth:1}];let u=a[0].data.reduce((d,i)=>d+i,0);h={x:{display:!1},y:{display:!1}},s=y(u,e)}else{o=t.labels,a=Object.keys(t.data_sets).map(i=>({label:i,data:t.data_sets[i],tension:.3,borderWidth:3}));let u=Math.max(...a.flatMap(i=>i.data)),d=Math.ceil(u/5);h=g(e,d,u),s=C(e,n)}return{type:r,data:{labels:o,datasets:a},options:s,plugins:[ChartDataLabels]}}function y(t,e){return{responsive:!0,maintainAspectRatio:!1,layout:{padding:{top:20,bottom:10}},elements:{arc:{borderWidth:1,radius:window.innerWidth<768?"40%":"60%"}},plugins:{datalabels:{display:!0,color:"white",font:{weight:"bold",size:e},formatter:n=>{let r=(n/t*100).toFixed(1);return`${n} hrs
(${r}%)`}},legend:{display:!0,position:"bottom",labels:{font:{size:e,weight:"bold"}}}}}}function g(t,e,n){return{x:{beginAtZero:!0,title:{display:!0,text:"Time Period",font:{size:t,weight:"bold"}},ticks:{font:{size:t}}},y:{beginAtZero:!0,title:{display:!0,text:"Effort (Hours)",font:{size:t,weight:"bold"}},ticks:{font:{size:t},stepSize:e,suggestedMax:n+e}}}}function C(t,e){return{responsive:!0,maintainAspectRatio:!1,layout:{padding:{top:40,left:10,right:10,bottom:20}},scales:g(t,5,100),plugins:{datalabels:{display:!0,align:"top",color:"black",font:{weight:"thin",size:t},formatter:n=>`${n}`},legend:{display:!0,position:"bottom",align:"start",labels:{font:{size:t,weight:"bold"},padding:e?10:20,usePointStyle:!0,pointStyle:"line"}}}}}return{updateChart:m,updateEditLink:p,getChartConfig:b}})();document.addEventListener("DOMContentLoaded",()=>{l.updateChart(),l.updateEditLink()});document.getElementById("insight").addEventListener("change",l.updateEditLink);window.addEventListener("resize",l.updateChart);window.ChartManager=l;})();
