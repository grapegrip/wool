# wool

Extensible [grip](https://github.com/joeyespo/grip) clone

## Usage
```
USAGE:
    wool [FLAGS] <infile> [outfile]

FLAGS:
    -b, --browser             open in browser
    -e, --export              export html
    -h, --help                Prints help information
    -s, --highlight           syntax highlighting
    -n, --no-preview-frame    Don't render the preview frame
    -V, --version             Prints version information

ARGS:
    <infile>     Sets the input file to use
    <outfile>    Sets the output file to use
```

#### Example

preview on localhost:   
`wool readme.md` 

export to html:    
`wool readme.md --export mypreview.html`

###### Experimental

Syntax highlighting:   
`wool readme.md -s`
