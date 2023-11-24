# CLI
This is the command-line interface (CLI) tool for the `arkley`. 

## Table of Contents
- [General Usage](#general-usage)
- [Command Usage](#command-usage)
  - [Quadratic Command](#quadratic-command)
    - [Options](#options)
  - [Evaluate](#evaluate)
  - [Rearrange](#rearrange)
  - [Solve](#solve)  
- [Additional Options](#additional-options)
  - [Language](#language)
  - [Context](#context)

# General Usage
For general usage information and available commands, you can use the following command:

```bash
arkley --help
```
Or
```bash
arkley <COMMAND> --help
```
 
# Command Usage

## Quadratic

The `quadratic`` command handles various operations related to quadratic equations. It supports two ways of providing input:

1. Using `-i` option for a single input string:

```bash
arkley quadratic -i <input> <SUBCOMMAND>
```

2. Using individual options `-a`, `-b`, and `-c` for coefficients. *Any unnecessary arguments will be ignored.*: 

```bash
arkley quadratic -a <a_value> -b <b_value> -c <c_value> <SUBCOMMAND>
```

### Options 
For each calculation replace `<SUBCOMMAND>` with the given command

- To calculate roots: `roots`
- To calculate the discriminant: `discriminant`
- To calculate the sum of root: `sum-of-roots`
- To calculate the product of roots: `product-of-roots`
- To calculate the axis of symmetry: `axis-of-symmetry`
- To determine the concavity: `concavity`

## Evaluate
To evaluate a mathematical expression

```bash
arkley evaluate -e <EXPR>
```

## Rearrange
To rearrange an equation to isolate a variable

```bash
arkley rearrange -e "<EQUATION_OR_INEQUALITY>" -t "<variable>"
```


## Solve
To solve an equation for a specific variable:

```bash
arkley solve -e "<EQUATION_OR_INEQUALITY>"
```
