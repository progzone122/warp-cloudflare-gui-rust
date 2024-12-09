## Warp Cloudflare GUI written in Rust
A GUI application based on warp-cli for linux written in Rust

## Screenshots



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
