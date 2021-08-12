import { actualizar_select } from './modulos/select.ts';

/**
 * slected
 * @param {objeto select analizado} 
 * @returns literal visible en el select
 */
const selected = (select) => {
    if (select.options.length > 1)
        return select.options[select.selectedIndex].innerHTML;
    return "";
}

const actualizar_nombre = () => {
    let nombre = document.getElementById("nombre");
    nombre.value = `${selected(form.categoria_id)} \
    ${selected(form.marca_id)} ${selected(form.unidad_id)} \
    ${form.contenido.value} ${form.caracteristicas.value}`;
}

const actualizar_nombre_unidad = () => {
    let nombre_unidad = document.getElementById("nombre_unidad");
    nombre_unidad.innerHTML = form.unidad_id.options[form.unidad_id.selectedIndex].dataset.sigla;
}
/**
 * actualiza los dos selects que dependen de capitulo: categoría y marca
 * @param selectbox 
 * @param url 
 */
const onChange_capitulo = async (e) => {
    await actualizar_select(form.categoria_id, `/erprus/capitulos/${e.target.value}/categorias_nombres.json`);
    if (form.categoria_id[0]) {
        await actualizar_select(form.marca_id, `/erprus/categorias/${form.categoria_id[0].value}/marcas_nombres.json`);
    }
    actualizar_nombre();
}

const onChange_categoria = async (e) => {
    await actualizar_select(form.marca_id, `/erprus/categorias/${e.target.value}/marcas_nombres.json`);
    actualizar_nombre();
}

const onChange_unidad = (e) => {
    actualizar_nombre();
    actualizar_nombre_unidad();
}
const onLoad = () => {
    // formulario principal
    window.form = document.getElementById("mainform");

    // listeners para actualizar selects y nombre
    form.capitulo_id.addEventListener("change", onChange_capitulo);
    form.categoria_id.addEventListener("change", onChange_categoria);
    form.marca_id.addEventListener("change", actualizar_nombre);
    form.unidad_id.addEventListener("change", onChange_unidad);
    form.contenido.addEventListener("change", actualizar_nombre);
    form.caracteristicas.addEventListener("change", actualizar_nombre);

    // valor inicial del nombre
    var nombre = document.getElementById("nombre");
    if (nombre.value == "") actualizar_nombre();

    // valor inicial del nombre_unidad
    var nombre_unidad = document.getElementById("nombre_unidad");
    if (nombre_unidad.innerHTML == "") actualizar_nombre_unidad();

    // registra eventos en los botones

}

document.readyState === "complete" ? onLoad() : addEventListener("load", onLoad);
