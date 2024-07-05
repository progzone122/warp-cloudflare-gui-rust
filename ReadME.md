## Warp Cloudflare GUI written in Rust
A GUI application based on warp-cli for linux written in Rust

## Screenshots
![1720103650_grim](https://github.com/progzone122/warp-cloudflare-gui-rust/assets/80642969/def8c5ad-7628-4ada-a15b-3cbe1f068499) ![1720103658_grim](https://github.com/progzone122/warp-cloudflare-gui-rust/assets/80642969/6c47a077-0a78-4769-842d-dafd4fbdf5dc) ![1720103690_grim](https://github.com/progzone122/warp-cloudflare-gui-rust/assets/80642969/fed130a5-b058-4b36-a4e0-d79a9d21e2e9)


## Installation Guide
### I use Arch Linux btw?
Install using the script from AUR!

``
    yay -S warp-gui
``

**WARNING! To connect using WARP you need to register an account, this can be done through the GUI itself (Settings button in the bottom right corner -> Account -> register).**

### Or install manually
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
3. Download the GUI binary in the [release section](https://github.com/progzone122/warp-cloudflare-gui-rust/releases)
4. Grant permission to run

    ``
        sudo chmod +x warp-gui
    ``

5. To run the program anywhere using the terminal, move the binary file to the /usr/bin directory

    ``
        sudo mv ./warp-gui /usr/bin/warp-gui
    ``

6. Run binary

    ``
         warp-gui
    ``

7. **WARNING! To connect using WARP you need to register an account, this can be done through the GUI itself (Settings button in the bottom right corner -> Account -> register).**
