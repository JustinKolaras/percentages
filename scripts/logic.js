const newButton = document.getElementById("new_btn");

let equationCount = 1;

function onNewButtonClick() {
    // Duplicate the div `input_segment`
    const inputForm = document.getElementById("input_form");
    const segmentNode = document.getElementById("input_segment");

    const clone = segmentNode.cloneNode(true);

    // Reset the text of the clone.
    const equationLabel = clone.getElementsByTagName("label").equation_label;
    const equationInput = clone.getElementsByTagName("input").equation_input;
    equationLabel.innerHTML = `Input equation #${++equationCount}`;
    equationInput.value = "";

    inputForm.insertBefore(clone, newButton);
}

newButton.onclick = onNewButtonClick;
