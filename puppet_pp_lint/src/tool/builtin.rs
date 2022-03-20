pub fn is_constant<EXTRA>(f: &puppet_lang::builtin::BuiltinVariant<EXTRA>) -> bool {
    match &f {
        puppet_lang::builtin::BuiltinVariant::Undef => true,
        puppet_lang::builtin::BuiltinVariant::Return(_) => true,
        puppet_lang::builtin::BuiltinVariant::Tag(_)
        | puppet_lang::builtin::BuiltinVariant::Require(_)
        | puppet_lang::builtin::BuiltinVariant::Include(_)
        | puppet_lang::builtin::BuiltinVariant::Realize(_)
        | puppet_lang::builtin::BuiltinVariant::CreateResources(_) => {
            // TODO check lambda statement
            false
        }
    }
}
