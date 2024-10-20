## Warp Cloudflare GUI written in Rust
A GUI application based on warp-cli for linux written in Rust

## Screenshots

![1729435264_grim](https://github.com/user-attachments/assets/6e2d3bdb-b13f-4491-a071-f95141dadec1)
![1729435273_grim](https://github.com/user-attachments/assets/8de1de84-fe55-4153-9672-d880f90eef39)
![1729435282_grim](https://github.com/user-attachments/assets/d5bfa816-ca19-461a-a303-8559b25869f7)
![1729435290_grim](https://github.com/user-attachments/assets/311d553f-cb99-4637-8989-1b0286497247)



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
