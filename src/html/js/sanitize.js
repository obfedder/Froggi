function handleSecureInput(event) {
    const inputElement = event.target;
    const sanitized = sanitizeInput(inputElement.value); 
    inputElement.value = sanitized; 
}

function sanitizeInput(input) {
    // Replace special characters with their encoded equivalents
    return input.replace(/[<>"'\/]/g, (char) => {
        const charMap = { '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;', '/': '&#x2F;' };
        return charMap[char];
    });
}