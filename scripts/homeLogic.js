const newButton = document.getElementById("new_btn");

function onNewButtonClick() {
    // Duplicate the div `input_segment`
    const inputForm = document.getElementById("input_form");
    const segmentNode = document.getElementById("input_segment");

    const clone = segmentNode.cloneNode(true);
    inputForm.insertBefore(clone, newButton);
}

newButton.onclick = onNewButtonClick;
