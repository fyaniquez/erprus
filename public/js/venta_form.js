import "./modulos/detallecv.js";

const onLoad = () => {
    window.form = document.forms[0];
    window.detform = document.getElementById("detform");
    detform.addEventListener('change', onChange_detform);
    detform.addEventListener('keyup', onKeyup_detform);
    detform.addEventListener('click', onClick_detform);
    document.getElementById("agregar").addEventListener('click', onClick_agregar)
}

document.readyState === "complete" ? onLoad() : addEventListener("load", onLoad);