const PING_TIME = 1000;
const DEFAULT_THEME = 'theme-dark';
const LOCK_INTERFACE_BTN_ID = 'lockInterfaceBtn';
const POPUP_DURATION = 7500;
const PING_URL = '/';
const PREVIEW_URL = '/overlay';

const pingValue = document.getElementById('ping-value');
const lockInterfaceBtn = document.getElementById(LOCK_INTERFACE_BTN_ID);
const body = document.body;

const $ = (selector) => document.querySelector(selector);
const $$ = (selector) => document.querySelectorAll(selector);

function pingServer() {
    const startTime = performance.now();
    fetch(PING_URL, { method: 'HEAD', cache: 'no-store' })
        .then(response => {
            if (response.ok) {
                const time = Math.round(performance.now() - startTime);
                pingValue.textContent = `${time} ms`;
            } else {
                throw new Error(`Ping failed with status ${response.status}`);
            }
        })
        .catch(error => {
            pingValue.textContent = 'Configure IP E003.5';
            handleError('003', 'Ping request failed', error);
        });
}

function toggleMenu() {
    try {
        $('.hamburger-menu')?.classList.toggle('active');
        $('.sidenavbar')?.classList.toggle('active');
    } catch (error) {
        handleError('005', 'Error toggling menu', error);
    }
}

function toggleInterfaceLock() {
    body.classList.toggle('interface-locked');
    const isLocked = body.classList.contains('interface-locked');
    lockInterfaceBtn.innerHTML = `<strong>${isLocked ? 'Unlock' : 'Lock'} Interface</strong>`;
    localStorage.setItem('lockState', isLocked ? 'locked' : 'unlocked');
}

function deleteCookie(name, path = '/', domain) {
    let cookieString = `${name}=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=${path};`;
    if (domain) {
        cookieString += ` domain=${domain};`;
    }
    document.cookie = cookieString;
}

document.getElementById('log-out').addEventListener('click', function(event) {
	event.preventDefault();

	deleteCookie('AuthToken');
	deleteCookie('SessionToken');

	console.log('Current cookies after deletion attempt:', document.cookie);

	window.location.href = '/login';
});

document.getElementById('log-out').addEventListener('click', function (event) {
    event.preventDefault(); 
    deleteCookie('AuthToken'); 
    deleteCookie('SessionToken'); 
    window.location.href = '/login'; 
});

function applyTheme(theme) {
    const themeClasses = ['theme-light', 'theme-dark', 'theme-colorblind'];

    themeClasses.forEach(cls => body.classList.remove(cls));

    body.classList.add(theme);

    localStorage.setItem('theme', theme);
    $$('.theme-button').forEach(btn => {
        btn.classList.toggle('theme-active', btn.id === `${theme.replace('theme-', '')}-theme`);
    });
}
function initializeButtonIndicators(startButtonId, stopButtonId, storageKey) {
    const startButton = document.getElementById(startButtonId);
    const stopButton = document.getElementById(stopButtonId);

    if (!startButton || !stopButton) {
        handleError('008', `Button elements not found for: ${startButtonId}, ${stopButtonId}`);
        return;
    }

    function updateButtonState(isStart) {
        startButton.classList.toggle('clock-active', isStart);
        stopButton.classList.toggle('clock-active', !isStart);
        localStorage.setItem(storageKey, isStart ? 'start' : 'stop');
    }

    startButton.addEventListener('click', () => updateButtonState(true));
    stopButton.addEventListener('click', () => updateButtonState(false));

    const savedState = localStorage.getItem(storageKey);
    if (savedState === 'start' || savedState === 'stop') {
        updateButtonState(savedState === 'start');
    }
}

function sanitizeInput(input) {
    return input.replace(/\s+/g, '');
}

function escapeHTML(str) {
    return str.replace(/[&<>"']/g, function(match) {
        switch (match) {
            case '&':
                return '&amp;';
            case '<':
                return '&lt;';
            case '>':
                return '&gt;';
            case '"':
                return '&quot;';
            case "'":
                return '&#39;';
            default:
                return match;
        }
    });
}

function handleInput(event) {
    let input = event.target.value;
    input = sanitizeInput(input);
    event.target.value = input;
}

function processAndDisplay(input) {
    const safeInput = escapeHTML(input);
    document.getElementById('output').innerHTML = safeInput; 
}

