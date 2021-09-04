Summary: Hiera explorer
Name: hixplorer
Version: %(cat VERSION)
Release: 1%{dist}
License: MailRu Private
Group: Development/Tools
Source0: %{name}-%{version}.tar.gz
BuildRoot: %{_tmppath}/%{name}-%{version}-%{release}-root

AutoReqProv: no

BuildRequires: gcc
BuildRequires: autoconf
BuildRequires: automake
BuildRequires: libtool
BuildRequires: openssl-devel
BuildRequires: llvm-devel
BuildRequires: clang

%description
%{summary}

Tool that extracts data from heira, taking into account the hierarchy specified in the configuration file.

Built by: %__hammer_user_name__ (%__hammer_user_login__)
From git commit: %__hammer_git_hash__ (%__hammer_git_ref__)

Build details: %__hammer_build_url__

%prep

%build
if [ -e VERSION ]; then
   sed -i -e "s/^package[.]version = .*/package.version = \"$(cat VERSION)\"/" */Cargo.toml
fi
cargo build --release

%install
rm -rf %{buildroot}
%{__mkdir} -p %{buildroot}%{_bindir}

 {__install} -pD -m 755 target/release/hixplorer %{buildroot}%{_bindir}/hixplorer

%clean
rm -rf %{buildroot}

%files
%defattr(-,root,root,-)
%{_bindir}/hixplorer
