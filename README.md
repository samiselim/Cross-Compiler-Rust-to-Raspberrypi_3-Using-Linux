By Sami Abdelaziz Selim
# Cross_Compiler_Rust_Raspberrypi_3-

## > First of all need to install dependencies for u-boot 
 
```
sudo apt update
sudo apt install -y gcc-arm-linux-gnueabihf
sudo apt install -y binutils-arm-linux-gnueabihf
sudo apt install -y curl
```
## > Installing Rust as Normal Installation
  ```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```
  
  And
   > Proceed with installation (default)

## > Adding our taget to rustup 
   ``` rustup target add armv7-unknown-linux-gnueabihf``` 
## > Creating Hello_World Project name: myProject
```
    cargo new myProject
    cd myProject
```
## > Creating Config file
```
    mkdir .cargo
    touch config
    sudo nano config
```
## > Adding the following lines to this file (Target, Linker and Static Compiling )
        [target.armv7-unknown-linux-gnueabihf]
        linker = "arm-linux-gnueabihf-gcc"
        rustflags = ["-C", "target-feature=+crt-static"]
      
    
## > Build our Project
  ```cargo build --target=armv7-unknown-linux-gnueabihf```
## > Move the Excecutable file to raspberry pi by any way ex: (netcad)
## > On Raspberry pi give Excecute Permission to the Excecutable file 
    ``` chmod +x Excecutable_file


## > And hava fun with rust on Raspberry pi :) 
    
