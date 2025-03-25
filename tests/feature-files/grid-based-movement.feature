Feature: Grid-based Movement
    Scenario: A Player is requested to move to the right
        Given a Tiled map called player_3x3.tmx,
        And the Player is at 1,1,1,
        When the Player is requested to move right,
        Then the Player should have a Target.
        And the Player is facing right.

    Scenario: A Player moves to the right
        Given a Tiled map called player_3x3.tmx,
        And the Player is at 1,1,1,
        When the Player moves right,
        Then the Player's pixel coordinates are equivalent to tile 2,1,1.
        And the Player's grid coordinates are set to tile 2,1,1.
        And the Player is facing right.

    Scenario: A Player moves to the left
        Given a Tiled map called player_3x3.tmx,
        And the Player is at 1,1,1,
        When the Player moves left,
        Then the Player's pixel coordinates are equivalent to tile 0,1,1.
        And the Player's grid coordinates are set to tile 0,1,1.
        And the Player is facing left.

    Scenario: A Player moves up
        Given a Tiled map called player_3x3.tmx,
        And the Player is at 1,1,1,
        When the Player moves up,
        Then the Player's pixel coordinates are equivalent to tile 1,0,1.
        And the Player's grid coordinates are set to tile 1,0,1.
        And the Player is facing up.

    Scenario: A Player moves down
        Given a Tiled map called player_3x3.tmx,
        And the Player is at 1,1,1,
        When the Player moves down,
        Then the Player's pixel coordinates are equivalent to tile 1,2,1.
        And the Player's grid coordinates are set to tile 1,2,1.
        And the Player is facing down.

    Scenario: A Player cannot move right
        Given a Tiled map called collision_test.tmx,
        And the Player is at 1,1,1,
        When the Player moves right,
        Then the Player's pixel coordinates are equivalent to tile 1,1,1.
        And the Player's grid coordinates are set to tile 1,1,1.
        And the Player is facing right.

    Scenario: A Player cannot move left
        Given a Tiled map called collision_test.tmx,
        And the Player is at 1,1,1,
        When the Player moves left,
        Then the Player's pixel coordinates are equivalent to tile 1,1,1.
        And the Player's grid coordinates are set to tile 1,1,1.
        And the Player is facing left.

    Scenario: A Player cannot move up
        Given a Tiled map called collision_test.tmx,
        And the Player is at 1,1,1,
        When the Player moves up,
        Then the Player's pixel coordinates are equivalent to tile 1,1,1.
        And the Player's grid coordinates are set to tile 1,1,1.
        And the Player is facing up.

    Scenario: A Player cannot move down
        Given a Tiled map called collision_test.tmx,
        And the Player is at 1,1,1,
        When the Player moves down,
        Then the Player's pixel coordinates are equivalent to tile 1,1,1.
        And the Player's grid coordinates are set to tile 1,1,1.
        And the Player is facing down.

    Scenario: A Player cannot move past the left edge of the map.
        Given a Tiled map called player_bounds_test.tmx,
        And the Player is at 0,0,1,
        When the Player moves left,
        Then the Player's grid coordinates are set to tile 0,0,1.
        And the Player's pixel coordinates are equivalent to tile 0,0,1.
        And the Player is facing left.

    Scenario: A Player cannot move past the right edge of the map.
        Given a Tiled map called player_bounds_test.tmx,
        And the Player is at 0,0,1,
        When the Player moves right,
        Then the Player's grid coordinates are set to tile 0,0,1.
        And the Player's pixel coordinates are equivalent to tile 0,0,1.
        And the Player is facing right.

    Scenario: A Player cannot move past the top edge of the map.
        Given a Tiled map called player_bounds_test.tmx,
        And the Player is at 0,0,1,
        When the Player moves up,
        Then the Player's grid coordinates are set to tile 0,0,1.
        And the Player's pixel coordinates are equivalent to tile 0,0,1.
        And the Player is facing up.

    Scenario: A Player cannot move past the bottom edge of the map.
        Given a Tiled map called player_bounds_test.tmx,
        And the Player is at 0,0,1,
        When the Player moves down,
        Then the Player's grid coordinates are set to tile 0,0,1.
        And the Player's pixel coordinates are equivalent to tile 0,0,1.
        And the Player is facing down.
