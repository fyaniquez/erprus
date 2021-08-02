/**
 * funciones javascript para manipular selects
 * fyaniquez
 * 29/03/2021
 */
/**
* limpia el selectbox y carga las opciones
* @param selectbox que se alimentara con la lista
* @param opciones lista de opciones 
*/
export const cargar_select = (selectbox, opciones) => {
  selectbox.innerHTML = "";
  opciones.forEach(element => {
    let option = document.createElement("option");
    option.value = element.id;
    option.innerHTML = element.nombre;
    selectbox.options.add(option);
  });
}
/**
 * obtiene objetos de la acción REST(url) y actualiza el selectbox con la informacion 
 * @param selectbox que se alimentara con la lista
 * @param url acción REST relativa
 */
export const actualizar_select = async (selectbox, url) => {
  await fetch(url).then(async (response) => {
    let opciones = await response.json();
    cargar_select(selectbox, opciones);
  }).catch(function (error) {
    selectbox.innerHTML = "";
  });
}