document.addEventListener('DOMContentLoaded', () => {

    applyTheme(localStorage.getItem('theme') || DEFAULT_THEME);

    const lockState = localStorage.getItem('lockState');
    if (lockState === 'locked') {
        body.classList.add('interface-locked');
        lockInterfaceBtn.innerHTML = '<strong>Unlock Interface</strong>';
    }

    const iframe = $('#previewIframe');
    if (iframe) iframe.src = PREVIEW_URL;

    const hostIPInput = $('#host-ip');
    if (hostIPInput) {
        hostIPInput.value = localStorage.getItem('hostIP') || '';
        hostIPInput.addEventListener('input', () => {
            const sanitizedValue = hostIPInput.value.replace(/[&<>"'\\/?*]/g, '');
            hostIPInput.value = sanitizedValue;
            localStorage.setItem('hostIP', sanitizedValue);
        });
    }

    ['light', 'dark', 'colorblind'].forEach(themeName => {
        $(`#${themeName}-theme`)?.addEventListener('click', () => applyTheme(`theme-${themeName}`));
    });

    lockInterfaceBtn?.addEventListener('click', toggleInterfaceLock);

    $('.hamburger-menu')?.addEventListener('click', toggleMenu);

    initializeButtonIndicators('countdownStartButton', 'countdownStopButton', 'countdownState');
    initializeButtonIndicators('clockStartButton', 'clockStopButton', 'clockState');

    setInterval(pingServer, PING_TIME);

    initializePopup();

    let sidebar = document.querySelector(".sidebar");

    let storedScrollPosition = sessionStorage.getItem("sidebarScroll");
    if (storedScrollPosition !== null) {
        sidebar.scrollTop = Number(storedScrollPosition);
    }
    window.addEventListener("beforeunload", () => {
        sessionStorage.setItem("sidebarScroll", sidebar.scrollTop);
    });
});

document.getElementById('overlay-color').addEventListener('input', function () {
    const color = this.value;
    document.getElementById('color-value').textContent = color;
    localStorage.setItem('overlayColor', color);

    updateOverlay(color, document.getElementById('overlay-alpha').value);
});

document.getElementById('overlay-alpha').addEventListener('input', function () {
    const alpha = this.value; 
    document.getElementById('alpha-value').textContent = alpha;
    localStorage.setItem('overlayAlpha', alpha);

    updateOverlay(document.getElementById('overlay-color').value, alpha);
});

document.getElementById('reset-color').addEventListener('click', function () {
    const defaultColor = '#00b140';
    const defaultAlpha = '1';

    document.getElementById('overlay-color').value = defaultColor;
    document.getElementById('overlay-alpha').value = defaultAlpha;

    document.getElementById('color-value').textContent = defaultColor;
    document.getElementById('alpha-value').textContent = defaultAlpha;

    localStorage.setItem('overlayColor', defaultColor);
    localStorage.setItem('overlayAlpha', defaultAlpha);

    updateOverlay(defaultColor, defaultAlpha);
});

function updateOverlay(color, alpha) {
    const overlay = document.querySelector('.overlay');
    if (overlay) {
        const r = parseInt(color.slice(1, 3), 16);
        const g = parseInt(color.slice(3, 5), 16);
        const b = parseInt(color.slice(5, 7), 16);
        overlay.style.backgroundColor = `rgba(${r}, ${g}, ${b}, ${alpha})`;
    }
}

window.onload = function() {
    const savedColor = localStorage.getItem('overlayColor') || '#00b140';
    const savedAlpha = localStorage.getItem('overlayAlpha') || '1';

    document.getElementById('overlay-color').value = savedColor;
    document.getElementById('overlay-alpha').value = savedAlpha;

    document.getElementById('color-value').textContent = savedColor;
    document.getElementById('alpha-value').textContent = savedAlpha;

    updateOverlay(savedColor, savedAlpha);
};

document.getElementById('reset-color').addEventListener('click', function () {
    const defaultColor = '#00b140';
    const defaultAlpha = '100';
    const defaultOpacity = '50';

    document.getElementById('overlay-color').value = defaultColor;
    document.getElementById('color-value').textContent = defaultColor;
    localStorage.setItem('overlayColor', defaultColor);

    document.getElementById('overlay-alpha').value = defaultAlpha;
    document.getElementById('alpha-value').textContent = defaultAlpha;
    localStorage.setItem('overlayAlpha', defaultAlpha);

});

function apiCopy(button, text) {
    if (navigator.clipboard && window.isSecureContext) {
        navigator.clipboard.writeText(text).then(() => {
            console.log('Text copied to clipboard');
        }).catch(err => {
            console.error('Failed to copy text: ', err);
        });
    } else {
        const tempInput = document.createElement('input');
        tempInput.value = text;
        document.body.appendChild(tempInput);
        tempInput.select();
        try {
            document.execCommand('copy');
            console.log('Text copied to clipboard');
        } catch (err) {
            console.error('Failed to copy text: ', err);
        }
        document.body.removeChild(tempInput);
    }

    button.textContent = 'Copied!';

    setTimeout(() => {
        button.textContent = 'Copy';
    }, 2000);
}

function applyCooldown(button) {
    button.disabled = true;
    button.classList.add('popup-cooldown');
    setTimeout(function () {
        button.disabled = false;
        button.classList.remove('popup-cooldown');
    }, 7500);
}
pingServer();
loadPresets()