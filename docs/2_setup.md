# Setup

## Installing Runtime
From the [Releases](https://github.com/dolphin2410/chrome-native/releases) page, you can download the msi file. Execute it, and the default install path would be `C:/Program Files/chrome-native`. There you'll see a `manifest.json` file. 

## Editing manifest.json

1. Change the `name` to whatever hostname you'd like to have. I'll call this `hostname` from now on.
2. ***IMPORTANT!!!***  You need to add your extension url (`chrome-extension://~~~~~/` format) to the `allowed_origins` section. Skipping this step will lead to errors in your extension.

## Link with web browser
In order to use chrome's `Native Messaging API`, you need to have your registry set up. You can do so by running one of these commands.

1. Google Chrome
```bash
REG ADD "HKCU\Software\Google\Chrome\NativeMessagingHosts\<hostname>" /ve /t REG_SZ /d "C:/Program Files/chrome-native/manifest.json" /f
```

2. Microsoft Edge
```bash
REG ADD "HKCU\Software\Microsoft\Edge\NativeMessagingHosts\<hostname>" /ve /t REG_SZ /d "C:/Program Files/chrome-native/manifest.json" /f
```

