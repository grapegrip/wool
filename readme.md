# asd

Extensible [grip](https://github.com/joeyespo/grip) clone

## Usage
```
USAGE:
    asd [FLAGS] <infile> [outfile]

FLAGS:
    -e, --export     export html
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <infile>     Sets the input file to use
    <outfile>    
```

#### Example

preview on localhost:   
`asd readme.md` 

export to html:    
`asd readme.md --export mypreview.html`

###### Experimental

Syntax highlighting:   
`asd readme.md -h`
