# FROGGI
[![Rust](https://github.com/obfedder/Froggi/actions/workflows/rust.yml/badge.svg)](https://github.com/obfedder/Froggi/actions/workflows/rust.yml)
![Version](https://img.shields.io/badge/version-2.0.0-blue) ![GitHub License](https://img.shields.io/github/license/obfedder/froggi)

**F**lexible **R**eal-time **O**verlay for **G**ame **G**raphics and **I**nformation  
Is a self-hosted & portable (just one binary file!) scoreboard solution that aims to provide an intuitive and simple sports broadcasting overlay.

# Features
 - ✨ Optical character recognition (OCR) using [froggi-ocr](https://github.com/obfedder/froggi-ocr) and [scoresight-ocr](https://github.com/locaal-ai/scoresight)
 - ✨ Authentication through an API key in an HTTP header (allowing for authentication for use with Bitfocus Companion, and other use cases with http)
 - ✨ An (optional) sponsor logo slideshow
 - ✨ Easily transferrable team presets
 - ✨ Cross platform (Supports Windows, MacOS, Linux, and Docker)

# Installation
## Docker
Froggi is on [Docker Hub](https://hub.docker.com/repository/docker/obfedder/froggi/general).  
(Docker images are built every stable release, the latest docker image will always be the latest commit on the main branch)  
- Make sure [Docker](https://www.docker.com/get-started/) is installed.
- Then, create the container.
  - Port is defined with "-p (external):(internal), only change external.
  - You can run "docker ps -a" to see information about all containers (running or not)
```
docker create --name froggi -p 80:3000 obfedder/froggi
```
- Finally, run the container
```
docker start froggi
```
- Froggi will be accessible under localhost:80 (or whatever port you bound the external port to)
- For more information, consult [Docker documentation](https://docs.docker.com/).

## Binary
Prebuilt binaries for Windows and Linux are found under [releases](https://github.com/obfedder/Froggi/releases). If running on MacOS then follow the guide on how to [compile from source](https://github.com/obfedder/Froggi/#compilation).  

# Usage
If running as a standalone executable, make sure to start the "froggi" binary, not the "froggi-worker" binary.  
If running under Docker, simply start the container.  

# Setup
- After running for the first time, navigate to the web interface (default port is 3000).  
- You will then be prompted to create the login for Froggi's interface, and after creating it will be sent to the dashboard.  
- You can then set game presets and upload sponsor images in the "Sponsors & Teaminfo" page under the burger menu.  
- There is additional configuration under the "Settings" page under the burger menu also.
- For specialized configuration options modify the config.json automatically generated by Froggi upon first run.  
- Modifications to config.json are automatically applied upon restarting Froggi, the easiest way to restart/stop Froggi are the program controls at the bottom of the "Settings" page in the burger menu.  

# Roadmap
Froggi is an indev project so changes are very likley.
Here are some features/updates planned in no particular order
 - [] 🗺️ Options for sport-specific features and customizations
 - [] 🗺️ Pop-up animations, and support for .gif animations (in an 16:9 or 1920x1080 aspect ratio)
 - [] 🗺️ A first party bitfocus companion plugin
 - [] 🗺️ Frontend settings saving via the backend

# Platform support
### Windows
Froggi has full Windows support (with one exception), and binaries under [releases](https://github.com/obfedder/Froggi/releases).  
However due to the way Windows signals work, you should _never_ stop Froggi by simply doing Ctrl+c in the terminal (doing so will terminate Froggi instead of letting it shutdown gracefully), instead stop Froggi through the program controls at the bottom of settings in the web interface.  
It is heavily suggested to run Froggi under WSL or Docker due to froggi being developed & maintained for linux.

### MacOS
Froggi has full MacOS support, however due to the difficulty in cross-compiling for MacOS precompiled binaries are not offered. Detailed instructions on how to [compile from source](https://github.com/obfedder/Froggi/#compilation) are found below.  
If you are unable/unwanting to compile binaries from the source, it is reccomended to use the official docker image.  

### Linux
Froggi has full Linux support, and binaries under [releases](https://github.com/obfedder/Froggi/releases).

### Docker
Froggi has full Docker support, and an image on [Docker Hub](https://hub.docker.com/repository/docker/obfedder/froggi/general).  
Docker is the reccomended way to run Froggi.  

# Updating
Froggi has self-updating capabilities, allowing for easy updates directly through the Settings page whenever a new version is available.  
Updates are compiled from source, and in order to update Froggi needs all [build dependencies](https://github.com/obfedder/Froggi/#dependencies) installed.  
The Docker image comes with everything needed to compile updates from source.  

# Compilation 
## Dependencies
 - [Rust Toolchain](https://rustup.rs)
 - Essential C build tools (MacOS & Linux platforms only, Rustup on Windows will prompt you to install them)
     - Developer Tools on MacOS
     - GCC on Linux platforms (usually installed under base-devel, build-essential, or any package simmilar to that)
 - OpenSSL libraries and headers (Linux only, see https://docs.rs/openssl/latest/openssl/ for more info)
 - Git (also included with Developer Tools on MacOS)

Once all the build dependancies are installed, clone Froggi's git repository with:
```
git clone https://github.com/obfedder/Froggi.git
```
Then navigate to the directory with:
```
cd Froggi
```
Finally compile Froggi with:
```
cargo build --release
```
The compiled binaries will be under target/release/froggi(.exe if on Windows) and target/release/froggi-worker(.exe if on Windows).  
When running the binaries, _only run froggi_, never run froggi-worker. The two binaries must be in the same directory as each other.  
Or run them in place with cargo with:
```
cargo run --release --bin froggi
```

# Tech Stack
 - Rust, Axum, and Tokio
 - HTML, CSS, JavaScript, and the [HTMX](https://htmx.org/) library for the frontend

# Licence
Froggi is distributed under the [MIT Licence](https://mit-license.org/), a permissive open-source license. 
