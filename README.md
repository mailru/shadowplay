# About

Shadowplay is a utility that has the functionality of checking pappet syntax, a pappet manifest linter, a pretty printer, and a utility for
exploring the Hiera.

# Usage

## Correctness of YAML files

    shadowplay check yaml hieradata/default.yaml [hieradata/another.yaml] ...

In addition to the correctness of the syntax, the uniqueness of the keys in maps will be checked, as well as the correctness of the links
(anchors).

## Validity of Hiera YAML files

    shadowplay check hiera hieradata/default.yaml ...
    
For the specified files, YAML correctness will be checked, as well as the correctness of references to Puppet classes and class arguments.
For example, there will be an error generated if an unknown class argument is used.

As a side effect, it also checks the correctness of syntax of pappet manifests referenced by values ​​in Hiera.

## Linter of Puppet manifest files

    shadowplay check pp modules/hammer/manifests/config.pp ...

The specified files will be processed by the parser, then linter checks will be applied to the resulting AST (if parsing is successful).

## Pretty printing manifest file

    shadowplay pretty-print < /path/to/file.pp
