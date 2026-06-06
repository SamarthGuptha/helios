# Helios Distributed Compiler

This guide walks you through deploying the Helios P2P compiler network across two physical machines on the same local network. 

**Terminology:**
* **Computer A :** Your main development machine where you write code.
* **Computer B :** The remote machine that will compile the code.

## Set Up the Computers A and B
Computer B does not need the Helios source code, but it **does** need the actual compiler installed.  
Once you verify that the compiler exists on the worker, copy the helios-daemon onto it and open the .exe  

Open the .exe file on computer A aswell.

## Running a distributed build.
1. using a rust file as an example, setup a test file named `test.rs` on computer A
2. Open a **second, seperate terminal** on computer A, leave the daemon running.
3. Trigger the network using the Helios CLI, passing the absolute path to your test file:
    ```bash
   cargo run --bin helios-cli -- rustc C:\Users\YourUsername\Desktop\test.rs


## DEMO
Here, Computer A is the actual computer, while Computer B is a windows sandbox. 


https://github.com/user-attachments/assets/73cc1cbb-10ed-4f53-a510-3467fcee24b0


( AI Assisted. )




