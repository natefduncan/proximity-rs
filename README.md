# Proximity 
POI proximity rankings. Algo adapted from [density plot R package](https://cran.r-project.org/web/packages/pointdensityP/index.html).

## Installation

`cargo install --path .`

## Usage

Designed to (almost) fit nicely with [goose-rs](https://github.com/natefduncan/goose-rs) and [leaflet-rs](https://github.com/natefduncan/leaflet-rs).

For example:

```
goose parks,restaurants mesa,az -d 25 -f csv > test.csv && \
proximity test.csv -c search -x restaurants -y parks --output-y -n 10 | \
leaflet --colors "#2a9d8f,#f4a261" > map.html && \ 
open map.html
```

## Help
```
Proximity 0.1.3
Nate D.
Proximity score: which X points are closest to the most Y points?

USAGE:
    proximity [FLAGS] [OPTIONS] [ARGS]

FLAGS:
    -h, --help        Prints help information
        --output-y    Boolean. True will include all y values with score zero. False will only return x values.
    -V, --version     Prints version information

OPTIONS:
    -c, --c <CATEGORY>             Category column name. Default is 'category'.
    -g, --grid-size <GRID-SIZE>    Set binning size (km) in circle area. Default is 1km.
    -n, --n <N>                    Top N records to return. If not supplied, will return all.
    -r, --radius <RADIUS>          Set circle radius (km) for circle area. Default is 5km.
    -x, --x <x>                    Name of x value in category column.
    -y, --y <y>                    Name of y value in category column.

ARGS:
    <FILE_X>    Path to CSV. If not supplied, will look to STDIN.
    <FILE_Y>    Path to CSV. If not supplied, will assume x and y are in FILE1.
```





