/*
 * purpose: nested es una libreria para manerajr tablas anidadas
 * author: fyaniquez
 * date: 22/11/2020
 */
export default class Nested {
  constructor(maestro, detalles, campos) {
    this.campos = campos;
    this.maestro = maestro;
    this.detalles = detalles;
  }
  /*
   * purpose obtiene un elemento en la fila y la columna indicadas
   *
   */
  getElemento(nombre, fila_actual) {
    let id = `${this.maestro}_${this.detalles}_attributes_${fila_actual}_${nombre}`;
    let elemento = document.getElementById(id);
    return elemento;
  }
 /*
 * purpose: crea un widget para una columna de la tabla de detalles
 * parameters:
 * widget: select checkbox hidden input
 * id: id de la nueva fila generada con time
 * campo: columna para la que se crea el wiget
 * returns: widget creado de tipo nodo para agregarse c/appendChild
 */  
  crear_widget(id, campo) {
    let id_orig = `${this.maestro}_${this.detalles}_attributes_0_${campo}`;
    let orig = document.getElementById(id_orig);
    if (!orig) return null;
    let nuevo = orig.cloneNode(true);
    if (nuevo.name)
      nuevo.name = `${this.maestro}[${this.detalles}_attributes][${id}][${campo}]`;
    nuevo.id = `${this.maestro}_${this.detalles}_attributes_${id}_${campo}`;
    nuevo.value = '';
    nuevo.onchange = orig.onchange;
    nuevo.onkeyup = orig.onkeyup;
    return nuevo;
  }

/*
 * purpose: agrega nueva fila a la tabla de detalle
 * parameters: e : objeto que generó el evento
 */
  agregar_fila() {
    let p = document.getElementById(this.detalles);
    let idFila = (new Date).getTime();
    for( var fila in this.campos ) {
      let widget = this.crear_widget( idFila, fila )
      if (!widget) continue;
      if (this.campos[fila].hidden) {        
        widget.type = 'hidden';
        widget.value = '';
        p.appendChild(widget);
      } else {
        let option = document.createElement("div");
        option.className = "col";
        option.appendChild( widget );
        p.appendChild(option);        
      }
    }
  }
}
