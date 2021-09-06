# marin
## Running debug REPL
`cargo run --example repl`

## Examples
### Positional Arguments
`1 2 3`  
`arg1 arg2 arg3` 

### Keyword Arguments
`kw: 1`  
`keywordarg: True` 

### Flags
`-flag1 -flag2`  

### Quoted Strings
`kw: "string with spaces`  
`positional argument with spaces`  

### Ranges
`range: 1..10`  
`1..10`  
`..10`  
`ids: -10..20`  

### Lists
`vals: ["val1", "val2"]`  
`vals: [1, 2, 3]`  
`[1,2,3]`  
