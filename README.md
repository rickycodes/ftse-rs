## ftse-rs [![Build Status](https://travis-ci.org/rickycodes/ftse-rs.svg?branch=master)](https://travis-ci.org/rickycodes/ftse-rs)

scrape and filter hl.co.uk market summaries

#### USAGE:
```
ftse [FLAGS] [OPTIONS]

FLAGS:
    -f, --fallers    Limit the results to fallers only
    -h, --help       Prints help information
    -r, --risers     Limit the results to risers only
    -t, --table      Render output as table
    -V, --version    Prints version information

OPTIONS:
    -l, --limit <limit>      Limit the results to a specific number
    -m, --market <market>    Specify which market, either aim or 100 (ftse-100) [default: aim]
``` 

### FEATURES:

<a href="https://stedolan.github.io/jq/">`jq`</a> compatible:

```
ftse | jq '.[0]'
{
  "epic": "ABDP",
  "name": "AB Dynamics plc",
  "price": "0.00",
  "change_amount": "0.00",
  "change_percent": "0.00%"
}
```

"fancy" table output:

```
ftse -l=4 -t
+------+--------------------------------------+--------+----------+--------+
| EPIC | NAME                                 | CHANGE | CHANGE % | PRICE  |
+------+--------------------------------------+--------+----------+--------+
| ABDP | AB Dynamics plc                      | 0.00   | 0.00%    | 0.00   |
+------+--------------------------------------+--------+----------+--------+
| ABC  | Abcam plc                            | +46.00 | +3.65%   | 0.00   |
+------+--------------------------------------+--------+----------+--------+
| AMS  | Advanced Medical Solutions Group plc | +5.00  | +2.22%   | 230.00 |
+------+--------------------------------------+--------+----------+--------+
| APH  | Alliance Pharma Plc                  | -1.70  | -2.30%   | 72.30  |
+------+--------------------------------------+--------+----------+--------+
```
