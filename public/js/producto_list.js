
const borrar_fila = async (fila) => {
    conf = prompt(`¿Esta seguro de eliminar el producto: ${fila}?`);
    if (conf == null) return;
    const response = await fetch(`/erprus/productos/${fila}`, {
        method: 'DELETE'
    });
    const data = await response.json();
    alert(data);
}
const get_longitud = () => {
    let long = document.getElementById("longitud");
    return long.options[long.selectedIndex].value;
}
const ir_a = (pagina, filtro, longitud) => {
    let url = `/erprus/productos?pagina=${pagina}&filtro=${filtro}&longitud=${longitud}`;
    window.location.replace(encodeURI(url));
}
const onClick_reporte = (e) => {
    if (e.target.className && e.target.className == "btnrojo") {
        borrar_fila(e.target.parentNode.ariaRowIndex);
    }
}
const onClick_pagebar = (e) => {
    if (e.target.className && e.target.className == "page") {
        let pagina = e.target.innerText;
        let filtro = document.getElementById("filtro").value;
        let long = document.getElementById("longitud");
        let longitud = long.options[long.selectedIndex].value;
        if (isNaN(pagina)) {
            switch (pagina) {
                case ">":
                    pagina = +document.getElementById("actual").innerText + 1;
                    break;
                case "<":
                    pagina = +document.getElementById("actual").innerText - 1;
                    break;
                case "<<":
                    pagina = "1";
                    break;
                case ">>":
                    pagina = e.target.dataset.index;
                    break;
            }
        }
        window.location.replace(encodeURI(`/erprus/productos?pagina=${pagina}&filtro=${filtro}&longitud=${longitud}`));
    }
}
const onChange_longitud = (e) => {
    let pagina = "1";
    let filtro = document.getElementById("filtro").value;
    let longitud = e.target.options[e.target.selectedIndex].value;
    window.location.href = encodeURI(`/erprus/productos?pagina=${pagina}&filtro=${filtro}&longitud=${longitud}`);
}
const onChange_filtro = (e) => {
    let pagina = "1";
    let filtro = e.target.value;
    let long = document.getElementById("longitud");
    let longitud = long.options[long.selectedIndex].value;
    window.location.href = encodeURI(`/erprus/productos?pagina=${pagina}&filtro=${filtro}&longitud=${longitud}`);
}
const onclick_primero = (e) => {
    ir_a("1", document.getElementById("filtro").value, get_longitud());
}
const onclick_ultimo = (e) => {
    let pagina = document.getElementById("ultimo").dataset.index;
    ir_a(pagina, document.getElementById("filtro").value, get_longitud());
}
const onclick_siguiente = (e) => {
    let pagina = +document.getElementById("actual").innerText + 1;
    ir_a(pagina.toString(), document.getElementById("filtro").value, get_longitud());
}
const onclick_previo = (e) => {
    let pagina = +document.getElementById("actual").innerText - 1;
    ir_a(pagina, document.getElementById("filtro").value, get_longitud());
}
const onLoad = () => {
    // registra eventos en los botones
    reporte.addEventListener("click", onClick_reporte);
    pagebar.addEventListener("click", onClick_pagebar);
    document.getElementById("longitud").addEventListener("change", onChange_longitud);
    document.getElementById("filtro").addEventListener("change", onChange_filtro);
    /*
    document.getElementById("primero").addEventListener("click", onclick_primero);
    document.getElementById("ultimo").addEventListener("click", onclick_ultimo);
    document.getElementById("previo").addEventListener("click", onclick_previo);
    document.getElementById("siguiente").addEventListener("click", onclick_siguiente);
    */
}

document.readyState === "complete" ? onLoad() : addEventListener("load", onLoad);