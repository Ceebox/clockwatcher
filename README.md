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

## Configuration
To set the length of the timer, right click the Clockwatcher tray icon and press "Settings", which will present you with a nice GUI to configure how long you want the timer to be.
If you wish to adjust the theme of the application, go to the `settings.clockwatcher` file and add a new line containing "Theme: " (in a similar format as the "Milliseconds" field above it).
After this property you can choose from a select of themes:
* Default
* Light
* Dark
* Default-Static
* Fun

You can add your own CSS files in a similar format to these themes if you wish to customise the appearance of the application.
