const totalizar = () => {
    let subtots = document.getElementsByTagName("input");
    let total = 0.0;
    for (var i = 0; i < subtots.length; i++) {
        let id = subtots[i].id;
        if (id.indexOf('subtotal') > 0)
            total += +subtots[i].value;
    };
    form.compra_total.value = total;
}
const onclick_producto = (e) => {
    let p = e.target;
    let origen = p.dataset.origen;
    let producto_id = document.getElementById(`producto_id_${origen}`);
    producto_id.value = p.dataset.id;
    let nombre = document.getElementById(`nombre_${origen}`);
    nombre.value = p.dataset.nombre;
    let precio = document.getElementById(`precio_${origen}`);
    precio.value = p.dataset.precio;
    document.getElementById(`cantidad_${origen}`).focus();
}

function agregar_fila_tsearch(tsearch, producto, origen) {
    let option = document.createElement("li");
    option.appendChild(document.createTextNode(producto.nombre));
    option.setAttribute('data-origen', origen);
    option.setAttribute('data-id', producto.id);
    option.setAttribute('data-nombre', producto.nombre);
    option.setAttribute('data-precio', producto.precio);
    option.addEventListener('click', (e) => onclick_producto(e));
    tsearch.appendChild(option);
}

const onSuccess_find_productos = (productos, origen) => {
    if (productos.length == 1)
        onSuccess_find_producto(productos[0], origen);
    let tsearch = document.getElementById('tsearch');
    tsearch.innerHTML = '';
    productos.forEach(producto => agregar_fila_tsearch(tsearch, producto, origen));
}
const onSuccess_find_producto = (producto, origen) => {
    let nombre = document.getElementById(`nombre_${origen}`);
    nombre.value = producto.nombre;
    let precio = document.getElementById(`precio_${origen}`);
    precio.value = producto.precio;
    document.getElementById(`cantidad_${origen}`).focus();
}
const onChange_producto_id = (producto_id, origen) => {
    let url = `/erprus/precios/${producto_id}`;
    fetch(url, {
        headers: {
            'Content-Type': 'application/json'
        }
    }).then(async (response) => {
        const productos = await response.json();
        onSuccess_find_producto(productos[0], origen);
    }).catch(function (error) {
        alert('no se pudo obtener el producto por: ' + error.message);
    });
}

const onChange_cantidad = (cantidad, origen) => {
    let subtotal = document.getElementById(`subtotal_${origen}`);
    let precio = document.getElementById(`precio_${origen}`);
    subtotal.value = +cantidad * +precio.value;
    totalizar();
}

const onChange_detform = (e) => {
    let id = e.target.id;
    let valor = e.target.value.trim();
    if (isNaN(valor)) return;
    let origen = id.match(/[0-9]+/);
    if (/^producto_id/.test(id))
        onChange_producto_id(valor, origen);
    else if (/^cantidad/.test(id))
        onChange_cantidad(valor, origen);
}

const onKeyup_detform = (e) => {
    let id = e.target.id;
    if (!/^nombre/.test(id)) return;
    let nombre = e.target.value.trim();
    if (nombre == '') return;
    let origen = id.match(/[0-9]+/);
    let url = `/erprus/precios?filtro=${nombre}&longitud=10&pagina=1`;
    fetch(url, {
        headers: {
            'Content-Type': 'application/json'
        }
    }).then(async (response) => {
        const productos = await response.json();
        onSuccess_find_productos(productos, origen);
    }).catch(function (error) {
        alert('no se pudo obtener el producto por: ' + error.message);
    });
}

const onClick_detform = (e) => {
    let id = e.target.className;
    if (id != "btnrojo") return;
    let fila = e.target.parentNode.parentNode;
    if (fila.id == "detalle_0") return;
    fila.parentNode.removeChild(fila);
}

const onClick_agregar = (e) => {
    let fila = document.getElementById("detalle_0");
    let time = new Date().getTime().toString();
    let regexp = new RegExp("0", 'g');
    let newFields = fila.outerHTML.replace(regexp, time);
    fila.parentNode.insertAdjacentHTML('beforeend', newFields);
}

const onLoad = () => {
    window.form = document.forms[0];
    window.detform = document.getElementById("detform");
    detform.addEventListener('change', onChange_detform);
    detform.addEventListener('keyup', onKeyup_detform);
    detform.addEventListener('click', onClick_detform);
    document.getElementById("agregar").addEventListener('click', onClick_agregar)
}

document.readyState === "complete" ? onLoad() : addEventListener("load", onLoad);
