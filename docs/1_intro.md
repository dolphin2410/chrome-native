# Chrome Native
Chrome Native is a software that works as a bridge - from the native level to the browser. This might not be a secure way of communicating, but it might be useful in some circumstances.

## How it works
Chrome provides an API called `nativeMessaging`. This API uses `stdio`(standard I/O - commandline) to communicate with a native executable application. Chrome Native parses the given message from the browser and automatically sends the data to the right plugin.

## The plugin
The plugin is a `.dll` type, which means it is a library. This is because the main executable that the browser will be talking to, is the `chrome-native-rt` program, not the plugin. The plugin should be able to be loaded by the runtime program.