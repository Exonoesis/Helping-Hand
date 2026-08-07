Feature: Map Changing
    Scenario: A player transitions to a different map
        Given a Tiled map called transition_test.tmx,
        And a map size of 3 x 3 tiles,
        And the Player is at 2,2,
        When the player interacts with the tile ahead of them,
        Then the map size should be 16 x 11 tiles,
        And the Player should be at 7,9.

    Scenario: A player doesn't transition to a different map
        Given a Tiled map called no_transition_test.tmx,
        And a map size of 3 x 3 tiles,
        And the Player is at 2,1,
        When the player interacts with the tile ahead of them,
        Then the map size should be 3 x 3 tiles,
        And the Player should be at 2,1.

    Scenario: A map with no gaps renders correctly
        Given a Tiled map called single_sprite_sheet.tmx,
        When the Tiled map is loaded,
        Then the map size should be 2 x 2 tiles,
        And there should be 4 tiles

    Scenario: A map with a gap renders correctly
        Given a Tiled map called one_blank.tmx,
        When the Tiled map is loaded,
        Then the map size should be 2 x 2 tiles,
        And there should be 3 tiles

    #   Tiled  ->    Bevy
    #    0  1        0  1
    # 0 [1][2]    1 [3][4]
    # 1 [3][4]    0 [1][2]
    Scenario: Tiled (Y-Down) tiles are converted to Bevy (Y-Up) axis alignment.
        Given a Tiled map called single_sprite_sheet.tmx,
        When the Tiled map is loaded,
        Then the tile at grid coordinate 0,0,0 has a pixel coordinate of 0,64,0.
        And the tile at grid coordinate 1,0,0 has a pixel coordinate of 64,64,0.
        And the tile at grid coordinate 0,1,0 has a pixel coordinate of 0,0,0.
        And the tile at grid coordinate 1,1,0 has a pixel coordinate of 64,0,0.
