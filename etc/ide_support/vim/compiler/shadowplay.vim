" Vim compiler file
" Compiler:	shadowplay
" Maintainer: Evgenii Lepikhin <e.lepikhin@corp.mail.ru>

if exists('current_compiler')
  finish
endif
let current_compiler = 'shadowplay'

if exists(':CompilerSet') != 2		" older Vim always used :setlocal
  command -nargs=* CompilerSet setlocal <args>
endif

let s:cpo_save = &cpo
set cpo&vim

CompilerSet makeprg=shadowplay\ check\ pp
CompilerSet errorformat=Puppet\ manifest\ lint\ error\ in\ "%f"\ at\ line\ %l\ column\ %c:\ %m,
                        \Puppet\ manifest\ syntax\ error\ in\ "%f"\ at\ line\ %l\ column\ %c:\ %m

let &cpo = s:cpo_save
unlet s:cpo_save