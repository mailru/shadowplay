
# Table of Contents

1.  [About](#org69c4ee5)
2.  [Installation methods](#org0de282f)
    1.  [Via deb/rpm](#org99f1862)
    2.  [MaOS binaries](#org47f1002)
    3.  [Via cargo](#org1d0629f)
    4.  [Guix manifest](#org422efc7)
3.  [Usage](#org03891a1)
    1.  [Correctness of YAML files](#org5e1b701)
    2.  [Validity of Hiera YAML files](#orge989d6f)
    3.  [Linter of Puppet manifest files](#org533deba)
    4.  [Pretty printing manifest file](#orgbb610ea)
    5.  [Config file generator](#orgd106844)
    6.  [Hiera explorer](#org824a981)
    7.  [\*.pp AST dumper](#orgb8ef884)
4.  [Available lints for \*.pp](#orgb030ef1)
    1.  [ArgumentLooksSensitive](#org18746a9)
    2.  [ArgumentTyped](#org49fe89c)
    3.  [ConstantExpressionInCondition](#orgc0f08a1)
    4.  [DefaultCaseIsNotLast](#org26462c5)
    5.  [DoNotUseUnless](#org649528e)
    6.  [DoubleNegation](#org0b09277)
    7.  [EmptyCasesList](#org78da798)
    8.  [EnsureAttributeIsNotTheFirst](#org1dc599c)
    9.  [ErbReferencesToUnknownVariable](#org6390d4c)
    10. [ExecAttributes](#orgcd3755f)
    11. [ExpressionInSingleQuotes](#orgdea08c8)
    12. [FileModeAttributeIsString](#org1de4f41)
    13. [InvalidResourceCollectionInvocation](#org99aa6c0)
    14. [InvalidResourceSetInvocation](#org9baa7c1)
    15. [InvalidStringEscape](#org31c4451)
    16. [InvalidVariableAssignment](#org5da3c5d)
    17. [LowerCaseArgumentName](#orgc014a22)
    18. [LowerCaseVariable](#org7861a05)
    19. [MultipleDefaultCase](#orgc437d51)
    20. [MultipleResourcesWithoutDefault](#org844dda2)
    21. [NegationOfEquation](#org50aeb09)
    22. [NoDefaultCase](#org5750d1a)
    23. [OptionalArgumentsGoesFirst](#orgf25c7e3)
    24. [PerExpressionResourceDefaults](#orgff01509)
    25. [ReadableArgumentsName](#orgd55110c)
    26. [ReferenceToUndefinedValue](#org8cba3d6)
    27. [RelationToTheLeft](#orgd97bab9)
    28. [SelectorInAttributeValue](#orga58d957)
    29. [SensitiveArgumentWithDefault](#org52367d2)
    30. [StatementWithNoEffect](#orgde1e58a)
    31. [UnconditionalExec](#orgf46ee37)
    32. [UniqueArgumentsNames](#orgaaa65b7)
    33. [UniqueAttributeName](#org83855a4)
    34. [UnusedVariables](#org6c6fa54)
    35. [UpperCaseName](#org687dd29)
    36. [UselessDoubleQuotes](#orgb5a08c4)
    37. [UselessParens](#orgd85a9a5)
5.  [Linter for YAML files](#orgba846f3)
6.  [Linter for Hiera YAML files](#org53c14e2)
    1.  [Reference to a module which has syntax errors](#org5e31778)
    2.  [Reference to class which is not found in modules/](#orgc745c51)
    3.  [Reference in undefined class argument](#orgc38e4a2)
    4.  [Single column in the name of key of root map](#org22bf944)


<a id="org69c4ee5"></a>

# About

Shadowplay is a utility for checking puppet syntax, a puppet manifest linter, a pretty printer, and a utility for exploring the Hiera.

![img](./doc/screenshot-emacs.png "Flycheck plugin for Emacs")


<a id="org0de282f"></a>

# Installation methods


<a id="org99f1862"></a>

## Via deb/rpm

Latest releases can be downloaded here: <https://github.com/mailru/shadowplay/releases>


<a id="org47f1002"></a>

## MaOS binaries

Lastest binaries for MacOS can be downloaded here: <https://github.com/mailru/shadowplay/releases>


<a id="org1d0629f"></a>

## Via cargo

    cargo install shadowplay


<a id="org422efc7"></a>

## Guix manifest

Guix manifest is not merged into main repository yet. One can use etc/guix.scm from Shadowplay repo. All missing dependencies are also
included into manifest file.


<a id="org03891a1"></a>

# Usage


<a id="org5e1b701"></a>

## Correctness of YAML files

    shadowplay check yaml hieradata/default.yaml [hieradata/another.yaml] ...

In addition to the correctness of the syntax, the uniqueness of the keys in maps will be checked, as well as the correctness of the links
(anchors).


<a id="orge989d6f"></a>

## Validity of Hiera YAML files

    shadowplay check hiera hieradata/default.yaml ...

For the specified files, YAML correctness will be checked, as well as the correctness of references to Puppet classes and class arguments.
For example, there will be an error generated if an unknown class argument is used.

As a side effect, it also checks the correctness of syntax of pappet manifests referenced by values ​​in Hiera.


<a id="org533deba"></a>

## Linter of Puppet manifest files

    shadowplay --repo-path ./ check pp modules/hammer/manifests/config.pp ...

The specified files will be processed by the parser, then linter checks will be applied to the resulting AST (if parsing is successful).


<a id="orgbb610ea"></a>

## Pretty printing manifest file

    shadowplay pretty-print-pp < /path/to/file.pp


<a id="orgd106844"></a>

## Config file generator

Use may want to disable some lints or customize it. She can generate default config and edit it later with the command:

    shadowplay generate-config >/etc/shadowplay.yaml


<a id="org824a981"></a>

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


<a id="orgb8ef884"></a>

## \*.pp AST dumper

    shadowplay dump modules/sshd/manifests/install.pp

Outputs AST in JSON format. Mainly for internal purposes.


<a id="orgb030ef1"></a>

# Available lints for \*.pp


<a id="org18746a9"></a>

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


<a id="org49fe89c"></a>

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


<a id="orgc0f08a1"></a>

## ConstantExpressionInCondition

Warns if constant expression is used in condition

Bad:

    if 1 == 2 - 1 { notify('1=2-1') }

Such type of conditions always evaluated into constant false or true, thus can be safely removed. Good:

    notify('1=2-1')


<a id="org26462c5"></a>

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


<a id="org649528e"></a>

## DoNotUseUnless

Warns if 'unless' conditional statement is used

Bad:

    unless $value { }

Good:

    if !$value { }


<a id="org0b09277"></a>

## DoubleNegation

Warns if double negation is used

Bad:

    if !(!$value) { }
    
    if !($value != 1) { }

Good:

    if $value { }
    
    if $value == 1 { }


<a id="org78da798"></a>

## EmptyCasesList

Warns if case { &#x2026; } has no cases

Bad:

    case $value { }


<a id="org1dc599c"></a>

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


<a id="org6390d4c"></a>

## ErbReferencesToUnknownVariable

Checks ERB templates specified in template() for undefined variables

Bad:

    class some::class () {
      # here template_file.erb contains: <% @some_undefined_variable %>
      $value = template('some/template_file.erb')
    }


<a id="orgcd3755f"></a>

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


<a id="orgdea08c8"></a>

## ExpressionInSingleQuotes

Warns if interpolated expression found single-qouted string

Bad:

    $value = 'Hello $world'
    
    $value = '2 + 2 = ${2+2}'


<a id="org1de4f41"></a>

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


<a id="org99aa6c0"></a>

## InvalidResourceCollectionInvocation

Checks if existing resource set is used and all arguments are known in it's class

Bad:

    # relation to unknown resource
    Class['unknown_class'] -> Class['known_class']


<a id="org9baa7c1"></a>

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


<a id="org31c4451"></a>

## InvalidStringEscape

Checks if only allowed characters are escaped in strings

Bad:

    $value = '\s*\.'
    
    $value = "\s*\."

Good:

    $value = '\\s*\\.'
    
    $value = "\\s*\\."


<a id="org5da3c5d"></a>

## InvalidVariableAssignment

Warns if left part of assignment is not a variable or array of variables

Bad:

    lookup('some::value') = 1


<a id="orgc014a22"></a>

## LowerCaseArgumentName

Warns if argument name is not lowercase, as suggested by Puppet's style guide

Bad:

    class some::class (
      $ArgumentInCamelCase
    ) {}


<a id="org7861a05"></a>

## LowerCaseVariable

Warns if variable name is not lowercase

Bad:

    class some::class () {
      $VariableIsNOTInLowercase = 1


<a id="orgc437d51"></a>

## MultipleDefaultCase

Warns if case statement has multiple 'default' cases

Bad:

    case $val {
      1: {}
      default: {}
      default: {}
    }


<a id="org844dda2"></a>

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


<a id="org50aeb09"></a>

## NegationOfEquation

Warns on negation of equation

Bad:

    if !($a == 1) { }
    
    if !($a =~ /./) { }

Good:

    if $a != 1 { }
    
    if $a !~ /./ { }


<a id="org5750d1a"></a>

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


<a id="orgf25c7e3"></a>

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


<a id="orgff01509"></a>

## PerExpressionResourceDefaults

Warns if local resource defaults are used

Bad:

    Exec {
      provider => shell,
    }
    
    exec { 'run command':
      command => 'echo Hello',
    }


<a id="orgd55110c"></a>

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


<a id="org8cba3d6"></a>

## ReferenceToUndefinedValue

Warns if variable is not defined in current context

Bad:

    if $some_undefined_variable { }


<a id="orgd97bab9"></a>

## RelationToTheLeft

Checks for left-directed relations

Bad:

    Class['c'] <- Class['b'] <~ Class['a']

Good:

    Class['a'] ~> Class['b'] -> Class['c']


<a id="orga58d957"></a>

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


<a id="org52367d2"></a>

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


<a id="orgde1e58a"></a>

## StatementWithNoEffect

Checks for statements without side effects

Bad:

    if $a {
      if $b {
        2 + 2
      }
    }


<a id="orgf46ee37"></a>

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


<a id="orgaaa65b7"></a>

## UniqueArgumentsNames

Checks for class/definition/plan arguments uniqueness

Bad:

    class some::class (
      $arg,
      $arg,
      $arg,
    ) { }


<a id="org83855a4"></a>

## UniqueAttributeName

Resource attributes must be unique

Bad:

    service { 'sshd':
      ensure => running,
      ensure => stopped,
    }


<a id="org6c6fa54"></a>

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


<a id="org687dd29"></a>

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


<a id="orgb5a08c4"></a>

## UselessDoubleQuotes

Warns if double quoted string has no interpolated expressions and no escaped single quotes

Bad:

    $var = "simple literal"

Good:

    $var = 'simple literal'


<a id="orgd85a9a5"></a>

## UselessParens

Checks for extra parens

Bad:

    if (($var1) or ($var2)) { }

Good:

    if $var1 or $var2 { }


<a id="orgba846f3"></a>

# Linter for YAML files

Some basic checks are implemented:

-   File is not executable
-   File is empty (no root value available)
-   File parsed without syntax errors
-   Maps does not contain duplicate keys
-   Attempt to merge anchor which type is not array nor map


<a id="org53c14e2"></a>

# Linter for Hiera YAML files

All lints of YAML files plus:


<a id="org5e31778"></a>

## Reference to a module which has syntax errors

Linter will fail if some<sub>class</sub> was unable to parse:

    some_class::argument: 1


<a id="orgc745c51"></a>

## Reference to class which is not found in modules/

Linter will fail if modules/some<sub>class</sub>/init.pp does not exists:

    some_class::argument: 1


<a id="orgc38e4a2"></a>

## Reference in undefined class argument

Linter will fail if some<sub>class</sub> does not accept argument $argument<sub>name</sub>:

    some_class::argument_name: 1


<a id="org22bf944"></a>

## Single column in the name of key of root map

Linter protects agains typos like:

    some_class:argument_name: 1

