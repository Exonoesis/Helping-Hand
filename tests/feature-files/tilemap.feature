Feature: Load Tilemap from Tiled.
    Scenario: All tiles are loaded.
        Given a Tiled map called test_map.tmx,
        When the Tiled map is loaded,
        Then there are 16 tiles loaded.

    Scenario: Tiles are loaded in a grid.
        Given a Tiled map called test_map.tmx,
        When the Tiled map is loaded,
        Then there are 16 tiles loaded.
        And the tiles are in a 4x4 grid.