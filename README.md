# rhumb

## Goals
Given two points A and B on the Earth:
* Generate geojson lines (with interpolated points) for the rhumb and great circle paths
    * Work-around for inconsistencies and ambiguities with geojson viewers - e.g. when using Mercator projection may cause a line between just point A and B to display the rhumb line, but the great circle when rendering the globe in 3D.
* Calculate difference in distances between the rhumb and great circle path
