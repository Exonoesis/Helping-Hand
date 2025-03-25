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
        And tile 0,0,0 points to spritesheet atlas_64x.png.
        And tile 0,0,0's spritesheet has dimensions 1024 x 3072.
        And tile 1,0,0 points to spritesheet atlas_64x.png.
        And tile 1,0,0's spritesheet has dimensions 1024 x 3072.
        And tile 0,1,0 points to spritesheet atlas_64x.png.
        And tile 0,1,0's spritesheet has dimensions 1024 x 3072.
        And tile 1,1,0 points to spritesheet atlas_64x.png.
        And tile 1,1,0's spritesheet has dimensions 1024 x 3072.
        And tile 0,0,0 points to image number 1.
        And tile 1,0,0 points to image number 5.
        And tile 0,1,0 points to image number 49.
        And tile 1,1,0 points to image number 53.


    Scenario: Tile textures correctly load from multiple sprite sheets.
        Given a Tiled map called multiple_sprite_sheet.tmx,
        When the Tiled map is loaded,
        Then there are 4 tiles loaded.
        And tile 0,0,0 points to spritesheet !CL_DEMO_64.png.
        And tile 1,0,0 points to spritesheet !CL_DEMO_64.png.
        And tile 0,1,0 points to spritesheet atlas_64x.png.
        And tile 1,1,0 points to spritesheet atlas_64x.png.
        And tile 0,0,0 points to image number 131.
        And tile 1,0,0 points to image number 128.
        And tile 0,1,0 points to image number 115.
        And tile 1,1,0 points to image number 164.

    Scenario: Map loads correctly when some tiles have no image.
        Given a Tiled map called one_blank.tmx,
        When the Tiled map is loaded,
        Then tile 0,0,0 contains an image element.
        And tile 1,0,0 contains an image element.
        And tile 0,1,0 contains an image element.
        And tile 1,1,0 contains no image element.

    Scenario: Load a Tiled map with multiple layers.
        Given a Tiled map called two_layers.tmx,
        When the Tiled map is loaded,
        Then there exist 2 layers of tiles.
        And tile 0,0,0 overlaps tile 0,0,1.
        And tile 1,0,0 overlaps tile 1,0,1.
        And tile 0,1,0 overlaps tile 0,1,1.
        And tile 1,1,0 overlaps tile 1,1,1.

    Scenario: Adaptor bundles are created correctly.
        Given a Tiled map called one_blank.tmx,
        When the Tiled map is loaded,
        And the Tiled map has been converted to a rendered map,
        Then there should be 4 rendered tiles created.

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

    #   Tiled  ->    Bevy
    #    0  1        0  1
    # 0 [1][2]    1 [3][4]
    # 1 [3][4]    0 [1][2]
    Scenario: Tiled (Y-Down) tiles are converted to Bevy (Y-Up) axis alignment.
        Given a Tiled map called single_sprite_sheet.tmx,
        When the Tiled map is loaded,
        And the Tiled map has been converted to a rendered map,
        Then Tiled tile 0,0,0 is equivalent to Bevy tile 0,1,0.
        And Tiled tile 1,0,0 is equivalent to Bevy tile 1,1,0.
        And Tiled tile 0,1,0 is equivalent to Bevy tile 0,0,0.
        And Tiled tile 1,1,0 is equivalent to Bevy tile 1,0,0.

    Scenario: A player is found on the Tiled map.
        Given a Tiled map called player_2x3.tmx,
        When the Tiled map is loaded,
        Then there is 1 player in the Tiled map.
        And that player is at tile 0,1,1.

    Scenario: A player is found on the Rendered map.
        Given a Tiled map called player_2x3.tmx,
        When the Tiled map is loaded,
        And the Tiled map has been converted to a rendered map,
        Then there is 1 player in the Rendered map.
        And that player on the Rendered map is at tile 0,1,1.

    Scenario: Translate 3D cords to 1D cords.
        Given a Tiled map called player_2x3.tmx,
        When the Tiled map is loaded,
        Then 3D cords 0,1,1 point to tile index 8.
