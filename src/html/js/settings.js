document.addEventListener('DOMContentLoaded', function() {
    const toggleButton = document.getElementById('toggle-keybinds');
    const keybindList = document.getElementById('keybind-list');
  
    toggleButton.addEventListener('click', function() {
      const isVisible = keybindList.style.display !== 'none';
      keybindList.style.display = isVisible ? 'none' : 'block';
      toggleButton.textContent = isVisible ? 'Show Keybinds' : 'Hide Keybinds';
    });
  });