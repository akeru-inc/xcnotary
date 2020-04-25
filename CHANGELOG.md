# Change Log

## [v0.4.0](https://github.com/akeru-inc/xcnotary/releases/tag/v0.4.0)

* Added support for .dmg file notarization. [#3](https://github.com/akeru-inc/xcnotary/issues/3)
* Handle "Unnotarized Developer ID" spctl response for new Developer accounts. [#4](https://github.com/akeru-inc/xcnotary/issues/4)

## [v0.3.0](https://github.com/akeru-inc/xcnotary/releases/tag/v0.3.0)

* New `--provider` flag to support for account associated with more than one developer team. [#2](https://github.com/akeru-inc/xcnotary/issues/2)

## [v0.2.0](https://github.com/akeru-inc/xcnotary/releases/tag/v0.2.0)

* Added support for .pkg file notarization. [#1](https://github.com/akeru-inc/xcnotary/issues/1)
* Revised command line interface for additional flexibility, breaking backward-compatibility from 0.1.0. Command argument now needs to be specified, i.e. `xcnotary precheck` or `xcnotary notarize`.
* Code signing checks are now tested using generated bundles and packages of various levels of code signing correctness.

## [v0.1.0](https://github.com/akeru-inc/xcnotary/releases/tag/v0.1.0)

Initial release.
