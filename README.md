
# Table of Contents

1.  [About](#org2c6524d)
2.  [Installation methods](#org80bc635)
    1.  [Via deb/rpm](#org9482d53)
    2.  [MaOS binaries](#org5fd5560)
    3.  [Via cargo](#orgf1a4e91)
    4.  [Guix manifest](#orgf53d9bf)
3.  [Usage](#orge6106c3)
    1.  [Correctness of YAML files](#org3c9b184)
    2.  [Validity of Hiera YAML files](#org9d03a8e)
    3.  [Linter of Puppet manifest files](#orgd2b4208)
    4.  [Pretty printing manifest file](#org100b6f2)
    5.  [Config file generator](#org5b082d9)
    6.  [Hiera explorer](#org463f849)
    7.  [\*.pp AST dumper](#orgfa548a6)
4.  [Available lints for \*.pp](#org6071675)
    1.  [ArgumentLooksSensitive](#orgb94aff2)
    2.  [ArgumentTyped](#org669c20a)
    3.  [ConstantExpressionInCondition](#org015b123)
    4.  [DefaultCaseIsNotLast](#orgdfdacf8)
    5.  [DoNotUseUnless](#orge07b8f6)
    6.  [DoubleNegation](#org793a364)
    7.  [EmptyCasesList](#org6573c84)
    8.  [EnsureAttributeIsNotTheFirst](#orgc2fe9a0)
    9.  [ErbReferencesToUnknownVariable](#orge547f8a)
    10. [ExecAttributes](#orgd0ebc4e)
    11. [ExpressionInSingleQuotes](#orgb2f711a)
    12. [FileModeAttributeIsString](#orgc8d722b)
    13. [InvalidResourceCollectionInvocation](#orgc72b7a3)
    14. [InvalidResourceSetInvocation](#orgece14af)
    15. [InvalidStringEscape](#orge531304)
    16. [InvalidVariableAssignment](#org5a427be)
    17. [LowerCaseArgumentName](#org1907aa6)
    18. [LowerCaseVariable](#org41c6751)
    19. [MultipleDefaultCase](#orga5f48d1)
    20. [MultipleResourcesWithoutDefault](#orgfde8c48)
    21. [NegationOfEquation](#orgaf05a57)
    22. [NoDefaultCase](#org93ae086)
    23. [OptionalArgumentsGoesFirst](#org86b144e)
    24. [PerExpressionResourceDefaults](#org9d35cbe)
    25. [ReadableArgumentsName](#orga600947)
    26. [ReferenceToUndefinedValue](#org93a8f42)
    27. [RelationToTheLeft](#org0fd8e18)
    28. [SelectorInAttributeValue](#orgc8da91f)
    29. [SensitiveArgumentWithDefault](#org621e2d6)
    30. [StatementWithNoEffect](#org23e8751)
    31. [UnconditionalExec](#org2f3cc0d)
    32. [UniqueArgumentsNames](#org4706b73)
    33. [UniqueAttributeName](#org2ac04df)
    34. [UnusedVariables](#org0fe9664)
    35. [UpperCaseName](#org1d46d2e)
    36. [UselessDoubleQuotes](#orgc183bd0)
    37. [UselessParens](#org6a8ead3)
    38. [MagicNumber](#orga787a14)
5.  [Linter for YAML files](#org5d26bb2)
6.  [Linter for Hiera YAML files](#orgc7b7371)
    1.  [Reference to a module which has syntax errors](#org23d0281)
    2.  [Reference to class which is not found in modules/](#org362139c)
    3.  [Reference in undefined class argument](#org2f1c1c6)
    4.  [Single column in the name of key of root map](#orgd329a10)


<a id="org2c6524d"></a>

# About

Shadowplay is a utility for checking puppet syntax, a puppet manifest linter, a pretty printer, and a utility for exploring the Hiera.

![img](./doc/screenshot-emacs.png "Flycheck plugin for Emacs")


<a id="org80bc635"></a>

# Installation methods


<a id="org9482d53"></a>

## Via deb/rpm

Latest releases can be downloaded here: <https://github.com/mailru/shadowplay/releases>


<a id="org5fd5560"></a>

## MaOS binaries

Lastest binaries for MacOS can be downloaded here: <https://github.com/mailru/shadowplay/releases>


<a id="orgf1a4e91"></a>

## Via cargo

    cargo install shadowplay


<a id="orgf53d9bf"></a>

## Guix manifest

Guix manifest is not merged into main repository yet. One can use etc/guix.scm from Shadowplay repo. All missing dependencies are also
included into manifest file.


<a id="orge6106c3"></a>

# Usage


<a id="org3c9b184"></a>

## Correctness of YAML files

    shadowplay check yaml hieradata/default.yaml [hieradata/another.yaml] ...

In addition to the correctness of the syntax, the uniqueness of the keys in maps will be checked, as well as the correctness of the links
(anchors).


<a id="org9d03a8e"></a>

## Validity of Hiera YAML files

    shadowplay check hiera hieradata/default.yaml ...

For the specified files, YAML correctness will be checked, as well as the correctness of references to Puppet classes and class arguments.
For example, there will be an error generated if an unknown class argument is used.

As a side effect, it also checks the correctness of syntax of pappet manifests referenced by values ​​in Hiera.


<a id="orgd2b4208"></a>

## Linter of Puppet manifest files

    shadowplay --repo-path ./ check pp modules/hammer/manifests/config.pp ...

The specified files will be processed by the parser, then linter checks will be applied to the resulting AST (if parsing is successful).


<a id="org100b6f2"></a>

## Pretty printing manifest file

    shadowplay pretty-print-pp < /path/to/file.pp


<a id="org5b082d9"></a>

## Config file generator

Use may want to disable some lints or customize it. She can generate default config and edit it later with the command:

    shadowplay generate-config >/etc/shadowplay.yaml


<a id="org463f849"></a>

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


<a id="orgfa548a6"></a>

## \*.pp AST dumper

    shadowplay dump modules/sshd/manifests/install.pp

Outputs AST in JSON format. Mainly for internal purposes.


<a id="org6071675"></a>

# Available lints for \*.pp


<a id="orgb94aff2"></a>

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


<a id="org669c20a"></a>

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


<a id="org015b123"></a>

## ConstantExpressionInCondition

Warns if constant expression is used in condition

Bad:

    if 1 == 2 - 1 { notify('1=2-1') }

Such type of conditions always evaluated into constant false or true, thus can be safely removed. Good:

    notify('1=2-1')


<a id="orgdfdacf8"></a>

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


<a id="orge07b8f6"></a>

## DoNotUseUnless

Warns if 'unless' conditional statement is used

Bad:

    unless $value { }

Good:

    if !$value { }


<a id="org793a364"></a>

## DoubleNegation

Warns if double negation is used

Bad:

    if !(!$value) { }
    
    if !($value != 1) { }

Good:

    if $value { }
    
    if $value == 1 { }


<a id="org6573c84"></a>

## EmptyCasesList

Warns if case { &#x2026; } has no cases

Bad:

    case $value { }


<a id="orgc2fe9a0"></a>

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


<a id="orge547f8a"></a>

## ErbReferencesToUnknownVariable

Checks ERB templates specified in template() for undefined variables

Bad:

    class some::class () {
      # here template_file.erb contains: <% @some_undefined_variable %>
      $value = template('some/template_file.erb')
    }


<a id="orgd0ebc4e"></a>

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


<a id="orgb2f711a"></a>

## ExpressionInSingleQuotes

Warns if interpolated expression found single-qouted string

Bad:

    $value = 'Hello $world'
    
    $value = '2 + 2 = ${2+2}'


<a id="orgc8d722b"></a>

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


<a id="orgc72b7a3"></a>

## InvalidResourceCollectionInvocation

Checks if existing resource set is used and all arguments are known in it's class

Bad:

    # relation to unknown resource
    Class['unknown_class'] -> Class['known_class']


<a id="orgece14af"></a>

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


<a id="orge531304"></a>

## InvalidStringEscape

Checks if only allowed characters are escaped in strings

Bad:

    $value = '\s*\.'
    
    $value = "\s*\."

Good:

    $value = '\\s*\\.'
    
    $value = "\\s*\\."


<a id="org5a427be"></a>

## InvalidVariableAssignment

Warns if left part of assignment is not a variable or array of variables

Bad:

    lookup('some::value') = 1


<a id="org1907aa6"></a>

## LowerCaseArgumentName

Warns if argument name is not lowercase, as suggested by Puppet's style guide

Bad:

    class some::class (
      $ArgumentInCamelCase
    ) {}


<a id="org41c6751"></a>

## LowerCaseVariable

Warns if variable name is not lowercase

Bad:

    class some::class () {
      $VariableIsNOTInLowercase = 1


<a id="orga5f48d1"></a>

## MultipleDefaultCase

Warns if case statement has multiple 'default' cases

Bad:

    case $val {
      1: {}
      default: {}
      default: {}
    }


<a id="orgfde8c48"></a>

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


<a id="orgaf05a57"></a>

## NegationOfEquation

Warns on negation of equation

Bad:

    if !($a == 1) { }
    
    if !($a =~ /./) { }

Good:

    if $a != 1 { }
    
    if $a !~ /./ { }


<a id="org93ae086"></a>

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


<a id="org86b144e"></a>

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


<a id="org9d35cbe"></a>

## PerExpressionResourceDefaults

Warns if local resource defaults are used

Bad:

    Exec {
      provider => shell,
    }
    
    exec { 'run command':
      command => 'echo Hello',
    }


<a id="orga600947"></a>

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


<a id="org93a8f42"></a>

## ReferenceToUndefinedValue

Warns if variable is not defined in current context

Bad:

    if $some_undefined_variable { }


<a id="org0fd8e18"></a>

## RelationToTheLeft

Checks for left-directed relations

Bad:

    Class['c'] <- Class['b'] <~ Class['a']

Good:

    Class['a'] ~> Class['b'] -> Class['c']


<a id="orgc8da91f"></a>

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


<a id="org621e2d6"></a>

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


<a id="org23e8751"></a>

## StatementWithNoEffect

Checks for statements without side effects

Bad:

    if $a {
      if $b {
        2 + 2
      }
    }


<a id="org2f3cc0d"></a>

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


<a id="org4706b73"></a>

## UniqueArgumentsNames

Checks for class/definition/plan arguments uniqueness

Bad:

    class some::class (
      $arg,
      $arg,
      $arg,
    ) { }


<a id="org2ac04df"></a>

## UniqueAttributeName

Resource attributes must be unique

Bad:

    service { 'sshd':
      ensure => running,
      ensure => stopped,
    }


<a id="org0fe9664"></a>

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


<a id="org1d46d2e"></a>

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


<a id="orgc183bd0"></a>

## UselessDoubleQuotes

Warns if double quoted string has no interpolated expressions and no escaped single quotes

Bad:

    $var = "simple literal"

Good:

    $var = 'simple literal'


<a id="org6a8ead3"></a>

## UselessParens

Checks for extra parens

Bad:

    if (($var1) or ($var2)) { }

Good:

    if $var1 or $var2 { }


<a id="orga787a14"></a>

## MagicNumber

Warns if term contains magic number.

Bad:

    if $port == 58271 { }

Good:

    $default_service_port = 58271
    
    if $port == $default_service_port { }


<a id="org5d26bb2"></a>

# Linter for YAML files

Some basic checks are implemented:

-   File is not executable
-   File is empty (no root value available)
-   File parsed without syntax errors
-   Maps does not contain duplicate keys
-   Attempt to merge anchor which type is not array nor map


<a id="orgc7b7371"></a>

# Linter for Hiera YAML files

All lints of YAML files plus:


<a id="org23d0281"></a>

## Reference to a module which has syntax errors

Linter will fail if some<sub>class</sub> was unable to parse:

    some_class::argument: 1


<a id="org362139c"></a>

## Reference to class which is not found in modules/

Linter will fail if modules/some<sub>class</sub>/init.pp does not exists:

    some_class::argument: 1


<a id="org2f1c1c6"></a>

## Reference in undefined class argument

Linter will fail if some<sub>class</sub> does not accept argument $argument<sub>name</sub>:

    some_class::argument_name: 1


<a id="orgd329a10"></a>

## Single column in the name of key of root map

Linter protects agains typos like:

    some_class:argument_name: 1

