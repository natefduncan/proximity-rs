# Proximity 
POI proximity rankings. Algo adapted from [density plot R package](https://cran.r-project.org/web/packages/pointdensityP/index.html).

## Installation

`cargo install --path .`

## Usage

Designed to (almost) fit nicely with [goose-rs](https://github.com/natefduncan/goose-rs) and [leaflet-rs](https://github.com/natefduncan/leaflet-rs).

For example:

```
goose parks mesa,az -d 50 -f csv > parks.csv && \
goose restaurants mesa,az -d 50 -f csv > restaurants.csv && \
proximity TBD | \
leaflet --colors "#2a9d8f,#f4a261" > map.html &&
open map.html
```

## Help
```
Proximity 0.1.1
Nate D.
Proximity score based on desirable and undesirables.

USAGE:
    proximity [OPTIONS] [FILE]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -g, --grid-size <GRID-SIZE>    Set binning size (km) in circle area. Default is 1km.
    -n, --n <N>                    Top N records to return. If not supplied, will return all.
    -r, --radius <RADIUS>          Set circle radius (km) for circle area. Default is 5km.
    -x, --x <x>                    Name of x value in category column.
    -y, --y <y>                    Name of y value in category column.

ARGS:
    <FILE>    Path to CSV. If not supplied, will look to STDIN.
```





