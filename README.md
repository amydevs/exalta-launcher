# Exalta Launcher
![image](https://user-images.githubusercontent.com/50583248/174352490-2eebb7ac-594a-4337-85fe-2237dbf90ace.png)

## Setup
### Username/Password
1. Open the application.

### Steam
1. Create a new folder called `exalta` in your Realm of the Mad God launcher folder (ie: "C:\Program Files (x86)\Steam\steamapps\common\Realm of the Mad God"), and extract the contents of `exalta-steam.zip` into it.
2. Paste this in your game properties launch options:
  ```bash
  "C:\Windows\System32\cmd.exe" /c ".\exalta\exalta-steam.exe && echo %command%"
  ```


## Why?
Because the official launcher is a Unity game itself, and therefore has quite high resource usage and takes quite a bit of time to initialize.

## Features/Todo
- [X] Login with Username and Password
- [X] Run Game
- [X] Login With Steam
- [X] Downloading / Updating / Verifying Game Files
- [ ] Login in Case of Security Questions
- [ ] Signing Up 
- [ ] Mac / Linux Support
- [ ] Emulates Traffic Exactly
