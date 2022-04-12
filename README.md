
# Table of Contents

1.  [About](#orga7083b9)
2.  [Usage](#org6bd607e)
    1.  [Correctness of YAML files](#org7734830)
    2.  [Validity of Hiera YAML files](#org3f5c338)
    3.  [Linter of Puppet manifest files](#orgdc56b99)
    4.  [Pretty printing manifest file](#org979298b)
    5.  [Config file generator](#org6e15fc1)
    6.  [Hiera explorer](#org65220b9)
    7.  [\*.pp AST dumper](#orge17825e)
3.  [Available lints for \*.pp](#org1d70474)
    1.  [ArgumentLooksSensitive](#org146590b)
    2.  [ArgumentTyped](#orgf0a6a72)
    3.  [ConstantExpressionInCondition](#org5ee25b3)
    4.  [DefaultCaseIsNotLast](#orgd6eee23)
    5.  [DoNotUseUnless](#org70645cf)
    6.  [DoubleNegation](#org7b9218b)
    7.  [EmptyCasesList](#org92de2b2)
    8.  [EnsureAttributeIsNotTheFirst](#org41dff14)
    9.  [ErbReferencesToUnknownVariable](#org50aaaee)
    10. [ExecAttributes](#org736dcec)
    11. [ExpressionInSingleQuotes](#org9e2ef0d)
    12. [FileModeAttributeIsString](#org509970b)
    13. [InvalidResourceCollectionInvocation](#org0ce5e5e)
    14. [InvalidResourceSetInvocation](#org331eb6a)
    15. [InvalidStringEscape](#orgfb7625f)
    16. [InvalidVariableAssignment](#orgbbd6045)
    17. [LowerCaseArgumentName](#orgcd60075)
    18. [LowerCaseVariable](#orgdb8166d)
    19. [MultipleDefaultCase](#orge811961)
    20. [MultipleResourcesWithoutDefault](#org9aa9fc9)
    21. [NegationOfEquation](#org9597418)
    22. [NoDefaultCase](#orgb4c76c6)
    23. [OptionalArgumentsGoesFirst](#orge1cdf50)
    24. [PerExpressionResourceDefaults](#orgaa5f3b4)
    25. [ReadableArgumentsName](#org703a1dd)
    26. [ReferenceToUndefinedValue](#org047c996)
    27. [RelationToTheLeft](#org2aa6384)
    28. [SelectorInAttributeValue](#org3677780)
    29. [SensitiveArgumentWithDefault](#org47ef84d)
    30. [StatementWithNoEffect](#orgf0b80f9)
    31. [UnconditionalExec](#org3a62ad7)
    32. [UniqueArgumentsNames](#org8b95cbd)
    33. [UniqueAttributeName](#org8db2bdf)
    34. [UnusedVariables](#org760446f)
    35. [UpperCaseName](#org4375ef7)
    36. [UselessDoubleQuotes](#org4eff26b)
    37. [UselessParens](#orgae4872f)
4.  [Linter for YAML files](#orga0ae0ef)
5.  [Linter for Hiera YAML files](#orgf2a84fb)
    1.  [Reference to a module which has syntax errors](#org9ff9528)
    2.  [Reference to class which is not found in modules/](#org3f03ae9)
    3.  [Reference in undefined class argument](#orgf2c89bb)
    4.  [Single column in the name of key of root map](#orge25bd0c)


<a id="orga7083b9"></a>

# About

Shadowplay is a utility for checking puppet syntax, a puppet manifest linter, a pretty printer, and a utility for exploring the Hiera.

![img](./doc/screenshot-emacs.png "Flycheck plugin for Emacs")


<a id="org6bd607e"></a>

# Usage


<a id="org7734830"></a>

## Correctness of YAML files

    shadowplay check yaml hieradata/default.yaml [hieradata/another.yaml] ...

In addition to the correctness of the syntax, the uniqueness of the keys in maps will be checked, as well as the correctness of the links
(anchors).


<a id="org3f5c338"></a>

## Validity of Hiera YAML files

    shadowplay check hiera hieradata/default.yaml ...

For the specified files, YAML correctness will be checked, as well as the correctness of references to Puppet classes and class arguments.
For example, there will be an error generated if an unknown class argument is used.

As a side effect, it also checks the correctness of syntax of pappet manifests referenced by values ​​in Hiera.


<a id="orgdc56b99"></a>

## Linter of Puppet manifest files

    shadowplay --repo-path ./ check pp modules/hammer/manifests/config.pp ...

The specified files will be processed by the parser, then linter checks will be applied to the resulting AST (if parsing is successful).


<a id="org979298b"></a>

## Pretty printing manifest file

    shadowplay pretty-print-pp < /path/to/file.pp


<a id="org6e15fc1"></a>

## Config file generator

Use may want to disable some lints or customize it. She can generate default config and edit it later with the command:

    shadowplay generate-config >/etc/shadowplay.yaml


<a id="org65220b9"></a>

## Hiera explorer

Hiera is hierarchy of yaml files. In huge configurations it may be difficult to determine value of specific key for some host. Shadowplay
provides easy solution.

    shadowplay get host123 sshd::install::version

Command prints as much information as possible:

    Value: "present"
    Found in "./hieradata/default_CentOS7.yaml" at lines 63:63
    Value lookup path was: network/host123.yaml -> host123.yaml -> host.yaml -> default_CentOS7.yaml
    ===================================
    Git information:
    deadbeef1234 (Evgenii Lepikhin 2022-03-29 15:06:51 +0300 63) sshd::install::version:             'present'


<a id="orge17825e"></a>

## \*.pp AST dumper

    shadowplay dump modules/sshd/manifests/install.pp

Outputs AST in JSON format. Mainly for internal purposes.


<a id="org1d70474"></a>

# Available lints for \*.pp


<a id="org146590b"></a>

## ArgumentLooksSensitive

Warns if argument name looks like sensitive, but argument is not typed with type Sensitive

Bad:

    class some::class (
      $secret_token,
    ) { }

Good:

    class some::class (
      Sensitive $secret_token,
    ) { }


<a id="orgf0a6a72"></a>

## ArgumentTyped

Warns if argument is not typed

Bad:

    class some::class (
      $config_path,
    ) { }

Good:

    class some::class (
      Stdlib::Absolutepath $config_path,
    ) { }


<a id="org5ee25b3"></a>

## ConstantExpressionInCondition

Warns if constant expression is used in condition

Bad:

    if 1 == 2 - 1 { notify('1=2-1') }

Such type of conditions always evaluated into constant false or true, thus can be safely removed. Good:

    notify('1=2-1')


<a id="orgd6eee23"></a>

## DefaultCaseIsNotLast

Warns if 'default' case is not the last

Bad:

    case $value {
      'a': { }
      default: { }
      'b': { }
    }

Good:

    case $value {
      'a': { }
      'b': { }
      default: { }
    }


<a id="org70645cf"></a>

## DoNotUseUnless

Warns if 'unless' conditional statement is used

Bad:

    unless $value { }

Good:

    if !$value { }


<a id="org7b9218b"></a>

## DoubleNegation

Warns if double negation is used

Bad:

    if !(!$value) { }
    
    if !($value != 1) { }

Good:

    if $value { }
    
    if $value == 1 { }


<a id="org92de2b2"></a>

## EmptyCasesList

Warns if case { &#x2026; } has no cases

Bad:

    case $value { }


<a id="org41dff14"></a>

## EnsureAttributeIsNotTheFirst

Warns if 'ensure' argument of resource is not the first

Bad:

    file { '/etc/passwd':
      user => root,
      ensure => file,
    }

Good:

    file { '/etc/passwd':
      ensure => file,
      user => root,
    }


<a id="org50aaaee"></a>

## ErbReferencesToUnknownVariable

Checks ERB templates specified in template() for undefined variables

Bad:

    class some::class () {
      # here template_file.erb contains: <% @some_undefined_variable %>
      $value = template('some/template_file.erb')
    }


<a id="org736dcec"></a>

## ExecAttributes

Checks exec { &#x2026;} arguments

Bad:

    # implicit 'command' attribute
    exec { 'echo Hello' : }
    
    exec {
      unknown_attribute => 1,
    }
    
    # invalid provider
    exec {
      provider => 'unknown provider value'
    }
    
    # 'path' is not set, 'provider' is not 'shell', thus 'command' attribute of exec {} must start with absolute path
    exec {
      command => 'echo Hello'
    }


<a id="org9e2ef0d"></a>

## ExpressionInSingleQuotes

Warns if interpolated expression found single-qouted string

Bad:

    $value = 'Hello $world'
    
    $value = '2 + 2 = ${2+2}'


<a id="org509970b"></a>

## FileModeAttributeIsString

Warns if argument 'mode' of 'file' resource is not in 4-digit string form

Bad:

    file { '/some/file':
      mode => '644',
    }
    
    file { '/some/file':
      mode => 644,
    }

Good:

    file { '/some/file':
      mode => '0644',
    }


<a id="org0ce5e5e"></a>

## InvalidResourceCollectionInvocation

Checks if existing resource set is used and all arguments are known in it's class

Bad:

    # relation to unknown resource
    Class['unknown_class'] -> Class['known_class']


<a id="org331eb6a"></a>

## InvalidResourceSetInvocation

Checks if existing resource is used and all arguments are known in it's class

Bad:

    class class1 (
      $known_arg,
    ) { }
    
    class class2 {
      # Call to unknown class
      class { 'unknown_class': }
    
      # Call to known class with invalid argument
      class { 'class1':
        unknown_arg => 1
      }
    
      # Call to known class with invalid argument
      class1 { 'title':
        unknown_arg => 1,
      }
    
      # Call to internal resource with invalid argument
      file { '/some/file':
        uknown_arg => 1,
      }
    }


<a id="orgfb7625f"></a>

## InvalidStringEscape

Checks if only allowed characters are escaped in strings

Bad:

    $value = '\s*\.'
    
    $value = "\s*\."

Good:

    $value = '\\s*\\.'
    
    $value = "\\s*\\."


<a id="orgbbd6045"></a>

## InvalidVariableAssignment

Warns if left part of assignment is not a variable or array of variables

Bad:

    lookup('some::value') = 1


<a id="orgcd60075"></a>

## LowerCaseArgumentName

Warns if argument name is not lowercase, as suggested by Puppet's style guide

Bad:

    class some::class (
      $ArgumentInCamelCase
    ) {}


<a id="orgdb8166d"></a>

## LowerCaseVariable

Warns if variable name is not lowercase

Bad:

    class some::class () {
      $VariableIsNOTInLowercase = 1


<a id="orge811961"></a>

## MultipleDefaultCase

Warns if case statement has multiple 'default' cases

Bad:

    case $val {
      1: {}
      default: {}
      default: {}
    }


<a id="org9aa9fc9"></a>

## MultipleResourcesWithoutDefault

Warns if resource set contains multiple resources and no defaults specified

Bad:

    file {
      '/etc/passwd':
        ensure => file,
        user => root,
      '/etc/group':
        ensure => file,
        user => root,
        group => wheel,
    }

Good:

    file {
      default:
        ensure => file,
        user => root,
      '/etc/passwd':
      '/etc/group':
        group => wheel,
    }


<a id="org9597418"></a>

## NegationOfEquation

Warns on negation of equation

Bad:

    if !($a == 1) { }
    
    if !($a =~ /./) { }

Good:

    if $a != 1 { }
    
    if $a !~ /./ { }


<a id="orgb4c76c6"></a>

## NoDefaultCase

Warns if case statement has no default case

Bad:

    case $val {
      1, 2: {  }
      3: { }
    }

Good:

    case $val {
      1, 2: {  }
      3: { }
      default: { }
    }


<a id="orge1cdf50"></a>

## OptionalArgumentsGoesFirst

Warns if optional argument specified before required

    class some::class (
      $optional_arg = 1,
      $required_arg,
    ) { }

Good:

    class some::class (
      $required_arg,
      $optional_arg = 1,
    ) { }


<a id="orgaa5f3b4"></a>

## PerExpressionResourceDefaults

Warns if local resource defaults are used

Bad:

    Exec {
      provider => shell,
    }
    
    exec { 'run command':
      command => 'echo Hello',
    }


<a id="org703a1dd"></a>

## ReadableArgumentsName

Warns if argument name is not readable enough

Bad:

    class some::class (
      String $c = '/etc/config',
    ) { }

Good:

    class some::class (
      String $config = '/etc/config',
    ) { }


<a id="org047c996"></a>

## ReferenceToUndefinedValue

Warns if variable is not defined in current context

Bad:

    if $some_undefined_variable { }


<a id="org2aa6384"></a>

## RelationToTheLeft

Checks for left-directed relations

Bad:

    Class['c'] <- Class['b'] <~ Class['a']

Good:

    Class['a'] ~> Class['b'] -> Class['c']


<a id="org3677780"></a>

## SelectorInAttributeValue

Warns if selector (&#x2026; ? &#x2026; : &#x2026;) used in resource attribute

Bad:

    file { '/etc/shadow':
      mode => $is_secure ? '0600' : '0644',
    }

Good:

    $file_mode = $is_secure ? '0600' : '0644'
    
    file { '/etc/shadow':
      mode => $file_mode,
    }


<a id="org47ef84d"></a>

## SensitiveArgumentWithDefault

Warns if argument typed with Sensitive contains default value

Bad:

    class some::class (
      Sensitive $password = 'admin',
    )

Public available default value for sensitive data is nonsense. Good:

    class some::class (
      Sensitive $password,
    )


<a id="orgf0b80f9"></a>

## StatementWithNoEffect

Checks for statements without side effects

Bad:

    if $a {
      if $b {
        2 + 2
      }
    }


<a id="org3a62ad7"></a>

## UnconditionalExec

Warns if exec { &#x2026; } is specified without unless, onlyif, creates or refreshonly attributes

Bad:

    exec { 'run command':
      command => '/bin/rm -rf /var/cache/myapp',
    }

Good:

    exec { 'run command':
      command => '/bin/rm -rf /var/cache/myapp',
      onlyif => 'test -e /var/cache/myapp',
    }


<a id="org8b95cbd"></a>

## UniqueArgumentsNames

Checks for class/definition/plan arguments uniqueness

Bad:

    class some::class (
      $arg,
      $arg,
      $arg,
    ) { }


<a id="org8db2bdf"></a>

## UniqueAttributeName

Resource attributes must be unique

Bad:

    service { 'sshd':
      ensure => running,
      ensure => stopped,
    }


<a id="org760446f"></a>

## UnusedVariables

Checks for unused variables. Experimental lint false-positives are possible.

Bad:

    class some::class (
      $unused_argument,
    ) {
      service { 'sshd':
        ensure => running,
      }
    }


<a id="org4375ef7"></a>

## UpperCaseName

Warns if resource set used with uppercase letters

Bad:

    Service { 'sshd':
      ensure => running,
    }

Good:

    service { 'sshd':
      ensure => running,
    }


<a id="org4eff26b"></a>

## UselessDoubleQuotes

Warns if double quoted string has no interpolated expressions and no escaped single quotes

Bad:

    $var = "simple literal"

Good:

    $var = 'simple literal'


<a id="orgae4872f"></a>

## UselessParens

Checks for extra parens

Bad:

    if (($var1) or ($var2)) { }

Good:

    if $var1 or $var2 { }


<a id="orga0ae0ef"></a>

# Linter for YAML files

Some basic checks are implemented:

-   File is not executable
-   File is empty (no root value available)
-   File parsed without syntax errors
-   Maps does not contain duplicate keys
-   Attempt to merge anchor which type is not array nor map


<a id="orgf2a84fb"></a>

# Linter for Hiera YAML files

All lints of YAML files plus:


<a id="org9ff9528"></a>

## Reference to a module which has syntax errors

Linter will fail if some<sub>class</sub> was unable to parse:

    some_class::argument: 1


<a id="org3f03ae9"></a>

## Reference to class which is not found in modules/

Linter will fail if modules/some<sub>class</sub>/init.pp does not exists:

    some_class::argument: 1


<a id="orgf2c89bb"></a>

## Reference in undefined class argument

Linter will fail if some<sub>class</sub> does not accept argument $argument<sub>name</sub>:

    some_class::argument_name: 1


<a id="orge25bd0c"></a>

## Single column in the name of key of root map

Linter protects agains typos like:

    some_class:argument_name: 1

