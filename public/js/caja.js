const onDescripcionInvalid = (textbox) => {
    if (textbox.value === '') {
        textbox.setCustomValidity('Es obligatorio registrar una descripción');
    } else {
        textbox.setCustomValidity('');
    }
    return true;
}
const onSubmit = () => {
    let desc = form.descripcion;
    if (desc.value.trim() === "") {
        alert("Registrar una descripción es obligatorio");
        desc.focus();
        return false;
    }
}
const onLoad = () => {
    window.form = document.forms[0];
    form.addEventListener('submit', onSubmit);
    form.descripcion.addEventListener('invalid', onDescripcionInvalid);
}
document.readyState === "complete" ? onLoad() : addEventListener("load", onLoad);