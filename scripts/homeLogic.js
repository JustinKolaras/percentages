const newButton = document.getElementById("new_btn");

function onNewButtonClick() {
    // Duplicate the div `input_segment`
    const inputForm = document.getElementById("input_form");
    const segmentNode = document.getElementById("input_segment");

    const clone = segmentNode.cloneNode(true);

    // Reset the text of the clone and increment their name.
    const equationInput = clone.getElementsByTagName("input").equation_input;
    equationInput.value = "";

    inputForm.insertBefore(clone, newButton);
}

newButton.onclick = onNewButtonClick;
