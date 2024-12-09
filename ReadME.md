## Warp Cloudflare GUI written in Rust
A GUI application based on warp-cli for linux written in Rust

## Screenshots
<div style="display: flex; flex-wrap: wrap;">
    <img src="https://github.com/user-attachments/assets/c4d4f95a-c572-41af-bc57-30fe46f2e9b7" width="400">
    <img src="https://github.com/user-attachments/assets/dfbe90ab-506a-4672-8790-e63288ae95bb" width="400">
    <img src="https://github.com/user-attachments/assets/9d7ecf32-45c2-44ad-889d-c3220964b19e" width="400">
    <img src="https://github.com/user-attachments/assets/0f792b3c-38d8-4f1d-9eb2-75eb45cd4ffb" width="400">
</div>


## Installation Guide

**WARNING! To connect using WARP you need to register an account, this can be done through the GUI itself (Settings button in the bottom right corner -> Account -> register).**

### I use Arch Linux btw?
1. Install using the script from AUR!

``
    yay -S warp-gui
``

2. Start systemd service **warp-svc**

``
     sudo systemctl enable warp-svc.service --now
``

### Debian package
1. Install [warp-cli](https://developers.cloudflare.com/warp-client/get-started/linux/)

   - Arch Linux:

     ``
         yay -S cloudflare-warp-bin
     ``
   - Ubuntu / Debian:

     ``
         sudo apt install cloudflare-warp
     ``

   - CentOS / RHEL:

     ``
         sudo yum install cloudflare-warp
     ``
2. Start systemd service **warp-svc**

   ``
        sudo systemctl enable warp-svc.service --now
   ``
3. Download and install Debian package:
   - If you are using dpkg:
   ``
      sudo dpkg -i warp-gui.deb
   ``
   - If you are using debtap:
   
      ```
         sudo debtap -u
         debtap -Q warp-gui.deb
         sudo pacman -U generated_name.pkg.tar.xz
      ```

### AppImage package
1. Install [warp-cli](https://developers.cloudflare.com/warp-client/get-started/linux/)

   - Arch Linux:

     ``
         yay -S cloudflare-warp-bin
     ``
   - Ubuntu / Debian:

     ``
         sudo apt install cloudflare-warp
     ``

   - CentOS / RHEL:

     ``
         sudo yum install cloudflare-warp
     ``
2. Start systemd service **warp-svc**

   ``
        sudo systemctl enable warp-svc.service --now
   ``
3. Download and run AppImage package
