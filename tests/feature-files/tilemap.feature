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
        And tile 1 points to spritesheet atlas_64x.png.
        And tile 1's spritesheet has dimensions 1024 x 3072.
        And tile 2 points to spritesheet atlas_64x.png.
        And tile 2's spritesheet has dimensions 1024 x 3072.
        And tile 3 points to spritesheet atlas_64x.png.
        And tile 3's spritesheet has dimensions 1024 x 3072.
        And tile 4 points to spritesheet atlas_64x.png.
        And tile 4's spritesheet has dimensions 1024 x 3072.
        And tile 1 points to image number 1.
        And tile 2 points to image number 5.
        And tile 3 points to image number 49.
        And tile 4 points to image number 53.


    Scenario: Tile textures correctly load from multiple sprite sheets.
        Given a Tiled map called multiple_sprite_sheet.tmx,
        When the Tiled map is loaded,
        Then there are 4 tiles loaded.
        And tile 1 points to spritesheet !CL_DEMO_64.png.
        And tile 2 points to spritesheet !CL_DEMO_64.png.
        And tile 3 points to spritesheet atlas_64x.png.
        And tile 4 points to spritesheet atlas_64x.png.
        And tile 1 points to image number 131.
        And tile 2 points to image number 128.
        And tile 3 points to image number 115.
        And tile 4 points to image number 164.

    Scenario: Map loads correctly when some tiles have no image.
        Given a Tiled map called one_blank.tmx,
        When the Tiled map is loaded,
        Then tile 1 contains an image element.
        And tile 2 contains an image element.
        And tile 3 contains an image element.
        And tile 4 contains no image element.

    Scenario: Load a Tiled map with multiple layers.
        Given a Tiled map called two_layers.tmx,
        When the Tiled map is loaded,
        Then there exist 2 layers of tiles.
        And tile 1 overlaps tile 5.
        And tile 2 overlaps tile 6.
        And tile 3 overlaps tile 7.
        And tile 4 overlaps tile 8.

    Scenario: Adaptor bundles are created correctly.
        Given a Tiled map called one_blank.tmx,
        When the Tiled map is loaded,
        And the Tiled map has been converted to a rendered map,
        Then tile 1 is in the rendered map.
        And tile 2 is in the rendered map.
        And tile 3 is in the rendered map.
        And tile 4 is not in the rendered map.

    Scenario: Absolute paths starting at assets are correctly trimmed to be Bevy-friendly.
        Given an absolute asset path of assets/textures/environments/atlas_64x.png,
        When the absolute path is trimmed,
        Then the trimmed path should be textures/environments/atlas_64x.png.

    Scenario: Absolute paths starting before assets are correctly trimmed to be Bevy-friendly.
        Given an absolute asset path of junk/stuff/files/assets/textures/environments/atlas_64x.png,
        When the absolute path is trimmed,
        Then the trimmed path should be textures/environments/atlas_64x.png.

    Scenario: Absolute paths with multiple instances of the word assets are correctly trimmed to be Bevy-friendly.
        Given an absolute asset path of stuff/files/assets/textures/image-assets/environments/atlas_64x.png,
        When the absolute path is trimmed,
        Then the trimmed path should be textures/image-assets/environments/atlas_64x.png.

    # Tiled  ->  Bevy
    # [1][2]    [3][4]
    # [3][4]    [1][2]
    Scenario: Tiled (Y-Down) tiles are converted to Bevy (Y-Up) axis alignment.
        Given a Tiled map called single_sprite_sheet.tmx,
        When the Tiled map is loaded,
        And the Tiled map has been converted to a rendered map,
        Then Tiled tile 1 overlaps Bevy tile 3.
        And Tiled tile 2 overlaps Bevy tile 4.
        And Tiled tile 3 overlaps Bevy tile 1.
        And Tiled tile 4 overlaps Bevy tile 2.