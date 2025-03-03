Feature: Interactives
    Scenario: Interactive markers are being found from a Tiled map.
        Given a Tiled map called object_test.tmx,
        When the Tiled map is loaded,
        And an Interactive Collection is extracted from the Tiled map,
        Then there are 2 interactive markers in the collection.
        And marker 1 has a position of 0,0,2.
        And marker 1 has a size of 64x64.
        And marker 2 has a position of 64,128,2.
        And marker 2 has a size of 128x64.

    Scenario: A position left of the marker reports as lower.
        Given a Tiled map called marker_test.tmx,
        And a position of 0,63,
        When the Tiled map is loaded,
        And an Interactive Collection is extracted from the Tiled map,
        Then there is 1 interactive marker in the collection.
        And the position reports lower on the marker.

    Scenario: A position right of the marker reports as higher.
        Given a Tiled map called marker_test.tmx,
        And a position of 128,64,
        When the Tiled map is loaded,
        And an Interactive Collection is extracted from the Tiled map,
        Then there is 1 interactive marker in the collection.
        And the position reports higher on the marker.

    Scenario: A position above the marker reports as lower.
        Given a Tiled map called marker_test.tmx,
        And a position of 63,0,
        When the Tiled map is loaded,
        And an Interactive Collection is extracted from the Tiled map,
        Then there is 1 interactive marker in the collection.
        And the position reports lower on the marker.

    Scenario: A position below the marker reports as higher.
        Given a Tiled map called marker_test.tmx,
        And a position of 64,128,
        When the Tiled map is loaded,
        And an Interactive Collection is extracted from the Tiled map,
        Then there is 1 interactive marker in the collection.
        And the position reports higher on the marker.

    Scenario: A position on the marker reports as match.
        Given a Tiled map called marker_test.tmx,
        And a position of 64,64,
        When the Tiled map is loaded,
        And an Interactive Collection is extracted from the Tiled map,
        Then there is 1 interactive marker in the collection.
        And the position reports match on the marker.

    Scenario: A marker for a certain position is found.
        Given a Tiled map called multiple_marker_test.tmx,
        And a position of 64,0,
        When the Tiled map is loaded,
        And an Interactive Collection is extracted from the Tiled map,
        And a marker is requested for the position,
        Then the marker has a position of 64,0,2.
        And the marker has a size of 64x64.

    Scenario: A marker is not found for a certain position.
        Given a Tiled map called multiple_marker_test.tmx,
        And a position of 0,0,
        When the Tiled map is loaded,
        And an Interactive Collection is extracted from the Tiled map,
        And a marker is requested for the position,
        Then it reported there is no marker.

    Scenario: A marker at the beginning of the list is found.
        Given a Tiled map called multiple_marker_test.tmx,
        And a position of 32,96,
        When the Tiled map is loaded,
        And an Interactive Collection is extracted from the Tiled map,
        And a marker is requested for the position,
        Then the marker has a position of 0,64,2.
        And the marker has a size of 64x64.

    Scenario: A marker at the middle of the list is found.
        Given a Tiled map called multiple_marker_test.tmx,
        And a position of 96,96,
        When the Tiled map is loaded,
        And an Interactive Collection is extracted from the Tiled map,
        And a marker is requested for the position,
        Then the marker has a position of 64,64,2.
        And the marker has a size of 64x64.

    Scenario: A marker at the end of the list is found.
        Given a Tiled map called multiple_marker_test.tmx,
        And a position of 160,160,
        When the Tiled map is loaded,
        And an Interactive Collection is extracted from the Tiled map,
        And a marker is requested for the position,
        Then the marker has a position of 128,128,2.
        And the marker has a size of 64x64.

    Scenario: A marker has a given interactive type
        Given a Tiled map called multiple_marker_test.tmx,
        And a position of 32,96,
        When the Tiled map is loaded,
        And an Interactive Collection is extracted from the Tiled map,
        And a marker is requested for the position,
        Then the marker has the type Transition.
        And the Transition marker has a path of Below Player.
