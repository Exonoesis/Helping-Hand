Feature: Grid-based Movement
    Scenario: A player is requested to move to the right
        Given a Tiled map called player_3x3.tmx,
        And the player is at 1,1,1,
        When the Player is requested to move right,
        Then the Player should have a Target.

    Scenario: A player moves to the right
        Given a Tiled map called player_3x3.tmx,
        And the player is at 1,1,1,
        When the Player moves right,
        Then the Player's pixel coordinates are equivalent to tile 2,1,1.
        And the Player's grid coordinates has changed to tile 2,1,1.

    Scenario: A player moves to the left
        Given a Tiled map called player_3x3.tmx,
        And the player is at 1,1,1,
        When the Player moves left,
        Then the Player's pixel coordinates are equivalent to tile 0,1,1.
        And the Player's grid coordinates has changed to tile 0,1,1.

    Scenario: A player moves up
        Given a Tiled map called player_3x3.tmx,
        And the player is at 1,1,1,
        When the Player moves up,
        Then the Player's pixel coordinates are equivalent to tile 1,0,1.
        And the Player's grid coordinates has changed to tile 1,0,1.

    Scenario: A player moves down
        Given a Tiled map called player_3x3.tmx,
        And the player is at 1,1,1,
        When the Player moves down,
        Then the Player's pixel coordinates are equivalent to tile 1,2,1.
        And the Player's grid coordinates has changed to tile 1,2,1.
