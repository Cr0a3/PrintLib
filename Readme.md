# PrintLib
PrintLib is a libary for rust which makes printing easy.

## Install
To add PrintLib to your project, go into your project folder and execute:
```
cargo add PrintLib
```

Then you can import PrintLib just like other crates:
```
use PrintLib::*;
```

## Documentation
This is the official documentation for the PrintLib.

### Error
The module <code>PrintLib::Error</code> is the module with which you can build error messages.

| Function-nane | Parmaters | Description |
|---------------|-----------|-------------|
|<code>new</code>|ecode: <code>String</code>, msg: <code>String</code>| This is the initializer of the class. It creates a new ErrorFactory with the error-code <code>ecode</code> and the error message <code>msg</code>|
|<code>add_code_line</code>|line: <code>String</code>, display_line_no: <code>bool</code>, line_no: <code>usize</code>, display_add: <code>bool</code>|This function adds a new  line to the error message. The parameter <code>line</code> is the code line of which the error message is specified. <code> display_lin_no</code> means if the line number (parameter <code>line_no</code> is the line number) is shown. <code>display_add</code> means if a +++ needs to be shown (it is good for showing potential fixes).
|<code>add_where</code|where_start: <code>usize</code>, where_length: <code>usize</code>, where_msg_b: <code>bool</code>, where_msg: <code>String</code>|This function adds an string to the error message to show where the error is. <code>where_start</code> means where the showing should start (e.g. where_start = 3 means it starts after the 3rd character). <code>where_length</code> means how long the showing should be. <code>where_msg_b</code>: when true it showes a string to the right of the showing. When false it shows just the showing. <code>where_msg</code> is the msg right to the showing (if <code>where_msg_b</code> is true)|
|<code>add_arrow</code>|file: <code>String</code>, line: <code>usize</code>, where_start: <code>usize</code>|Add adds an arrow in the format -->{<code>file</code>}:{<code>line</code>}:{<code>where_start</code>} to the error message|
|<code>add_arrowW</code>|file: <code>String</code>, line: <code>usize</code>, |Add adds an arrow in the format -->{<code>file</code>}:{<code>line</code>} to the error message|
|<code>print</code>||Prints the error message