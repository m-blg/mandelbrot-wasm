console.log("js_test");


const canvas = document.getElementById('canvas');

async function init_ifields() {
    await sleep(100);
    var width = canvas.width;
    var height = canvas.height; 

    const ifield_width = document.getElementById('ifield_width');
    const ifield_height = document.getElementById('ifield_height');

    // console.log(width);
    // console.log(height);
    ifield_width.value = width;
    ifield_height.value = height;
}

function apply_button_callback() {
    console.log("clicked2");
    canvas.width = document.getElementById('ifield_width').value;
    canvas.height = document.getElementById('ifield_height').value;
    canvas.style.width = String(canvas.width) + "px";
    canvas.style.height = String(canvas.height) + "px";
}
function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

init_ifields();