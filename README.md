# About

Pimperle is a puppet from medieval Europe, the prototype of the Russian Petrushka. In performances, the character taught to see stupidity.

The Pimperle application is a utility that has the functionality of checking Puppet syntax, a linter of Puppet manifests, and a utility for
exploring the Hiera.

# Usage

## Correctness of YAML files

    pimprle check yaml hieradata/default.yaml [hieradata/another.yaml] ...

In addition to the correctness of the syntax, the uniqueness of the keys in maps will be checked, as well as the correctness of the links
(anchors).

## Validity of Hiera YAML files

    pimprle check hiera hieradata/default.yaml ...
    
For the specified files, YAML correctness will be checked, as well as the correctness of references to Puppet classes and class arguments.
For example, there will be an error generated if an unknown class argument is used.

As a side effect, it also checks the correctness of syntax of pappet manifests referenced by values ​​in Hiera.

## Static analyzer *.pp

    pimprle check pp modules/hammer/manifests/config.pp ...

The specified files will be processed by the parser, then linter checks will be applied to the resulting AST (if parsing is successful).
