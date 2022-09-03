# parquet-cat
A command-line tool to explore Parquet files on S3, with folder support

Usage:
```
pcat 0.1.0
A command-line tool to explore Parquet files on S3, with folder support

USAGE:
    pcat [OPTIONS] <URL>

ARGS:
    <URL>    The path to the S3 file to be processed

OPTIONS:
    -h, --help                     Print help information
    -p, --profile <AWS_PROFILE>    AWS profile to use for credentials, support assuming roles
    -q, --query <QUERY>            SQL query running against the file [default: "select * from tbl"]
    -V, --version                  Print version information
```
