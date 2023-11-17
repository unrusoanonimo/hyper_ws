function scriptToB64(js) {
    return "data:text/html;base64," + btoa(`<h1></h1><script>e=document.querySelector("h1");${js}</script>`);
}
function displayInfo() {
    const e = document.getElementById("scriptB64info");
    e.hidden = false;
    setTimeout(() => e.hidden = true, 1000);
}
document.addEventListener("DOMContentLoaded", () => {
    const jsCode = document.querySelector("#js-code");
    document.querySelector("#scriptB64").addEventListener("click", () => {
       
        const b64link = scriptToB64(`e.innerHTML=${jsCode.value}`);
        navigator.clipboard.writeText(b64link);
        displayInfo();
    });
    document.querySelector("#scriptB64Update").addEventListener("click", () => {
        const b64link = scriptToB64(`setInterval(()=>e.innerHTML=${jsCode.value})`);
        
        navigator.clipboard.writeText(b64link);
        displayInfo();
    });
})

function fact(n) {
    if (n <= 1) {
        return 1
    }
    return fact(n - 1) * n;
}

function factL(n) {
    let r = 1;
    for (let i = 2; i <= n; i++) {
        r *= i;
    }
    return r;
}