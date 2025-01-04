# Assumptions:
# Window width = 1280
# Window height = 720
# Map width = 21
# Map height = 21

Feature: Camera
    Scenario: The Camera has coordinates that are center-based.
        Given a Tiled map called follow_player_test.tmx,
        When the map is spawned,
        Then the camera's x and y positions should be 672px, 672px.

    Scenario: The Player has coordinates that are bottom-left corner based.
        Given a Tiled map called follow_player_test.tmx,
        When the map is spawned,
        Then the player's x and y positions should be 640px, 640px.

    Scenario: Camera is centered on player tile center by default.
        Given a Tiled map called follow_player_test.tmx,
        When the map is spawned,
        Then the camera's position and player tile's center position are both 672px, 672px.

    Scenario: If the player moves right, the camera also moves right.

    Scenario: If the player moves left, the camera also moves left.

    Scenario: If the player moves up, the camera also moves up.

    Scenario: If the player moves down, the camera also moves down.
