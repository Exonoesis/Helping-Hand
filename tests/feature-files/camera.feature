Feature: Camera
    Scenario: The Camera has coordinates that are center-based.
        Given a Tiled map called follow_player_test.tmx,
        When the map is spawned,
        Then the camera's x and y positions should be 736px, 480px.

    Scenario: The Player has coordinates that are bottom-left corner based.
        Given a Tiled map called follow_player_test.tmx,
        When the map is spawned,
        Then the player's x and y positions should be 704px, 448px.

    Scenario: Camera is centered on player tile center by default.
        Given a Tiled map called follow_player_test.tmx,
        When the map is spawned,
        Then the camera's position and player tile's center position are both 736px, 480px.

    ##################################################################################
    #               Tests for map taller and wider than the camera                   #
    ##################################################################################

    Scenario: On a map taller and wider than the camera; if the player moves right, the camera also moves right.
        Given a custom game resolution of 1280 x 720,
        And a Tiled map called follow_player_test.tmx,
        When the player moves right,
        Then the camera's position and player tile's center position are both 800px, 480px.

    Scenario: On a map taller and wider than the camera; if the player moves left, the camera also moves left.
        Given a custom game resolution of 1280 x 720,
        And a Tiled map called follow_player_test.tmx,
        When the player moves left,
        Then the camera's position and player tile's center position are both 672px, 480px.

    Scenario: On a map taller and wider than the camera; if the player moves up, the camera also moves up.
        Given a custom game resolution of 1280 x 720,
        And a Tiled map called follow_player_test.tmx,
        When the player moves up,
        Then the camera's position and player tile's center position are both 736px, 544px.

    Scenario: On a map taller and wider than the camera; if the player moves down, the camera also moves down.
        Given a custom game resolution of 1280 x 720,
        And a Tiled map called follow_player_test.tmx,
        When the player moves down,
        Then the camera's position and player tile's center position are both 736px, 416px.

    Scenario: On a map taller and wider than the camera; the camera does not go beyond the left edge of the map.
        Given a custom game resolution of 1280 x 720,
        And a Tiled map called follow_player_test.tmx,
        When the player moves left,
        And the player moves left,
        And the player moves left,
        Then the player's center x and y positions should be 544px, 480px.
        And the camera's x and y positions should be 608px, 480px.

    Scenario: On a map taller and wider than the camera; the camera does not go beyond the right edge of the map.
        Given a custom game resolution of 1280 x 720,
        And a Tiled map called follow_player_test.tmx,
        When the player moves right,
        And the player moves right,
        Then the player's center x and y positions should be 864px, 480px.
        And the camera's x and y positions should be 800px, 480px.

    Scenario: On a map taller and wider than the camera; the camera does not go beyond the top edge of the map.
        Given a custom game resolution of 1280 x 720,
        And a Tiled map called follow_player_test.tmx,
        When the player moves up,
        And the player moves up,
        Then the player's center x and y positions should be 736px, 608px.
        And the camera's x and y positions should be 736px, 568px.

    Scenario: On a map taller and wider than the camera; the camera does not go beyond the bottom edge of the map.
        Given a custom game resolution of 1280 x 720,
        And a Tiled map called follow_player_test.tmx,
        When the player moves down,
        And the player moves down,
        And the player moves down,
        Then the player's center x and y positions should be 736px, 288px.
        And the camera's x and y positions should be 736px, 328px.
