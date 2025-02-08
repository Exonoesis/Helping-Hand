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
        And a position of 0,64,
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
        And a position of 64,0,
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