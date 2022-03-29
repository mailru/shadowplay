pub fn is_constant<EXTRA>(f: &crate::puppet_lang::builtin::BuiltinVariant<EXTRA>) -> bool {
    match &f {
        crate::puppet_lang::builtin::BuiltinVariant::Undef => true,
        crate::puppet_lang::builtin::BuiltinVariant::Return(_) => true,
        crate::puppet_lang::builtin::BuiltinVariant::Tag(_)
        | crate::puppet_lang::builtin::BuiltinVariant::Require(_)
        | crate::puppet_lang::builtin::BuiltinVariant::Include(_)
        | crate::puppet_lang::builtin::BuiltinVariant::Realize(_)
        | crate::puppet_lang::builtin::BuiltinVariant::Template(_)
        | crate::puppet_lang::builtin::BuiltinVariant::CreateResources(_) => {
            // TODO check lambda statement
            false
        }
    }
}
