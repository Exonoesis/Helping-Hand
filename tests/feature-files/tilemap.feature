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

    Scenario: Tile textures correctly load from a single sprite sheet.
        Given a Tiled map called single_sprite_sheet.tmx,
        When the Tiled map is loaded,
        Then there are 4 tiles loaded.
        And each tile points to the same sprite sheet.
        And each tile points to the correct image on the single sprite sheet.

    Scenario: Tile textures correctly load from multiple sprite sheets.
        Given a Tiled map called multiple_sprite_sheet.tmx,
        When the Tiled map is loaded,
        Then there are 4 tiles loaded.
        And the top two tiles point to one sprite sheet,
        And the bottom two tiles point to the other sprite sheet,
        And each tile points to the correct image on the multiple sprite sheets.

    Scenario: Map loads correctly when some tiles have no image.
        Given a Tiled map called one_blank.tmx,
        When the Tiled map is loaded,
        Then the first three tiles contain an image element,
        And the last tile has no image element.

    Scenario: Load a Tiled map with multiple layers.
        Given a Tiled map called two_layers.tmx,
        When the Tiled map is loaded,
        Then there exist two overlapping layers of tiles.