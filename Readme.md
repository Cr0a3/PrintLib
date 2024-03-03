# PrintLib

PrintLib is a libary for rust which makes printing easy.


> This document is and (works) only for the newest ``PrintLib`` version
> 
[Github reposentory](https://github.com/Toni-Graphics/PrintLib)

## Install

To add PrintLib to your project, go into your project folder and execute:

```bash
cargo add PrintLib
```

Then you can import PrintLib just like other crates:

```rust
use PrintLib::*;
```

## Examples

In the example folder you can see two examples:

- ``error_test``: Test usage of the ErrorFactory
- ``logger_test``: Test usage of the Logger

Link to the examples:
    <https://github.com/Toni-Graphics/PrintLib/tree/master/examples/>

## Documentation

This is the official documentation for the PrintLib.

### Error

The module ``PrintLib::error`` is the module with which you can build error messages.

The ``ErrorFactory`` class

| Function name | Parmaters | Description |
|---------------|-----------|-------------|
|``new``|ecode: ``String``, msg: ``String``| This is the initializer of the class. It creates a new ErrorFactory with the error-code ``ecode`` and the error message ``msg``|
|``add_code_line``|line: ``String``, display_line_no: ``bool``, line_no: ``usize``, display_add: ``bool``|This function adds a new  line to the error message. The parameter ``line`` is the code line of which the error message is specified. ``display_lin_no`` means if the line number (parameter ``line_no`` is the line number) is shown. ``display_add`` means if a +++ needs to be shown (it is good for showing potential fixes).|
|``add_where</code|where_start: ``usize``, where_length: ``usize``, where_msg_b: ``bool``, where_msg: ``String``|This function adds an string to the error message to show where the error is. ``where_start`` means where the showing should start (e.g. where_start = 3 means it starts after the 3rd character). ``where_length`` means how long the showing should be. ``where_msg_b``: when true it showes a string to the right of the showing. When false it shows just the showing. ``where_msg`` is the msg right to the showing (if ``where_msg_b`` is true)|
|``add_arrow``|file: ``String``, line: ``usize``, where_start: ``usize``|Add adds an arrow in the format -->{``file``}:{``line``}:{``where_start``} to the error message|
|``add_arrowW``|file: ``String``, line: ``usize``, |Add adds an arrow in the format -->{``file``}:{``line``} to the error message|
|``print``||Prints the error message|

### Logger

The module ``PrintLib::logr`` is the module with which you can use the logger.

#### The ``Logger`` Class

| Function name | Parmaters | Description |
|---------------|-----------|-------------|
|``new``| conf | Is the initializer of the class. The parameter ``conf`` is the ``LoggerConfig`` for the class. Returns a new Instance.|
|``debug``|msg: ``String``| Prints the message ``msg`` in level debug|
|``info``|msg: ``String``| Prints the message ``msg`` in level info|
|``warn``|msg: ``String``| Prints the message ``msg`` in level warning|
|``error``|msg: ``String``| Prints the message ``msg`` in level error|

#### The ``def`` function

Returns ``Logger`` with the default configuration

#### The ``LoggerConfig`` struct

| Name | Type | Description |
|---------------|---------|-----------|
|``time``|``bool``|If the time should be presented in logger messages|
|``info_string``|``String``|String which indicates that the message is level info (Normal: [INFO  ])|
|``debug_string``|``String``|String which indicates that the message is level debug (Normal: [DEBUG ])|
|``warn_string``|``String``|String which indicates that the message is level warn (Normal: [WARN!] )|
|``err_string``|``String``|String which indicates that the message is level error (Normal: [[ERR!]  ])|
|``info_color``|``LoggerColor``|The color in which the level indicator ([xyz]) at level info is|
|``debug_color``|``LoggerColor``|The color in which the level indicator ([xyz]) at level debug is|
|``warn_color``|``LoggerColor``|The color in which the level indicator ([xyz]) at level warn is|
|``err_color``|``LoggerColor``|The color in which the level indicator ([xyz]) at level err is|

#### The ``LoggerColor`` struct

| Name | Type | Description |
|---------------|---------|-----------|
|``r``|``u8``|The red channel|
|``g``|``u8``|The green channel|
|``b``|``u8``|The blue channel|

### Color

The module ``PrintLib::colorize`` is the module with which you can colorize your ``&str``s.

#### The ``Color`` struct

The color struct is the struct in which rgb color can be stored.
| Parameter name | Type | Description |
|----------------|------|-------------|
|``r``|``u8``|The red color channel.|
|``g``|``u8``|The green color channel.|
|``b``|``u8``|The blue color channel.|

#### The ``Colorize`` trait

| Function name | Parameters | Return type | Description |
|---------------|------------|-----------|------------|
|``color``|``&self``, clr: ``Color``|``String``|Turns the colorof the string to the specified rgb-values in ``clr``|
|``black``|``&self``|``String``| Makes the string black|
|``red``|``&self``|``String``| Makes the string red|
|``green``|``&self``|``String``| Makes the string green|
|``yellow``|``&self``|``String``| Makes the string yellow|
|``blue``|``&self``|``String``| Makes the string blue|
|``magenta``|``&self``|``String``| Makes the string magenta|
|``cyan``|``&self``|``String``| Makes the string cyan|
|``white``|``&self``|``String``| Makes the string white|
|``gray``|``&self``|``String``| Makes the string gray|
