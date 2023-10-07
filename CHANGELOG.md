# Changelog 
## Notes
- The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
- This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html)
- This project uses [ISO Standard](https://www.iso.org/iso-8601-date-and-time-format.html) date formatting

## [Unreleased]
### Fixed
- macOS Sonoma crashing the application by updating dependencies.

## [0.3.0] - 2023-09-22
### Added
- Tutorial Level Map, including free-to-use placeholder assets
- Unit Testing for Level Boundaries
- Spellchecking logging for mistyped asset loading
- Main Menu w/ Quit and Play Buttons that exit the application and transition to the game respectively
- Caching for Builds on GitHub
- Windows and Runtime optimizations
- Generalizations focused on reusability of functions
- Interactive Entities

### Changed
- Updated Bevy to version 0.11
- Updated Bevy ECS LDtk to version 0.8
- Updated Bevy Kira Audio to version 0.16
- Restricted Player Movement to Level Boundaries
- Restricted Camera Movement to Level Boundaries
- Game App Setup moved into Plugins
- Plugin system now load via Game States
- Updated dependencies for Continuous Integration GitHub Actions
- Isolated Animation code from Collision Detection code
- Made character movement reliant on component change detection rather than event broadcasts


### Fixed
- Camera now maintains own z position instead of adopting the players
- Fixed off-by-one error in collision tile placement
- Camera now updates position correctly upon changing window size

## [0.2.0] - 2022-07-28
### Added
- Making Universal Builds for MacOS
- Play Background Music
- Play Movement Based SFX (Footsteps & Collision)

### Fixed
- Making Windows/MacOS/Linux executables automatically per new version

## [0.1.0] - 2022-06-12
### Added
- Collision (Basic)
- LDtk Map Imports
- Player Tracking Camera (Basic)
- Character Movement via WASD

[0.3.0]: https://github.com/Exonoesis/Helping-Hand/releases/tag/v0.3.0
[0.2.0]: https://github.com/Exonoesis/Helping-Hand/releases/tag/v0.2.0
[0.1.0]: https://github.com/Exonoesis/Helping-Hand/releases/tag/v0.1.0