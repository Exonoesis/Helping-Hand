Feature: Grid-based Movement
    Scenario: A player is requested to move to the right
        Given a Tiled map called player.tmx,
        And the player is at 0,1,1,
        When the Player is requested to move to the right,
        Then the Player should have a Target.

    Scenario: A player moves to the right
        Given a Tiled map called player.tmx,
        And the player is at 0,1,1,
        When the Player moves to the right,
        Then the Player's pixel coordinates are equivalent to tile 1,1,1.

    Scenario: A player moves to the left
        Given a Tiled map called player_left.tmx,
        And the player is at 1,0,1,
        When the Player moves to the left,
        Then the Player's pixel coorinates are equivalent to tile 0,0,1.
