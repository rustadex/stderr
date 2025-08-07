# CHANGELOG


## 0.5 Static Logger and Macros

* Added a static logger `stderr::logger`
* Started rolling out new macros
* Data formatters and Debug Printing `DebugPrinter` 
* Enum `LogLevel` for use with `log()`
* Enum `OptionFlag` for use with print modes (mostly `log`)
* New `macro_driver` in examples
* Moved stderr helper functions to `utils/helpers/rs`
* Fixed some incorrect ANSI colors (yellow is actually yellow now)
* Working to keep the top-level API stable despite internal changes
* Manual integration testing
* Updated README
* Version bump (Minor)


### TODO:

* Finish macros.
* Finish trace.
