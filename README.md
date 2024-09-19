# Clockwatcher
## Description
Ever wondered how long until your work day is over? Well, now you know. Maybe.

## Running
Here are some fun commands to make this run:
```
npm install
npm run tauri dev
```

## Installing
You can download the installer from the releases section.

If you don't want to do that, here are some fun commands to install it:
```
npm install
npm run tauri build
```

You can then create a shortcut to `./src-tauri/target/release/clockwatcher.exe` and add it to `%HOMEDRIVE%\ProgramData\Microsoft\Windows\Start Menu\Programs\StartUp\`.
The actual `.exe` file produced by the build should be portable provided that WebView2 is installed on the target machine.
