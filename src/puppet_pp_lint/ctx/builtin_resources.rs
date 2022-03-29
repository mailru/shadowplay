use std::collections::HashMap;

#[derive(Default)]
pub struct Attribute {}

#[derive(Default)]
pub struct Resource {
    pub attributes: HashMap<&'static str, Attribute>,
}

pub fn generate() -> HashMap<&'static str, Resource> {
    let mut r = HashMap::new();

    let mut file = Resource::default();
    let _ = file.attributes.insert("path", Attribute::default());
    let _ = file.attributes.insert("ensure", Attribute::default());
    let _ = file.attributes.insert("backup", Attribute::default());
    let _ = file.attributes.insert("checksum", Attribute::default());
    let _ = file
        .attributes
        .insert("checksum_value", Attribute::default());
    let _ = file.attributes.insert("content", Attribute::default());
    let _ = file.attributes.insert("ctime", Attribute::default());
    let _ = file.attributes.insert("force", Attribute::default());
    let _ = file.attributes.insert("group", Attribute::default());
    let _ = file.attributes.insert("ignore", Attribute::default());
    let _ = file.attributes.insert("links", Attribute::default());
    let _ = file.attributes.insert("max_files", Attribute::default());
    let _ = file.attributes.insert("mode", Attribute::default());
    let _ = file.attributes.insert("mtime", Attribute::default());
    let _ = file.attributes.insert("owner", Attribute::default());
    let _ = file.attributes.insert("provider", Attribute::default());
    let _ = file.attributes.insert("purge", Attribute::default());
    let _ = file.attributes.insert("recurse", Attribute::default());
    let _ = file.attributes.insert("recurselimit", Attribute::default());
    let _ = file.attributes.insert("replace", Attribute::default());
    let _ = file
        .attributes
        .insert("selinux_ignore_defaults", Attribute::default());
    let _ = file.attributes.insert("selrange", Attribute::default());
    let _ = file.attributes.insert("selrole", Attribute::default());
    let _ = file.attributes.insert("seltype", Attribute::default());
    let _ = file.attributes.insert("seluser", Attribute::default());
    let _ = file.attributes.insert("show_diff", Attribute::default());
    let _ = file.attributes.insert("source", Attribute::default());
    let _ = file
        .attributes
        .insert("source_permissions", Attribute::default());
    let _ = file.attributes.insert("sourceselect", Attribute::default());
    let _ = file
        .attributes
        .insert("staging_location", Attribute::default());
    let _ = file.attributes.insert("target", Attribute::default());
    let _ = file.attributes.insert("type", Attribute::default());
    let _ = file.attributes.insert("validate_cmd", Attribute::default());
    let _ = file
        .attributes
        .insert("validate_replacement", Attribute::default());
    let _ = r.insert("file", file);

    let mut exec = Resource::default();
    let _ = exec.attributes.insert("command", Attribute::default());
    let _ = exec.attributes.insert("creates", Attribute::default());
    let _ = exec.attributes.insert("cwd", Attribute::default());
    let _ = exec.attributes.insert("environment", Attribute::default());
    let _ = exec.attributes.insert("group", Attribute::default());
    let _ = exec.attributes.insert("logoutput", Attribute::default());
    let _ = exec.attributes.insert("onlyif", Attribute::default());
    let _ = exec.attributes.insert("path", Attribute::default());
    let _ = exec.attributes.insert("provider", Attribute::default());
    let _ = exec.attributes.insert("refresh", Attribute::default());
    let _ = exec.attributes.insert("refreshonly", Attribute::default());
    let _ = exec.attributes.insert("returns", Attribute::default());
    let _ = exec.attributes.insert("timeout", Attribute::default());
    let _ = exec.attributes.insert("tries", Attribute::default());
    let _ = exec.attributes.insert("try_sleep", Attribute::default());
    let _ = exec.attributes.insert("umask", Attribute::default());
    let _ = exec.attributes.insert("unless", Attribute::default());
    let _ = exec.attributes.insert("user", Attribute::default());
    let _ = r.insert("exec", exec);

    let mut filebucket = Resource::default();
    let _ = filebucket.attributes.insert("name", Attribute::default());
    let _ = filebucket.attributes.insert("path", Attribute::default());
    let _ = filebucket.attributes.insert("port", Attribute::default());
    let _ = filebucket.attributes.insert("server", Attribute::default());
    let _ = r.insert("filebucket", filebucket);

    let mut group = Resource::default();
    let _ = group.attributes.insert("name", Attribute::default());
    let _ = group.attributes.insert("ensure", Attribute::default());
    let _ = group.attributes.insert("allowdupe", Attribute::default());
    let _ = group
        .attributes
        .insert("attribute_membership", Attribute::default());
    let _ = group.attributes.insert("attributes", Attribute::default());
    let _ = group
        .attributes
        .insert("auth_membership", Attribute::default());
    let _ = group.attributes.insert("forcelocal", Attribute::default());
    let _ = group.attributes.insert("gid", Attribute::default());
    let _ = group
        .attributes
        .insert("ia_load_module", Attribute::default());
    let _ = group.attributes.insert("members", Attribute::default());
    let _ = group.attributes.insert("provider", Attribute::default());
    let _ = group.attributes.insert("system", Attribute::default());
    let _ = r.insert("group", group);

    let mut notify = Resource::default();
    let _ = notify.attributes.insert("name", Attribute::default());
    let _ = notify.attributes.insert("message", Attribute::default());
    let _ = notify.attributes.insert("withpath", Attribute::default());
    let _ = r.insert("notify", notify);

    let mut package = Resource::default();
    let _ = package.attributes.insert("name", Attribute::default());
    let _ = package.attributes.insert("command", Attribute::default());
    let _ = package.attributes.insert("ensure", Attribute::default());
    let _ = package.attributes.insert("adminfile", Attribute::default());
    let _ = package
        .attributes
        .insert("allow_virtual", Attribute::default());
    let _ = package
        .attributes
        .insert("allowcdrom", Attribute::default());
    let _ = package.attributes.insert("category", Attribute::default());
    let _ = package
        .attributes
        .insert("configfiles", Attribute::default());
    let _ = package
        .attributes
        .insert("description", Attribute::default());
    let _ = package
        .attributes
        .insert("enable_only", Attribute::default());
    let _ = package.attributes.insert("flavor", Attribute::default());
    let _ = package
        .attributes
        .insert("install_only", Attribute::default());
    let _ = package
        .attributes
        .insert("install_options", Attribute::default());
    let _ = package.attributes.insert("instance", Attribute::default());
    let _ = package.attributes.insert("mark", Attribute::default());
    let _ = package
        .attributes
        .insert("portage_settings", Attribute::default());
    let _ = package.attributes.insert("platform", Attribute::default());
    let _ = package.attributes.insert("provider", Attribute::default());
    let _ = package
        .attributes
        .insert("reinstall_on_refresh", Attribute::default());
    let _ = package
        .attributes
        .insert("responsefile", Attribute::default());
    let _ = package.attributes.insert("root", Attribute::default());
    let _ = package.attributes.insert("source", Attribute::default());
    let _ = package.attributes.insert("status", Attribute::default());
    let _ = package
        .attributes
        .insert("uninstall_options", Attribute::default());
    let _ = package.attributes.insert("vendor", Attribute::default());
    let _ = r.insert("package", package);

    let mut resources = Resource::default();
    let _ = resources.attributes.insert("name", Attribute::default());
    let _ = resources.attributes.insert("purge", Attribute::default());
    let _ = resources
        .attributes
        .insert("unless_system_user", Attribute::default());
    let _ = resources
        .attributes
        .insert("unless_uid", Attribute::default());
    let _ = r.insert("resources", resources);

    let mut schedule = Resource::default();
    let _ = schedule.attributes.insert("name", Attribute::default());
    let _ = schedule.attributes.insert("period", Attribute::default());
    let _ = schedule
        .attributes
        .insert("periodmatch", Attribute::default());
    let _ = schedule.attributes.insert("range", Attribute::default());
    let _ = schedule.attributes.insert("repeat", Attribute::default());
    let _ = schedule.attributes.insert("weekday", Attribute::default());
    let _ = r.insert("schedule", schedule);

    let mut service = Resource::default();
    let _ = service.attributes.insert("name", Attribute::default());
    let _ = service.attributes.insert("ensure", Attribute::default());
    let _ = service.attributes.insert("binary", Attribute::default());
    let _ = service.attributes.insert("control", Attribute::default());
    let _ = service.attributes.insert("enable", Attribute::default());
    let _ = service.attributes.insert("flags", Attribute::default());
    let _ = service
        .attributes
        .insert("hasrestart", Attribute::default());
    let _ = service.attributes.insert("hasstatus", Attribute::default());
    let _ = service
        .attributes
        .insert("logonaccount", Attribute::default());
    let _ = service
        .attributes
        .insert("logonpassword", Attribute::default());
    let _ = service.attributes.insert("manifest", Attribute::default());
    let _ = service.attributes.insert("path", Attribute::default());
    let _ = service.attributes.insert("pattern", Attribute::default());
    let _ = service.attributes.insert("provider", Attribute::default());
    let _ = service.attributes.insert("restart", Attribute::default());
    let _ = service.attributes.insert("start", Attribute::default());
    let _ = service.attributes.insert("status", Attribute::default());
    let _ = service.attributes.insert("stop", Attribute::default());
    let _ = service.attributes.insert("timeout", Attribute::default());
    let _ = r.insert("service", service);

    let mut stage = Resource::default();
    let _ = stage.attributes.insert("name", Attribute::default());
    let _ = r.insert("stage", stage);

    let mut tidy = Resource::default();
    let _ = tidy.attributes.insert("path", Attribute::default());
    let _ = tidy.attributes.insert("age", Attribute::default());
    let _ = tidy.attributes.insert("backup", Attribute::default());
    let _ = tidy.attributes.insert("matches", Attribute::default());
    let _ = tidy.attributes.insert("max_files", Attribute::default());
    let _ = tidy.attributes.insert("recurse", Attribute::default());
    let _ = tidy.attributes.insert("rmdirs", Attribute::default());
    let _ = tidy.attributes.insert("size", Attribute::default());
    let _ = tidy.attributes.insert("type", Attribute::default());
    let _ = r.insert("tidy", tidy);

    let mut user = Resource::default();
    let _ = user.attributes.insert("name", Attribute::default());
    let _ = user.attributes.insert("ensure", Attribute::default());
    let _ = user.attributes.insert("allowdupe", Attribute::default());
    let _ = user
        .attributes
        .insert("attribute_membership", Attribute::default());
    let _ = user.attributes.insert("attributes", Attribute::default());
    let _ = user
        .attributes
        .insert("auth_membership", Attribute::default());
    let _ = user.attributes.insert("auths", Attribute::default());
    let _ = user.attributes.insert("comment", Attribute::default());
    let _ = user.attributes.insert("expiry", Attribute::default());
    let _ = user.attributes.insert("forcelocal", Attribute::default());
    let _ = user.attributes.insert("gid", Attribute::default());
    let _ = user.attributes.insert("groups", Attribute::default());
    let _ = user.attributes.insert("home", Attribute::default());
    let _ = user
        .attributes
        .insert("ia_autoload_module", Attribute::default());
    let _ = user.attributes.insert("iterations", Attribute::default());
    let _ = user
        .attributes
        .insert("key_membership", Attribute::default());
    let _ = user.attributes.insert("keys", Attribute::default());
    let _ = user.attributes.insert("loginclass", Attribute::default());
    let _ = user.attributes.insert("managehome", Attribute::default());
    let _ = user.attributes.insert("membership", Attribute::default());
    let _ = user.attributes.insert("password", Attribute::default());
    let _ = user
        .attributes
        .insert("password_max_age", Attribute::default());
    let _ = user
        .attributes
        .insert("password_min_age", Attribute::default());
    let _ = user
        .attributes
        .insert("password_warn_days", Attribute::default());
    let _ = user
        .attributes
        .insert("profile_membership", Attribute::default());
    let _ = user.attributes.insert("profiles", Attribute::default());
    let _ = user.attributes.insert("project", Attribute::default());
    let _ = user.attributes.insert("provider", Attribute::default());
    let _ = user
        .attributes
        .insert("purge_ssh_keys", Attribute::default());
    let _ = user
        .attributes
        .insert("role_membership", Attribute::default());
    let _ = user.attributes.insert("roles", Attribute::default());
    let _ = user.attributes.insert("salt", Attribute::default());
    let _ = user.attributes.insert("shell", Attribute::default());
    let _ = user.attributes.insert("system", Attribute::default());
    let _ = user.attributes.insert("uid", Attribute::default());
    let _ = r.insert("user", user);

    r
}
